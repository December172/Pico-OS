#!/usr/bin/env python3


"""Shared helpers for the SVD constant generators.

Both tools/gen_field_ranges.py and tools/gen_enum_values.py (plus
tools/gen_register_offsets.py) read an SVD through this module so that naming,
peripheral merging and register-family detection stay identical between the
generated blocks.

Device selection
----------------
The active device is chosen with the SVD_DEVICE environment variable (default
RP2040), which selects the SVD file, the output directory and the peripheral
merge map:

    SVD_DEVICE=RP2350 python tools/gen_field_ranges.py

tools/gen_constants.py drives all three generators for a device.

Register families
-----------------
The SVD repeats the same register layout for many instance families
(GPIO0..GPIO29, CH0..CH11, SM0..SM3, EP0..EP15, INSTR_MEM0..31, ...).  Emitting
one constant set per instance produces thousands of near-duplicate constants.

`register_families()` collapses that repetition: registers that expose an
*identical* layout (the same digit-free field tokens with identical bit ranges,
in the same order) are grouped together, and each group is emitted once under a
shared family token.  Instances that only differ by a trailing number (the
common case) therefore collapse to one token with the digits removed, e.g.

    PIO_INSTR_MEM0_LOW ... PIO_INSTR_MEM31_LOW  ->  PIO_INSTR_MEM_LOW

Groups that share an identical layout but have no common digit-free name are
left as separate instances unless their names expose a common leading token
stream (checked with `_merge_token`).

Every identifier is strict UPPER_SNAKE_CASE; illegal characters are folded to
'_', underscore runs are collapsed and edges trimmed.
"""

import os
import re
import xml.etree.ElementTree as ET

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# Device profiles.  The active device is selected with the SVD_DEVICE
# environment variable (default RP2040) so an existing generator invocation
# keeps working unchanged:
#
#     python tools/gen_field_ranges.py
#     SVD_DEVICE=RP2350 python tools/gen_field_ranges.py
#
# `merged` maps the output file stem -> primary peripheral.  Peripherals that
# share an identical register/field layout are emitted into one file; sibling
# instances are discovered via the `derivedFrom` attribute (e.g. PIO1 derives
# from PIO0), so the aliases do not need to be listed here.
DEVICES = {
    "RP2040": {
        "svd": os.path.join(ROOT, "specs", "RP2040.svd"),
        "const_dir": os.path.join(ROOT, "src", "Native", "Constants", "RP2040"),
        "merged": {
            "PIO": "PIO0",
            "I2C": "I2C0",
            "UART": "UART0",
            "SPI": "SPI0",
            "PLL": "PLL_SYS",
        },
    },
    "RP2350": {
        "svd": os.path.join(ROOT, "specs", "RP2350.svd"),
        "const_dir": os.path.join(ROOT, "src", "Native", "Constants", "RP2350"),
        "merged": {
            "PIO": "PIO0",
            "I2C": "I2C0",
            "UART": "UART0",
            "SPI": "SPI0",
            "TIMER": "TIMER0",
            "PLL": "PLL_SYS",
            "PPB": "PPB",
            "SIO": "SIO",
        },
    },
}

DEVICE = os.environ.get("SVD_DEVICE", "RP2040")
if DEVICE not in DEVICES:
    raise SystemExit(f"unknown SVD device {DEVICE!r} (known: {', '.join(DEVICES)})")

SVD = DEVICES[DEVICE]["svd"]
SVD_NAME = os.path.basename(SVD)
CONST_DIR = DEVICES[DEVICE]["const_dir"]
MERGED = DEVICES[DEVICE]["merged"]

IDENT = re.compile(r"[^0-9A-Za-z_]")
DIGITS = re.compile(r"\d+")
UNDERSCORES = re.compile(r"_+")
BITRANGE = re.compile(r"\[(\d+):(\d+)\]")

# Idempotent-regeneration markers for each generated block.  Blocks are kept in
# this canonical order (register addresses, then field bit ranges, then
# enumerated values) so `upsert_block` can insert a missing block ahead of the
# later ones.
REG_BEGIN = "// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ===="
REG_END = "// ==== END AUTO-GENERATED REGISTER OFFSETS ===="
FIELD_BEGIN = "// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ===="
FIELD_END = "// ==== END AUTO-GENERATED FIELD BIT RANGES ===="
ENUM_BEGIN = "// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ===="
ENUM_END = "// ==== END AUTO-GENERATED ENUMERATED VALUES ===="
BLOCK_ORDER = (REG_BEGIN, FIELD_BEGIN, ENUM_BEGIN)
# Legacy marker block written by the former standalone tools/gen_auxsrc_enums.py.
LEGACY_ENUM_BEGIN = "// ==== BEGIN AUTO-GENERATED AUXSRC ENUMERATED VALUES (tools/gen_auxsrc_enums.py) ===="
LEGACY_ENUM_END = "// ==== END AUTO-GENERATED AUXSRC ENUMERATED VALUES ===="


def split_block(content, begin, end):
    """Return (before, inside, after) for a delimited block; empty inside/after if absent."""
    idx = content.find(begin)
    if idx < 0:
        return content, "", ""
    jdx = content.find(end, idx)
    if jdx < 0:
        return content[:idx], content[idx + len(begin):], ""
    return content[:idx], content[idx + len(begin):jdx], content[jdx + len(end):]


def upsert_block(content, begin, end, lines):
    """Replace (or insert) the [begin..end] block, preserving every other block.

    Blocks are ordered canonicallly via `BLOCK_ORDER`; a missing block is
    inserted ahead of the first block that should follow it, otherwise appended.
    """
    block = begin + "\n" + "\n".join(lines) + "\n" + end
    before, _inside, after = split_block(content, begin, end)
    if _inside or begin in content:
        out = before.rstrip("\n") + "\n\n" + block
        tail = after.strip("\n")
        if tail:
            out += "\n\n" + tail
        return out + "\n"

    pos = len(content)
    mine = BLOCK_ORDER.index(begin)
    for later in BLOCK_ORDER[mine + 1:]:
        p = content.find(later)
        if p >= 0:
            pos = min(pos, p)
    head = content[:pos].rstrip("\n")
    tail = content[pos:].strip("\n")
    out = head + "\n\n" + block
    if tail:
        out += "\n\n" + tail
    return out + "\n"


def sanitize(name: str) -> str:
    """Fold a raw SVD name into a strict UPPER_SNAKE_CASE identifier."""
    name = IDENT.sub("_", name).upper()
    name = UNDERSCORES.sub("_", name).strip("_")
    if name and name[0].isdigit():
        name = "_" + name
    return name


def token(name: str) -> str:
    """Strip digit runs then sanitize -- collapses numbered instance families."""
    return sanitize(DIGITS.sub("", name))


def ident(*parts) -> str:
    """Join parts into a strict UPPER_SNAKE_CASE identifier."""
    return sanitize("_".join(p for p in parts if p))


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


def parse_svd():
    """Return an ordered {peripheral_name: element} map for the active device SVD."""
    root = ET.parse(SVD).getroot()
    return {p.findtext("name"): p for p in root.findall("./peripherals/peripheral")}


def peripheral_base(periphs, name):
    """Return the base address of a peripheral as an int."""
    return int(periphs[name].findtext("baseAddress"), 0)


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


def output_groups(periphs):
    """Return [(file_stem, primary_name, [alias_names])] with the MERGED map applied.

    Peripherals merged into a shared file are emitted once under their primary
    name with every `derivedFrom` sibling listed as an alias; everything else is
    emitted one-file-per-peripheral under its own name.
    """
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


def existing_constants(text):
    """Names of pub const identifiers already present in the file (outside our block)."""
    return set(re.findall(r"pub const\s+([0-9A-Za-z_]+)\s*:", text))


def field_range(field):
    """Return (high, low) for an SVD <field> element."""
    br = field.findtext("bitRange")
    if br is None:
        lo = int(field.findtext("bitOffset"))
        high = lo + int(field.findtext("bitWidth")) - 1
        return high, lo
    m = BITRANGE.fullmatch(br.strip())
    if not m:
        raise RuntimeError(f"unparsable bitRange {br!r}")
    return int(m.group(1)), int(m.group(2))


def collect_register_fields(peripheral):
    """Return [(register_name, [(field_name, high, low, enum_values), ...]), ...].

    `enum_values` is a list of (name, value) pairs in document order (empty when
    the field carries no <enumeratedValues>).  Registers keep document order.
    """
    out = []
    for reg in peripheral.findall("./registers/register"):
        reg_name = reg.findtext("name")
        fields = []
        for f in reg.findall("./fields/field"):
            high, low = field_range(f)
            values = []
            for ev in f.findall("./enumeratedValues/enumeratedValue"):
                val = ev.findtext("value")
                if val is None:
                    continue
                values.append((ev.findtext("name"), int(val, 0)))
            fields.append((f.findtext("name"), high, low, values))
        out.append((reg_name, fields))
    return out


def collect_register_offsets(peripheral):
    """Return [(register_name, address_offset), ...] in document order."""
    out = []
    for reg in peripheral.findall("./registers/register"):
        off = reg.findtext("addressOffset")
        out.append((reg.findtext("name"), int(off, 0) if off is not None else 0))
    return out


def _common_prefix(lists):
    out = []
    for parts in zip(*lists):
        if len(set(parts)) == 1:
            out.append(parts[0])
        else:
            break
    return out


def _common_suffix(lists):
    out = []
    for parts in zip(*[list(reversed(p)) for p in lists]):
        if len(set(parts)) == 1:
            out.append(parts[0])
        else:
            break
    return list(reversed(out))


def _merge_token(bases):
    """Build a shared token from the digit-free names of a layout group.

    Combines the common leading and trailing underscore-delimited tokens so a
    group like CLK_USB_CTRL/CLK_ADC_CTRL/CLK_RTC_CTRL collapses to CLK_CTRL.
    Returns None when the names share no usable token.
    """
    lists = [b.split("_") if b else [] for b in bases]
    pre = _common_prefix(lists)
    suf = _common_suffix(lists)
    room = min(len(l) for l in lists) - len(pre)
    if room < len(suf):
        suf = suf[:room]
    combined = pre + suf
    return "_".join(combined) if combined else None


class Family:
    """A group of registers sharing one identical field layout."""

    def __init__(self, regs, fields, name):
        self.regs = regs          # register names, document order
        self.fields = fields      # [(field_name, high, low, enum_values), ...]
        self.name = name          # shared name token (sanitized)

    @property
    def label(self):
        if len(self.regs) == 1:
            return self.regs[0]
        if len(self.regs) <= 8:
            return ", ".join(self.regs)
        return f"{self.regs[0]}..{self.regs[-1]}"


def register_families(registers):
    """Collapse registers into one Family per identical field layout.

    `registers` is the output of collect_register_fields().  Layout equality is
    judged on digit-free field tokens and their bit ranges, so TXF0..TXF3 (whose
    field names repeat the register name) collapse together.  A register whose
    digit-free name is shared by two or more layout-compatible siblings is
    merged into that family; otherwise it keeps its own (digit-preserving) name.

    A family keeps the union of *its own* member fields (de-duplicated by name
    and bit range), so token-equivalent layouts with distinct field names (e.g.
    ROSC FREQA DS0..DS3 and FREQB DS4..DS7) are not truncated or cross-merged.
    """
    order = []
    sig_regs = {}
    reg_fields = {}
    for reg_name, fields in registers:
        reg_fields[reg_name] = fields
        sig = tuple((token(f[0]), f[1], f[2]) for f in fields)
        if sig not in sig_regs:
            sig_regs[sig] = []
            order.append(sig)
        sig_regs[sig].append(reg_name)

    def union(group):
        seen = {}
        for reg_name in group:
            for field in reg_fields[reg_name]:
                seen.setdefault((sanitize(field[0]), field[1], field[2]), field)
        return list(seen.values())

    families = []
    used = set()
    for sig in order:
        regs = sig_regs[sig]
        subs = {}
        for reg_name in regs:
            subs.setdefault(token(reg_name), []).append(reg_name)

        if len(subs) == 1:
            base = next(iter(subs))
            group = subs[base]
            # Only a genuine numbered family drops its instance digits; a lone
            # register keeps its exact name (FC0_REF_KHZ, PADS_BANK0, ...).
            name = base if len(group) >= 2 else sanitize(group[0])
            if not name or name in used:
                name = sanitize(group[0])
            used.add(name)
            families.append(Family(group, union(group), name))
            continue

        merged = _merge_token(list(subs))
        if merged and merged not in used:
            used.add(merged)
            families.append(Family(regs, union(regs), merged))
        else:
            for base, group in subs.items():
                name = base if len(group) >= 2 else sanitize(group[0])
                if not name or name in used:
                    name = sanitize(group[0])
                used.add(name)
                families.append(Family(group, union(group), name))
    return families


def register_family_names(registers):
    """Return {register_name: family_token} for every register."""
    return {reg: family.name for family in register_families(registers) for reg in family.regs}


def family_field_tokens(family):
    """Yield (token, high, low) for a family, de-duplicating fields.

    Fields are folded by digit-free token only when several distinct field
    names share that token *and* agree on the bit range -- that collapses
    INSTR_MEM0..INSTR_MEM31 while leaving a lone PADS_BANK0 or FC0_REF_KHZ
    untouched.  When the digit variants disagree on the range the exact field
    names are kept so they stay distinguishable.
    """
    buckets = {}
    order = []
    for field_name, high, low, _values in family.fields:
        tok = token(field_name)
        if tok not in buckets:
            buckets[tok] = []
            order.append(tok)
        buckets[tok].append((field_name, high, low))

    for tok in order:
        items = buckets[tok]
        ranges = {(high, low) for _n, high, low in items}
        names = {sanitize(n) for n, _h, _l in items}
        if len(ranges) == 1 and len(names) > 1:
            yield tok, items[0][1], items[0][2]
        else:
            for field_name, high, low in items:
                yield sanitize(field_name), high, low
