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

Peripherals that only carry a `derivedFrom` attribute (I2C1, PIO1, PLL_USB,
SPI1, UART1) inherit the register/field layout of their base peripheral but keep
their own name prefix.

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


def existing_constants(text):
    """Names of pub const identifiers already present in the file (outside generated block)."""
    head = text.split(BEGIN, 1)[0]
    return set(re.findall(r"pub const\s+([0-9A-Za-z_]+)\s*:", head))


def build_block(peripheral_name, fields, reserved):
    """Return the generated lines for one peripheral and report name conflicts.

    Every constant is fully qualified as <PERIPHERAL>_<REGISTER>_<FIELD>_LOW/_HIGH.
    When the field name repeats its register name the redundant component is
    dropped (PLL_SYS.FBDIV_INT.FBDIV_INT -> PLL_SYS_FBDIV_INT_LOW/_HIGH).
    """
    used = set(reserved)
    lines = []
    warnings = []
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


def main():
    periphs = {}
    root = ET.parse(SVD).getroot()
    for p in root.findall("./peripherals/peripheral"):
        periphs[p.findtext("name")] = p

    total_lines = 0
    all_warnings = []
    for name in periphs:
        path = os.path.join(CONST_DIR, name + ".rs")
        if not os.path.isfile(path):
            print(f"  ! no constants file for peripheral {name} (skipped)", file=sys.stderr)
            continue

        peripheral = resolve(periphs, name)
        fields = collect_fields(peripheral)
        with open(path, "r") as fh:
            content = fh.read()

        reserved = existing_constants(content)
        body, warnings = build_block(name, fields, reserved)
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
        print(f"  {name:24} fields={len(fields):4}  -> {n_consts} constants")

    print(f"\nTotal generated constants: {total_lines}")
    if all_warnings:
        print("\nRenames (bare name already taken):")
        for w in all_warnings:
            print("  " + w)


if __name__ == "__main__":
    main()