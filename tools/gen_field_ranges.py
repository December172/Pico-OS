#!/usr/bin/env python3


"""Generate per-field bit-range constants for the active SVD device.

The active device is selected with the SVD_DEVICE environment variable (default
RP2040); constants are written to src/Native/Constants/<DEVICE>/<PERIPHERAL>.rs.
Set SVD_EMIT_BASES_OFFSETS=0 when tools/gen_register_offsets.py owns the base
and register-offset declarations (the tools/gen_constants.py driver does this).

For every register field this script writes two constants into the matching
src/Native/Constants/<DEVICE>/<PERIPHERAL>.rs file:

    pub const <PERIPHERAL>_<REGISTER>_<FIELD>_LOW:  u32 = <low bit>;
    pub const <PERIPHERAL>_<REGISTER>_<FIELD>_HIGH: u32 = <high bit>;

For single-bit fields (high == low) one constant is emitted instead:

    pub const <PERIPHERAL>_<REGISTER>_<FIELD>_BIT:  u32 = <bit>;

Duplication handling
--------------------
The SVD repeats identical register layouts for hundreds of instances
(GPIO0..GPIO29, CH0..CH11, INSTR_MEM0..31, EP0..EP15, SM0..SM3, ...).  Rather
than emitting one constant set per instance this script collapses every group
of layout-identical registers into a single family and emits the bit ranges
once.  Numeric instance suffixes are removed from the shared name:

    PIO_INSTR_MEM0_LOW ... PIO_INSTR_MEM31_LOW  ->  PIO_INSTR_MEM_LOW

Families whose members share a common leading/trailing name token are merged
too (CLK_USB_CTRL/CLK_ADC_CTRL/CLK_RTC_CTRL -> CLK_CTRL); groups without one
keep each instance separate.  When a field name repeats its register name the
redundant component is dropped (PLL_SYS.FBDIV_INT.FBDIV_INT ->
PLL_SYS_FBDIV_INT_LOW).

Fields that cover the whole 32-bit register (low == 0 and high == 31) are
skipped entirely -- their range carries no information the register width does
not already imply.

Peripherals that share an identical register/field layout can be merged into a
single file (see svd_common.MERGED).  The merged file carries one <NAME>_BASE
constant per instance, the shared <PERIPHERAL>_<REGISTER>_OFFSET constants
(mirroring PLL.rs) and the shared field bit-range constants.  Sibling instances
are discovered via the SVD `derivedFrom` attribute (e.g. PIO1 derives from PIO0).

The generated block is delimited so it can be regenerated idempotently.
"""

import os
import sys

from svd_common import (
    CONST_DIR,
    FIELD_BEGIN as BEGIN,
    FIELD_END as END,
    MERGED,
    REG_BEGIN,
    SVD_NAME,
    collect_register_fields,
    collect_register_offsets,
    existing_constants,
    family_field_tokens,
    parse_svd,
    register_families,
    resolve,
    sanitize,
    split_block,
    upsert_block,
)


def fmt_base(value):
    """Format a peripheral base address as 0xAAAA_BBBB."""
    return f"0x{value >> 16:04X}_{value & 0xFFFF:04X}"


class Emitter:
    """Emit uniquely named `pub const` lines, skipping identical duplicates."""

    def __init__(self, reserved):
        self.used = set(reserved)
        self.values = {}

    def add(self, lines, name, value):
        rendered = str(value)
        if name in self.used:
            if self.values.get(name) == rendered:
                return  # exact duplicate already emitted elsewhere
            suffix = ""
            n = 1
            while name + suffix in self.used:
                suffix = f"_{n}"
                n += 1
            name += suffix
        self.used.add(name)
        self.values[name] = rendered
        lines.append(f"pub const {name}:".ljust(52) + f"u32 = {rendered};")


def build_block(peripheral_name, registers, reserved, bases=None, offsets=None):
    """Return the generated lines for one peripheral.

    `bases` lists the per-instance <NAME>_BASE constants for merged peripherals,
    `offsets` the shared <PERIPHERAL>_<REG>_OFFSET constants (mirroring PLL.rs);
    both are emitted ahead of the shared field bit-range lines.
    """
    lines = []
    emit = Emitter(reserved)

    for base_name, addr in (bases or []):
        emit.add(lines, base_name + "_BASE", fmt_base(addr))
    if bases:
        lines.append("")

    for reg_name, off in (offsets or []):
        emit.add(lines, f"{peripheral_name}_{sanitize(reg_name)}_OFFSET", f"0x{off:X}")
    if offsets:
        lines.append("")

    for family in register_families(registers):
        family_lines = []
        for field_token, high, low in family_field_tokens(family):
            # A field covering the whole 32-bit register adds nothing useful:
            # every bit is addressable anyway, so drop the redundant range.
            if low == 0 and high == 31:
                continue
            stem = family.name if field_token == family.name else f"{family.name}_{field_token}"
            base = f"{peripheral_name}_{stem}"
            if low == high:
                emit.add(family_lines, base + "_BIT", low)
            else:
                emit.add(family_lines, base + "_LOW", low)
                emit.add(family_lines, base + "_HIGH", high)
        if family_lines:
            lines.append(f"// {family.label}")
            lines.extend(family_lines)
            lines.append("")

    while lines and lines[-1] == "":
        lines.pop()
    return lines


def build_outputs(periphs):
    """Return [(file_stem, primary_name, [alias_names])] with the MERGED map applied."""
    merged_primary = {}
    for stem, primary in MERGED.items():
        aliases = [n for n, p in periphs.items()
                   if n != primary and p.get("derivedFrom") == primary]
        merged_primary[primary] = (stem, primary, aliases)
    handled = set()
    outputs = []
    for name in periphs:
        if name in merged_primary:
            stem, primary, aliases = merged_primary[name]
            outputs.append((stem, primary, aliases))
            handled.add(primary)
            handled.update(aliases)
    for name in periphs:
        if name not in handled:
            outputs.append((name, name, []))
    return outputs


def main():
    periphs = parse_svd()
    # tools/gen_register_offsets.py owns the base/offset constants whenever it
    # has run (its REG block is present); the SVD_EMIT_BASES_OFFSETS override
    # remains for legacy layouts.
    override = os.environ.get("SVD_EMIT_BASES_OFFSETS")

    total = 0
    for stem, primary, aliases in build_outputs(periphs):
        path = os.path.join(CONST_DIR, stem + ".rs")
        exists = os.path.isfile(path)
        if not exists and not aliases:
            print(f"  ! no constants file for peripheral {primary} (skipped)", file=sys.stderr)
            continue

        peripheral = resolve(periphs, primary)
        registers = collect_register_fields(peripheral)

        if aliases:
            content = open(path).read() if exists else f"#![allow(dead_code)]\n// {stem}\n"
        else:
            with open(path, "r") as fh:
                content = fh.read()

        if override is None:
            emit_offsets = REG_BEGIN not in content
        else:
            emit_offsets = override != "0"

        if aliases and emit_offsets:
            offsets = collect_register_offsets(peripheral)
            bases = [(primary, int(periphs[primary].findtext("baseAddress"), 0))]
            bases += [(a, int(periphs[a].findtext("baseAddress"), 0)) for a in aliases]
        else:
            offsets = bases = None

        reserved = existing_constants(split_block(content, BEGIN, END)[0])
        body = build_block(stem, registers, reserved, bases, offsets)

        lines = [f"// Generated from specs/{SVD_NAME} -- do not edit by hand.", ""] + body
        content = upsert_block(content, BEGIN, END, lines)
        with open(path, "w") as fh:
            fh.write(content)

        n_consts = len([l for l in body if l.startswith("pub const")])
        total += n_consts
        print(f"  {stem:24} registers={len(registers):4}  -> {n_consts} constants")

    print(f"\nTotal generated constants: {total}")


if __name__ == "__main__":
    main()
