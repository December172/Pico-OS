#!/usr/bin/env python3


"""Generate per-field bit-range constants for RP2040 peripherals from specs/RP2040.svd.

For every register field in the CMSIS-SVD file this script writes two constants
into the matching src/Native/Constants/RP2040/<PERIPHERAL>.rs file:

    pub const <PERIPHERAL>_<REGISTER>_<FIELD>_LOW:  u32 = <low bit>;
    pub const <PERIPHERAL>_<REGISTER>_<FIELD>_HIGH: u32 = <high bit>;

For single-bit fields (high == low) one constant is emitted instead:

    pub const <PERIPHERAL>_<REGISTER>_<FIELD>_BIT:  u32 = <bit>;

Naming:
  * Names are always fully qualified with the register name, e.g.
        PLL_SYS_CS_REFDIV_LOW / PLL_SYS_CS_REFDIV_HIGH
  * When a field repeats its own register name the redundant component is
    dropped, e.g. PLL_SYS.FBDIV_INT.FBDIV_INT -> PLL_SYS_FBDIV_INT_LOW.
  * All identifiers are uppercase snake_case following Rust naming conventions;
    any non-alphanumeric source character is folded to '_'.

Peripherals that share an identical register/field layout can be merged into a
single file (see MERGED).  The merged file carries one <NAME>_BASE constant per
instance, the shared <PERIPHERAL>_<REGISTER>_OFFSET constants (mirroring PLL.rs)
and the shared field bit-range constants.  Sibling instances are discovered via
the SVD `derivedFrom` attribute (e.g. PIO1 derives from PIO0).

The generated block is delimited so it can be regenerated idempotently.
"""

import os
import re
import sys
import xml.etree.ElementTree as ET

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SVD = os.path.join(ROOT, "specs", "RP2040.svd")
CONST_DIR = os.path.join(ROOT, "src", "Native", "Constants", "RP2040")

BEGIN = "// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ===="
END = "// ==== END AUTO-GENERATED FIELD BIT RANGES ===="

# Peripherals sharing an identical register/field layout emitted into one file.
# Maps output file stem -> primary peripheral; sibling instances are discovered
# via `derivedFrom` (e.g. PIO1 derives from PIO0).
MERGED = {
    "PIO": "PIO0",
    "I2C": "I2C0",
    "UART": "UART0",
    "SPI": "SPI0",
}

IDENT = re.compile(r"[^0-9A-Za-z_]")
BITRANGE = re.compile(r"\[(\d+):(\d+)\]")


def sanitize(name: str) -> str:
    name = IDENT.sub("_", name).upper()
    if name and name[0].isdigit():
        name = "_" + name
    return name


def resolve(periphs, name, seen=None):
    """Return the peripheral element to read registers/fields from, following derivedFrom."""
    seen = seen or set()
    if name in seen:
        raise RuntimeError("cyclic derivedFrom involving " + name)
    seen.add(name)
    p = periphs[name]
    regs = p.findall("./registers/register")
    derived = p.get("derivedFrom")
    if derived and not regs:
        return resolve(periphs, derived, seen)
    return p


def collect_fields(peripheral):
    """Yield (register_name, field_name, high, low) in document order."""
    out = []
    for reg in peripheral.findall("./registers/register"):
        reg_name = reg.findtext("name")
        fields = reg.findall("./fields/field")
        for f in fields:
            fld_name = f.findtext("name")
            br = f.findtext("bitRange")
            if br is None:
                lo = f.findtext("bitOffset")
                w = f.findtext("bitWidth")
                high = int(lo) + int(w) - 1
                low = int(lo)
            else:
                m = BITRANGE.fullmatch(br.strip())
                if not m:
                    raise RuntimeError(f"unparsable bitRange {br!r} in {reg_name}.{fld_name}")
                high, low = int(m.group(1)), int(m.group(2))
            out.append((reg_name, fld_name, high, low))
    return out


def collect_registers(peripheral):
    """Yield (register_name, address_offset) in document order."""
    out = []
    for reg in peripheral.findall("./registers/register"):
        off = reg.findtext("addressOffset")
        out.append((reg.findtext("name"), int(off, 0) if off is not None else 0))
    return out


def fmt_base(value):
    """Format a peripheral base address as 0xAAAA_BBBB."""
    return f"0x{value >> 16:04X}_{value & 0xFFFF:04X}"


def existing_constants(text):
    """Names of pub const identifiers already present in the file (outside generated block)."""
    head = text.split(BEGIN, 1)[0]
    return set(re.findall(r"pub const\s+([0-9A-Za-z_]+)\s*:", head))


def build_block(peripheral_name, fields, reserved, bases=None, registers=None):
    """Return the generated lines for one peripheral and report name conflicts.

    Every constant is fully qualified as <PERIPHERAL>_<REGISTER>_<FIELD>_LOW/_HIGH.
    When the field name repeats its register name the redundant component is
    dropped (PLL_SYS.FBDIV_INT.FBDIV_INT -> PLL_SYS_FBDIV_INT_LOW/_HIGH).

    For merged peripherals `bases` lists the per-instance <NAME>_BASE constants
    and `registers` the shared <PERIPHERAL>_<REG>_OFFSET constants; both are
    emitted ahead of the shared field bit-range lines (mirroring PLL.rs).
    """
    used = set(reserved)
    lines = []
    warnings = []

    for base_name, addr in (bases or []):
        const = base_name + "_BASE"
        used.add(const)
        lines.append(f"pub const {const}:".ljust(52) + f"u32 = {fmt_base(addr)};")
    if bases:
        lines.append("")

    for reg_name, off in (registers or []):
        const = f"{peripheral_name}_{sanitize(reg_name)}_OFFSET"
        used.add(const)
        lines.append(f"pub const {const}:".ljust(52) + f"u32 = 0x{off:X};")
    if registers:
        lines.append("")

    last_reg = None
    for reg_name, fld_name, high, low in fields:
        reg = sanitize(reg_name)
        fld = sanitize(fld_name)
        if fld == reg:
            base = f"{peripheral_name}_{reg}"
        else:
            base = f"{peripheral_name}_{reg}_{fld}"

        # Single-bit fields collapse to one _BIT constant for readability.
        if low == high:
            suffixes = ("_BIT",)
        else:
            suffixes = ("_LOW", "_HIGH")

        suffix = ""
        n = 1
        while any(base + suffix + s in used for s in suffixes):
            warnings.append(f"{peripheral_name}: {reg_name}.{fld_name} renamed with suffix {n}")
            suffix = f"_{n}"
            n += 1
        base += suffix

        if reg_name != last_reg:
            lines.append(f"// {reg_name}")
            last_reg = reg_name
        if low == high:
            lines.append(f"pub const {base}_BIT:".ljust(52) + f"u32 = {low};")
        else:
            lines.append(f"pub const {base}_LOW:".ljust(52) + f"u32 = {low};")
            lines.append(f"pub const {base}_HIGH:".ljust(52) + f"u32 = {high};")
        for s in suffixes:
            used.add(base + s)
    return lines, warnings


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
    periphs = {}
    root = ET.parse(SVD).getroot()
    for p in root.findall("./peripherals/peripheral"):
        periphs[p.findtext("name")] = p

    total_lines = 0
    all_warnings = []
    for stem, primary, aliases in build_outputs(periphs):
        path = os.path.join(CONST_DIR, stem + ".rs")
        exists = os.path.isfile(path)
        if not exists and not aliases:
            print(f"  ! no constants file for peripheral {primary} (skipped)", file=sys.stderr)
            continue

        peripheral = resolve(periphs, primary)
        fields = collect_fields(peripheral)

        if aliases:
            registers = collect_registers(peripheral)
            bases = [(primary, int(periphs[primary].findtext("baseAddress"), 0))]
            bases += [(a, int(periphs[a].findtext("baseAddress"), 0)) for a in aliases]
            content = open(path).read() if exists else f"#![allow(dead_code)]\n// {stem}\n"
        else:
            registers = None
            bases = None
            with open(path, "r") as fh:
                content = fh.read()

        reserved = existing_constants(content)
        body, warnings = build_block(stem, fields, reserved, bases, registers)
        all_warnings += warnings

        block = "\n".join(
            [BEGIN, "// Generated from specs/RP2040.svd -- do not edit by hand.", ""]
            + body
            + [END, ""]
        )

        head = content.split(BEGIN, 1)[0].rstrip("\n")
        with open(path, "w") as fh:
            fh.write(head + "\n\n" + block)

        n_consts = len([l for l in body if l.startswith("pub const")])
        total_lines += n_consts
        print(f"  {stem:24} fields={len(fields):4}  -> {n_consts} constants")

    print(f"\nTotal generated constants: {total_lines}")
    if all_warnings:
        print("\nRenames (bare name already taken):")
        for w in all_warnings:
            print("  " + w)


if __name__ == "__main__":
    main()