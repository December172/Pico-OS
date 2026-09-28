#!/usr/bin/env python3


"""Generate enumerated-value constants for every register field of the active device.

The active device is selected with the SVD_DEVICE environment variable (default
RP2040); constants are written to src/Native/Constants/<DEVICE>/<PERIPHERAL>.rs.

For each register field that carries `<enumeratedValues>` in the CMSIS-SVD file
this script writes one constant per enumerated value:

    pub const <PERIPHERAL>_<REGISTER>_<FIELD>_<VALUE>: u32 = <value>;

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

from svd_common import (
    CONST_DIR,
    ENUM_BEGIN as BEGIN,
    ENUM_END as END,
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

def collect_enum_fields(peripheral):
    """Yield (register_name, field_name, [(value_name, value), ...]) in document order."""
    out = []
    for reg_name, fields in collect_register_fields(peripheral):
        for fld_name, _high, _low, values in fields:
            if values:
                out.append((reg_name, fld_name, values))
    return out


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
        if not fields:
            continue

        with open(path, "r") as fh:
            content = fh.read()

        reserved = existing_constants(split_block(content, BEGIN, END)[0])
        registers = collect_register_fields(peripheral)
        reg_tokens = register_family_names(registers)

        body = []
        warnings = []

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
