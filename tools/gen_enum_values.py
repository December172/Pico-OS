#!/usr/bin/env python3


"""Generate enumerated-value constants for every RP2040 register field from specs/RP2040.svd.

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

Naming:
  * identifiers are fully qualified strict UPPER_SNAKE_CASE -- illegal
    characters are folded to '_', underscore runs are collapsed and edges
    trimmed, and no identifier starts with a digit.  Enum values such as
    `3V3`, `1_15MHZ` or `128` therefore yield clean names
    (`..._VOLTAGE_SELECT_3V3`, `..._FREQ_RANGE_1_15MHZ`, `..._OFFSET_128`).
  * numbered sibling registers that expose an identical field layout
    (GPIO0..GPIO29, CH0..CH11, SM0..SM3, EP1..EP15, CLK_GPOUT0..3, ...) are
    collapsed into a single constant set with the digit runs stripped from the
    register name.
  * when a field name repeats its register name the redundant component is
    dropped.

Peripherals that only carry a `derivedFrom` attribute (I2C1, PIO1, PLL_USB,
SPI1, UART1) inherit the register/field layout of their base peripheral but keep
their own name prefix.

The generated block is delimited so it can be regenerated idempotently, and is
written *after* the field bit-range block (tools/gen_field_ranges.py) so the
generators do not clobber each other's output.  Hand-written constants already
present in the file (including the bit-range block) are treated as reserved:
any generated identifier that would collide is renamed with a numeric suffix so
existing constants are never overwritten.
"""

import os
import re
import xml.etree.ElementTree as ET

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SVD = os.path.join(ROOT, "specs", "RP2040.svd")
CONST_DIR = os.path.join(ROOT, "src", "Native", "Constants", "RP2040")

BEGIN = "// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ===="
END = "// ==== END AUTO-GENERATED ENUMERATED VALUES ===="

# Legacy marker block written by the former standalone tools/gen_auxsrc_enums.py
# (kept so stale output from the old two-script setup is removed on regeneration).
LEGACY_BEGIN = "// ==== BEGIN AUTO-GENERATED AUXSRC ENUMERATED VALUES (tools/gen_auxsrc_enums.py) ===="
LEGACY_END = "// ==== END AUTO-GENERATED AUXSRC ENUMERATED VALUES ===="

# CLOCKS source-selector fields and the constant suffix they use.  AUXSRC feeds
# the glitchy auxiliary mux, SRC the glitchless one.  These are emitted by the
# dedicated CLOCKS pass below and skipped by the generic enumerated-value pass.
ID_FIELDS = {"AUXSRC": "AUXSOURCE", "SRC": "SRC"}
SKIP = {("CLOCKS", "AUXSRC"), ("CLOCKS", "SRC")}

# Peripherals sharing an identical layout are emitted into one file: maps the
# output file stem -> primary peripheral.  Sibling instances (discovered via
# `derivedFrom`, e.g. PIO1 derives from PIO0) are folded into the primary and
# their enum constants are emitted once with the shared stem prefix.
MERGED = {
    "PIO": "PIO0",
    "I2C": "I2C0",
    "UART": "UART0",
    "SPI": "SPI0",
}


def merge_outputs(periphs):
    """Return (primary->stem, alias->primary) maps describing the MERGED merges."""
    primary_stem = {}
    alias_of = {}
    for stem, primary in MERGED.items():
        primary_stem[primary] = stem
        for name, p in periphs.items():
            if name != primary and p.get("derivedFrom") == primary:
                alias_of[name] = primary
    return primary_stem, alias_of

IDENT = re.compile(r"[^0-9A-Za-z_]")
DIGITS = re.compile(r"\d+")
UNDERSCORES = re.compile(r"_+")


def sanitize(name: str) -> str:
    # strict UPPER_SNAKE_CASE: fold illegal chars, collapse underscore runs and
    # trim the edges so values like "3V3", "1_15MHZ" or "128" become clean
    # tokens (3V3, 1_15MHZ, 128) instead of producing "__1_15MHZ"/"__3V3".
    name = IDENT.sub("_", name).upper()
    name = UNDERSCORES.sub("_", name).strip("_")
    if name and name[0].isdigit():
        name = "_" + name
    return name


def ident(*parts) -> str:
    """Join parts into a strict UPPER_SNAKE_CASE identifier."""
    return sanitize("_".join(p for p in parts if p))


def token(name: str) -> str:
    """Strip digit runs then sanitize -- collapses numbered register families."""
    return sanitize(DIGITS.sub("", name))


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


def collect_enum_fields(peripheral):
    """Yield (register_name, field_name, [(value_name, value), ...]) in document order.

    CLOCKS AUXSRC/SRC fields are skipped here -- they are emitted by the
    dedicated clock-source pass (build_auxsrc) with dedicated naming.
    """
    pname = peripheral.findtext("name")
    out = []
    for reg in peripheral.findall("./registers/register"):
        reg_name = reg.findtext("name")
        for f in reg.findall("./fields/field"):
            fld_name = f.findtext("name")
            if (pname, fld_name) in SKIP:
                continue
            evs = f.findall("./enumeratedValues/enumeratedValue")
            if not evs:
                continue
            values = []
            for ev in evs:
                val = ev.findtext("value")
                if val is None:
                    continue
                values.append((ev.findtext("name"), int(val, 0)))
            if values:
                out.append((reg_name, f.findtext("name"), values))
    return out


def existing_constants(text):
    """Names of pub const identifiers already present in the file (outside our block)."""
    head = text.split(BEGIN, 1)[0]
    return set(re.findall(r"pub const\s+([0-9A-Za-z_]+)\s*:", head))


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
    """Collapse numbered sibling clocks (GPOUT0..GPOUT3) into one token."""
    if re.fullmatch(r"GPOUT\d+", clock):
        return "GPOUT"
    return clock


def collect_sources(peripheral):
    """Yield (field_name, register_name, bitrange_str, [(source, value), ...])."""
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
                m = re.fullmatch(r"\[(\d+):(\d+)\]", br.strip())
                if not m:
                    raise RuntimeError(f"unparsable bitRange {br!r} in {reg_name}.{fld_name}")
                br = f"[{m.group(1)}:{m.group(2)}]"

            values = []
            for ev in f.findall("./enumeratedValues/enumeratedValue"):
                val = ev.findtext("value")
                if val is None:
                    continue
                values.append((source_token(ev.findtext("name")), int(val, 0)))
            out.append((fld_name, reg_name, br, values))
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
    """Remove any previously generated block (merged and/or legacy AUXSRC)."""
    content = content.split(BEGIN, 1)[0]
    if LEGACY_BEGIN in content:
        pre, rest = content.split(LEGACY_BEGIN, 1)
        post = rest.split(LEGACY_END, 1)[1] if LEGACY_END in rest else ""
        content = pre + post
    return content


def build_block(peripheral_name, fields, reserved):
    """Group numbered sibling registers and emit one constant per enumerated value."""
    used = set(reserved)
    lines = []
    warnings = []

    # key -> {"regs": [...], "values": {name: value}, "order": [...]}
    groups = {}
    order = []
    for reg_name, fld_name, values in fields:
        base = token(reg_name)
        fld = token(fld_name)
        key = (base, fld)

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
        # Drop the redundant field component when it repeats the register name.
        stem = base if fld == base else f"{base}_{fld}"
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
    periphs = {}
    root = ET.parse(SVD).getroot()
    for p in root.findall("./peripherals/peripheral"):
        periphs[p.findtext("name")] = p

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

        reserved = existing_constants(content)
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
            gen_lines, gen_warn = build_block(stem, fields, reserved)
            body += gen_lines
            warnings += gen_warn

        while body and body[-1] == "":
            body.pop()
        all_warnings += [f"{name}: {w}" for w in warnings]
        if not body:
            continue

        block = "\n".join(
            [BEGIN, "// Generated from specs/RP2040.svd -- do not edit by hand.", ""]
            + body
            + [END, ""]
        )

        head = content.rstrip("\n")
        with open(path, "w") as fh:
            fh.write(head + "\n\n" + block)

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