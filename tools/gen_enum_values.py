#!/usr/bin/env python3


"""Generate enumerated-value constants for every register field of the active device.

The active device is selected with the SVD_DEVICE environment variable (default
RP2040); constants are written to src/Native/Constants/<DEVICE>/<PERIPHERAL>.rs.

For each register field that carries `<enumeratedValues>` in the CMSIS-SVD file
this script writes one constant per enumerated value:

    pub const <PERIPHERAL>_<REGISTER>_<FIELD>_<VALUE>: u32 = <value>;

The dedicated CLOCKS clock-source generator (formerly tools/gen_auxsrc_enums.py)
is merged into this script.  The `AUXSRC` and `SRC` fields of the `CLK_*_CTRL`
registers are named for their meaning instead:

    pub const CLOCKS_CLK_<CLOCK>_<SOURCE>_AUXSOURCE: u32 = <value>;  // AUXSRC
    pub const CLOCKS_CLK_<CLOCK>_<SOURCE>_SRC:       u32 = <value>;  // SRC

(e.g. CLOCKS_CLK_SYS_PLL_SYS_AUXSOURCE, CLOCKS_CLK_REF_XOSC_SRC).  Those two
fields are skipped by the generic pass so no duplicate constants appear.

Duplication handling
--------------------
Registers that share an identical field layout are collapsed into one family by
svd_common.register_families(); the family name is used in place of the
individual register name, so GPIO0..GPIO29 emit a single set of enum constants
under GPIO_CTRL and CH0..CH11 under CH_CTRL_TRIG.  Numeric instance suffixes are
removed from the shared name.  When two members of a family disagree on an
enum value the conflicting field is emitted per register instead, so different
values are never silently merged.

Naming:
  * identifiers are fully qualified strict UPPER_SNAKE_CASE -- illegal
    characters are folded to '_', underscore runs are collapsed and edges
    trimmed, and no identifier starts with a digit.  Enum values such as
    `3V3`, `1_15MHZ` or `128` therefore yield clean names
    (`..._VOLTAGE_SELECT_3V3`, `..._FREQ_RANGE_1_15MHZ`, `..._OFFSET_128`).
  * when a field name repeats its family name the redundant component is
    dropped.
  * names that would collide with the bit-range block (or another enum) are
    renamed with a numeric suffix so nothing is silently overwritten.

Peripherals that only carry a `derivedFrom` attribute (I2C1, PIO1, PLL_USB,
SPI1, UART1) inherit the register/field layout of their base peripheral but keep
their own name prefix.

The generated block is delimited so it can be regenerated idempotently, and is
written *after* the field bit-range block (tools/gen_field_ranges.py) so the
generators do not clobber each other's output.
"""

import os
import re

from svd_common import (
    CONST_DIR,
    ENUM_BEGIN as BEGIN,
    ENUM_END as END,
    LEGACY_ENUM_BEGIN,
    LEGACY_ENUM_END,
    SVD_NAME,
    collect_register_fields,
    existing_constants,
    ident,
    merge_outputs,
    parse_svd,
    register_family_names,
    resolve,
    sanitize,
    split_block,
    token,
    upsert_block,
)

# CLOCKS source-selector fields and the constant suffix they use.  AUXSRC feeds
# the glitchy auxiliary mux, SRC the glitchless one.  These are emitted by the
# dedicated CLOCKS pass below and skipped by the generic enumerated-value pass.
ID_FIELDS = {"AUXSRC": "AUXSOURCE", "SRC": "SRC"}
SKIP = {("CLOCKS", "AUXSRC"), ("CLOCKS", "SRC")}


def collect_enum_fields(peripheral):
    """Yield (register_name, field_name, [(value_name, value), ...]) in document order.

    CLOCKS AUXSRC/SRC fields are skipped here -- they are emitted by the
    dedicated clock-source pass (build_auxsrc) with dedicated naming.
    """
    pname = peripheral.findtext("name")
    out = []
    for reg_name, fields in collect_register_fields(peripheral):
        for fld_name, _high, _low, values in fields:
            if (pname, fld_name) in SKIP:
                continue
            if values:
                out.append((reg_name, fld_name, values))
    return out


def source_token(enum_name: str) -> str:
    """Normalise an SVD enumerated-value name into an uppercase source token."""
    n = enum_name.lower()
    if n.endswith("_clksrc_ph"):
        n = n[: -len("_clksrc_ph")] + "_ph"
    elif n.endswith("_clksrc"):
        n = n[: -len("_clksrc")]
    if n.startswith("clksrc_"):
        n = n[len("clksrc_"):]
    return sanitize(n)


def clock_token(reg_name: str) -> str:
    n = reg_name
    if n.startswith("CLK_"):
        n = n[len("CLK_"):]
    if n.endswith("_CTRL"):
        n = n[: -len("_CTRL")]
    return sanitize(n)


def merge_clock_token(clock: str) -> str:
    """Collapse numbered sibling clocks (GPOUT0..GPOUT3) into one token."""
    if re.fullmatch(r"GPOUT\d+", clock):
        return "GPOUT"
    return clock


def collect_sources(peripheral):
    """Yield (field_name, register_name, bitrange_str, [(source, value), ...])."""
    out = []
    for reg_name, fields in collect_register_fields(peripheral):
        for fld_name, high, low, values in fields:
            if fld_name not in ID_FIELDS:
                continue
            sources = []
            for ev_name, val in values:
                sources.append((source_token(ev_name), val))
            out.append((fld_name, reg_name, f"[{high}:{low}]", sources))
    return out


def build_auxsrc(peripheral_name, entries, used):
    """Emit CLOCKS AUXSRC/SRC constants; returns (lines, warnings)."""
    lines = []
    warnings = []

    # Collapse sibling clocks that expose an identical source layout
    # (GPOUT0..GPOUT3); keep first-seen source order (ROSC and ROSC_PH, both
    # value 4, are both retained).  AUXSRC and SRC stay separate groups.
    order = []
    groups = {}
    for field_name, reg_name, br, values in entries:
        clock = merge_clock_token(clock_token(reg_name))
        key = (field_name, clock)
        if key not in groups:
            groups[key] = {"regs": [], "br": br, "values": [], "seen": set()}
            order.append(key)
        group = groups[key]
        group["regs"].append(reg_name)
        for src, val in values:
            if src in group["seen"]:
                continue
            group["seen"].add(src)
            group["values"].append((src, val))

    for field_name, clock in order:
        group = groups[(field_name, clock)]
        suffix = ID_FIELDS[field_name]
        emitted = []
        for src, val in group["values"]:
            const = ident(peripheral_name, "CLK", clock, src, suffix)
            if const in used:
                warnings.append(f"{group['regs'][0]}.{field_name}: {src} skipped (name already taken)")
                continue
            used.add(const)
            emitted.append(f"pub const {const}:".ljust(52) + f"u32 = 0x{val:X};")
        if not emitted:
            continue
        lines.append(f"// {', '.join(group['regs'])}.{field_name} {group['br']}")
        lines.extend(emitted)
        lines.append("")

    while lines and lines[-1] == "":
        lines.pop()
    return lines, warnings


def strip_old_blocks(content):
    """Remove any previously generated legacy AUXSRC block.

    The current enumerated-value block is replaced in place by `upsert_block`,
    so it is left untouched here.
    """
    if LEGACY_ENUM_BEGIN in content:
        pre, rest = content.split(LEGACY_ENUM_BEGIN, 1)
        post = rest.split(LEGACY_ENUM_END, 1)[1] if LEGACY_ENUM_END in rest else ""
        content = pre + post
    return content


def build_block(peripheral_name, fields, reserved, reg_tokens):
    """Group sibling registers and emit one constant per enumerated value.

    `reg_tokens` maps a register name to the name of the layout family it
    belongs to; family members merge unless their value maps disagree, in which
    case the field falls back to the exact register name.
    """
    used = set(reserved)
    lines = []
    warnings = []

    # key -> {"regs": [...], "values": {name: value}, "order": [...]}
    groups = {}
    order = []
    for reg_name, fld_name, values in fields:
        key = (reg_tokens.get(reg_name, token(reg_name)), token(fld_name))

        # Only merge into an existing group when the value maps are consistent;
        # a conflicting value forces a dedicated group keyed by the exact name.
        conflict = False
        if key in groups:
            merged = groups[key]["values"]
            for name, val in values:
                if name in merged and merged[name] != val:
                    conflict = True
                    break
        if conflict:
            key = (sanitize(reg_name), sanitize(fld_name))
        if key not in groups:
            groups[key] = {"regs": [], "values": {}, "order": []}
            order.append(key)
        group = groups[key]
        group["regs"].append(reg_name)
        for name, val in values:
            if name in group["values"]:
                continue
            group["values"][name] = val
            group["order"].append(name)

    for base, fld in order:
        group = groups[(base, fld)]
        # Drop the redundant field component when it repeats the family name
        # (comparing digit-free tokens so FC0_SRC.FC0_SRC -> FC0_SRC).
        stem = base if token(base) == fld else f"{base}_{fld}"
        emitted = []
        for name in group["order"]:
            const = ident(peripheral_name, stem, name)
            # Disambiguate against bit-range constants (..._LOW/..._HIGH/..._BIT)
            # and any other emitted identifier by appending a suffix, mirroring
            # tools/gen_field_ranges.py.  Without this, enum values literally
            # named LOW/HIGH/ENABLE/... would be silently dropped.
            suffix = ""
            n = 1
            while const + suffix in used:
                suffix = f"_{n}"
                n += 1
            if suffix:
                warnings.append(f"{group['regs'][0]}.{name}: {const} renamed with {suffix.lstrip('_')}")
            const += suffix
            used.add(const)
            emitted.append(f"pub const {const}:".ljust(52) + f"u32 = 0x{group['values'][name]:X};")
        if not emitted:
            continue
        regs = group["regs"]
        label = regs[0] if len(regs) == 1 else f"{regs[0]}..{regs[-1]}"
        lines.append(f"// {label}: {fld}")
        lines.extend(emitted)
        lines.append("")

    while lines and lines[-1] == "":
        lines.pop()
    return lines, warnings


def main():
    periphs = parse_svd()
    primary_stem, alias_of = merge_outputs(periphs)

    total = 0
    all_warnings = []
    for name in periphs:
        if name in alias_of:
            continue  # folded into the primary peripheral's merged file

        stem = primary_stem.get(name, name)
        path = os.path.join(CONST_DIR, stem + ".rs")
        if not os.path.isfile(path):
            continue

        peripheral = resolve(periphs, name)
        fields = collect_enum_fields(peripheral)
        aux_entries = collect_sources(peripheral) if name == "CLOCKS" else []
        if not fields and not aux_entries:
            continue

        with open(path, "r") as fh:
            content = strip_old_blocks(fh.read())

        reserved = existing_constants(split_block(content, BEGIN, END)[0])
        registers = collect_register_fields(peripheral)
        reg_tokens = register_family_names(registers)

        body = []
        warnings = []

        # The dedicated CLOCKS clock-source set is written first so its
        # constants reserve their names before the generic pass runs.
        if aux_entries:
            aux_lines, aux_warn = build_auxsrc(stem, aux_entries, reserved)
            body += aux_lines
            if aux_lines:
                body.append("")
            warnings += aux_warn

        if fields:
            gen_lines, gen_warn = build_block(stem, fields, reserved, reg_tokens)
            body += gen_lines
            warnings += gen_warn

        while body and body[-1] == "":
            body.pop()
        all_warnings += [f"{name}: {w}" for w in warnings]
        if not body:
            continue

        lines = [f"// Generated from specs/{SVD_NAME} -- do not edit by hand.", ""] + body
        content = upsert_block(content, BEGIN, END, lines)
        with open(path, "w") as fh:
            fh.write(content)

        n = len([l for l in body if l.startswith("pub const")])
        total += n
        print(f"  {stem:24} enum_fields={len(fields):4}  -> {n} constants")

    print(f"\nTotal generated constants: {total}")
    if all_warnings:
        print("\nRenames (bare name already taken):")
        for w in all_warnings:
            print("  " + w)


if __name__ == "__main__":
    main()
