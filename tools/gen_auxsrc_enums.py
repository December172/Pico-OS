#!/usr/bin/env python3


"""Generate clock-source enumerated-value constants for the RP2040 CLOCKS registers.

Several CLOCKS control registers (`CLK_GPOUT0_CTRL`, `CLK_REF_CTRL`,
`CLK_SYS_CTRL`, `CLK_PERI_CTRL`, `CLK_USB_CTRL`, `CLK_ADC_CTRL`,
`CLK_RTC_CTRL`, ...) expose source-selector fields whose legal values are
described by `<enumeratedValue>` entries in the CMSIS-SVD file:

  * `AUXSRC` -- the glitchy auxiliary mux source, and
  * `SRC`    -- the glitchless mux source (`CLK_REF_CTRL`, `CLK_SYS_CTRL`).

This script turns every one of those entries into a constant:

    pub const CLOCKS_CLK_<CLOCK>_<SOURCE>_AUXSOURCE: u32 = <value>;  // AUXSRC
    pub const CLOCKS_CLK_<CLOCK>_<SOURCE>_SRC:       u32 = <value>;  // SRC

where `<CLOCK>` is the register name with the leading `CLK_` and trailing
`_CTRL` stripped, and `<SOURCE>` is the SVD enumerated-value name normalised
into an uppercase source token, e.g.

    clksrc_pll_sys  -> PLL_SYS      clksrc_gpin0 -> GPIN0
    rosc_clksrc     -> ROSC         rosc_clksrc_ph -> ROSC_PH
    xosc_clksrc     -> XOSC         clk_sys      -> CLK_SYS
    clk_ref         -> CLK_REF      clksrc_clk_ref_aux -> CLK_REF_AUX

Usage with the accompanying bit-range constants:

    Register::fieldSet(CLOCKS_CLK_SYS_CTRL_AUXSRC_HIGH,
                       CLOCKS_CLK_SYS_CTRL_AUXSRC_LOW,
                       CLOCKS_CLK_SYS_PLL_SYS_AUXSOURCE);
    Register::fieldSet(CLOCKS_CLK_REF_CTRL_SRC_HIGH,
                       CLOCKS_CLK_REF_CTRL_SRC_LOW,
                       CLOCKS_CLK_REF_XOSC_SRC);

The generated block is delimited so it can be regenerated idempotently.  It is
written *after* the field bit-range block produced by tools/gen_field_ranges.py
so the two generators do not clobber each other's output.
"""

import os
import re
import sys
import xml.etree.ElementTree as ET

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SVD = os.path.join(ROOT, "specs", "RP2040.svd")
CONST_DIR = os.path.join(ROOT, "src", "Native", "Constants", "RP2040")

BEGIN = "// ==== BEGIN AUTO-GENERATED AUXSRC ENUMERATED VALUES (tools/gen_auxsrc_enums.py) ===="
END = "// ==== END AUTO-GENERATED AUXSRC ENUMERATED VALUES ===="
FIELD_END = "// ==== END AUTO-GENERATED FIELD BIT RANGES ===="

PERIPHERAL = "CLOCKS"
# Field name -> constant suffix.  Both fields select a clock source from a set
# of enumerated values; AUXSRC feeds the glitchy auxiliary mux, SRC the
# glitchless one.
ID_FIELDS = {"AUXSRC": "AUXSOURCE", "SRC": "SRC"}

IDENT = re.compile(r"[^0-9A-Za-z_]")
BITRANGE = re.compile(r"\[(\d+):(\d+)\]")


def sanitize(name: str) -> str:
    name = IDENT.sub("_", name).upper()
    if name and name[0].isdigit():
        name = "_" + name
    return name


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


def merge_token(clock: str) -> str:
    """Collapse numbered sibling clocks (GPOUT0..GPOUT3) into one token.

    The four GPOUT control registers expose an identical AUXSRC layout, so a
    single CLOCKS_CLK_GPOUT_<SOURCE>_AUXSOURCE set covers all of them.
    GPOUT2/3 name their 4th source rosc_clksrc_ph while GPOUT0/1 name it
    rosc_clksrc; both tokens (both value 4) are retained in the merged set.
    """
    if re.fullmatch(r"GPOUT\d+", clock):
        return "GPOUT"
    return clock


def find_peripheral(root, name):
    for p in root.findall("./peripherals/peripheral"):
        if p.findtext("name") == name:
            return p
    raise RuntimeError(f"peripheral {name} not found in {SVD}")


def collect_sources(peripheral):
    """Yield (field_name, register_name, bitrange_str, [(source_const, value), ...])."""
    out = []
    for reg in peripheral.findall("./registers/register"):
        reg_name = reg.findtext("name")
        for f in reg.findall("./fields/field"):
            fld_name = f.findtext("name")
            if fld_name not in ID_FIELDS:
                continue
            br = f.findtext("bitRange")
            if br is None:
                lo = int(f.findtext("bitOffset"))
                high = lo + int(f.findtext("bitWidth")) - 1
                br = f"[{high}:{lo}]"
            else:
                m = BITRANGE.fullmatch(br.strip())
                if not m:
                    raise RuntimeError(f"unparsable bitRange {br!r} in {reg_name}.{fld_name}")
                high, low = int(m.group(1)), int(m.group(2))
                br = f"[{high}:{low}]"

            values = []
            for ev in f.findall("./enumeratedValues/enumeratedValue"):
                src = source_token(ev.findtext("name"))
                val = ev.findtext("value")
                if val is None:
                    continue
                values.append((src, val))
            out.append((fld_name, reg_name, br, values))
    return out


def existing_constants(text):
    """Names of pub const identifiers already present in the file (outside our block)."""
    head = text.split(BEGIN, 1)[0]
    return set(re.findall(r"pub const\s+([0-9A-Za-z_]+)\s*:", head))


def build_block(peripheral_name, entries, reserved):
    lines = []
    warnings = []
    used = set(reserved)

    # Collapse sibling clocks that expose an identical source layout
    # (GPOUT0..GPOUT3) into a single constant set, keeping first-seen source
    # order: ROSC and ROSC_PH (both value 4) are both retained.  AUXSRC and SRC
    # are kept as separate groups even for the same clock (CLK_REF/CLK_SYS).
    order = []
    groups = {}
    for field_name, reg_name, br, values in entries:
        clock = merge_token(clock_token(reg_name))
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
            const = f"{peripheral_name}_CLK_{clock}_{src}_{suffix}"
            if const in used:
                warnings.append(f"{group['regs'][0]}.{field_name}: {src} skipped (name already taken)")
                continue
            used.add(const)
            emitted.append(f"pub const {const}:".ljust(52) + f"u32 = 0x{int(val):X};")
        if not emitted:
            continue
        lines.append(f"// {', '.join(group['regs'])}.{field_name} {group['br']}")
        lines.extend(emitted)
        lines.append("")

    while lines and lines[-1] == "":
        lines.pop()
    return lines, warnings


def main():
    root = ET.parse(SVD).getroot()
    peripheral = find_peripheral(root, PERIPHERAL)
    entries = collect_sources(peripheral)

    path = os.path.join(CONST_DIR, PERIPHERAL + ".rs")
    with open(path, "r") as fh:
        content = fh.read()

    reserved = existing_constants(content)
    body, warnings = build_block(PERIPHERAL, entries, reserved)

    block = "\n".join(
        [BEGIN, "// Generated from specs/RP2040.svd -- do not edit by hand.", ""]
        + body
        + [END, ""]
    )

    # Keep everything before our BEGIN marker (this includes the field-range
    # block written by tools/gen_field_ranges.py).
    head = content.split(BEGIN, 1)[0].rstrip("\n")

    with open(path, "w") as fh:
        fh.write(head + "\n\n" + block)

    n_consts = len([l for l in body if l.startswith("pub const")])
    print(f"  {PERIPHERAL:24} source_fields={len(entries):3}  -> {n_consts} constants")
    if warnings:
        print("\nSkipped:")
        for w in warnings:
            print("  " + w)


if __name__ == "__main__":
    main()
