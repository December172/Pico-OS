#!/usr/bin/env python3
"""Generate register addresses, field ranges and enum constants from one SVD.

    python3 tools/gen_constants.py
    python3 tools/gen_constants.py RP2040 --offsets

The default device is $SVD_DEVICE or RP2040. Address generation is optional for
RP2040; --offsets includes it and --reset discards curated declarations first.
All three blocks are generated in memory from one SVD parse, then each changed
Rust file is written once. Existing block markers are migrated automatically.
"""

import argparse
import os
from pathlib import Path
import re
import sys

import svd_common as svd
from svd_common import (
    ENUM_BEGIN, ENUM_END, FIELD_BEGIN, FIELD_END, REG_BEGIN, REG_END,
    collect_register_fields, collect_register_offsets, existing_constants,
    family_field_tokens, ident, normalize_blocks, output_groups,
    peripheral_base, register_families, resolve, rust_source_filename,
    rust_source_stem, sanitize, split_block, token, upsert_block, write_if_changed,
)

DIGITS = re.compile(r"\d+")


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("device", nargs="?", default=os.environ.get("SVD_DEVICE", "RP2040"),
                        help="SVD device to generate (default: $SVD_DEVICE or RP2040)")
    offsets = parser.add_mutually_exclusive_group()
    offsets.add_argument("--offsets", dest="offsets", action="store_true", default=None,
                         help="run the register-address pass")
    offsets.add_argument("--no-offsets", dest="offsets", action="store_false",
                         help="skip the register-address pass")
    parser.add_argument("--no-create", dest="create", action="store_false", default=None,
                        help="do not create missing per-peripheral files")
    parser.add_argument("--reset", action="store_true",
                        help="drop hand-curated declarations before regenerating "
                             "(one-time migration to fully generated files)")
    parser.add_argument("--write-mod", action="store_true",
                        help="rewrite mod.rs even when it already exists")
    args = parser.parse_args()
    if args.offsets is None:
        args.offsets = args.device != "RP2040"
    if args.reset and not args.offsets:
        parser.error("--reset requires the register-address pass (--offsets)")
    return args


def fmt_base(value):
    """Format a peripheral base address as 0xAAAA_BBBB."""
    return f"0x{value >> 16:04X}_{value & 0xFFFF:04X}"


def fmt_off(value):
    """Format a register offset as a short lowercase hex literal."""
    return f"0x{value:X}"


def strip_nth_digit(name, index):
    """Return `name` with its (index+1)-th digit run removed."""
    seen = 0

    def repl(match):
        nonlocal seen
        cur = seen
        seen += 1
        return "" if cur == index else match.group(0)

    return DIGITS.sub(repl, name)


def collapse_index_run(run, pos):
    """Collapse a constant-stride run using digit-run `pos` as the index.

    Returns the shared template (with that digit run removed) when the run's
    position values are exactly 0..len-1, otherwise None.
    """
    if len(run) < 2:
        return None
    starts = []
    for name, _off in run:
        digits = DIGITS.findall(name)
        if pos >= len(digits):
            return None
        starts.append(int(digits[pos]))
    if starts != list(range(len(run))):
        return None
    templates = {sanitize(strip_nth_digit(n, pos)) for n, _o in run}
    return templates.pop() if len(templates) == 1 else None


def split_runs(members):
    """Split offset-sorted members into maximal constant-stride runs."""
    runs = []
    i = 0
    while i < len(members):
        j = i
        stride = None
        while j + 1 < len(members):
            nxt = members[j + 1][1] - members[j][1]
            if stride is None:
                stride = nxt
            if nxt != stride:
                break
            j += 1
        runs.append((stride, members[i:j + 1]))
        i = j + 1
    return runs


def plan_group(members):
    """Plan one digit-free-token group, choosing the best collapse dimension.

    Registers such as the DMA channel/alias matrix (CH0_AL1_CTRL ..
    CH11_AL3_CTRL) carry two varying digit runs.  Each varying run is tried as
    the function index in turn, the members are partitioned by the remaining
    varying runs, and the candidate collapsing the most registers wins.  Ties
    prefer the later run so names like INTERP0_ACCUM and IRQSUMMARY_PROC0_SECURE
    survive.
    """
    members = sorted(members, key=lambda m: m[1])
    splits = [DIGITS.findall(n) for n, _o in members]
    if len({len(s) for s in splits}) != 1 or not splits[0]:
        return [("const", n, o) for n, o in members]

    count = len(splits[0])
    columns = [[int(s[p]) for s in splits] for p in range(count)]
    varying = [p for p in range(count) if len(set(columns[p])) > 1]
    if not varying:
        return [("const", n, o) for n, o in members]

    best = None  # (score, pos, items)
    for pos in varying:
        others = [q for q in varying if q != pos]
        partitions = {}
        for (name, off), values in zip(members, splits):
            key = tuple(int(values[q]) for q in others)
            partitions.setdefault(key, []).append((name, off))

        items = []
        score = 0
        for group in partitions.values():
            for stride, run in split_runs(group):
                template = collapse_index_run(run, pos) if stride and stride > 0 else None
                if template:
                    items.append(("fn", template, run[0][1], stride, len(run),
                                  [n for n, _o in run]))
                    score += len(run)
                else:
                    items.extend(("const", n, o) for n, o in run)
        if best is None or (score, pos) >= (best[0], best[1]):
            best = (score, pos, items)
    return best[2]


def plan_registers(registers):
    """Collapse register offsets into ('const'|'fn', ...) items.

    Returns entries:
      ("const", name, offset)
      ("fn", template, base_offset, stride, count, [member names])
    """
    groups = {}
    order = []
    for name, off in registers:
        tok = token(name)
        if tok not in groups:
            groups[tok] = []
            order.append(tok)
        groups[tok].append((name, off))

    items = []
    for tok in order:
        items.extend(plan_group(groups[tok]))
    return items


def render_registers(items, bases):
    """Render the absolute register-address accessors for one peripheral.

    One `<INSTANCE>_<REG>` constant (or `<INSTANCE>_<REG>(index)` function) is
    emitted per instance, each as an absolute `<INSTANCE>_BASE + offset`.
    """
    lines = []
    used = set()

    def unique(name):
        if name not in used:
            used.add(name)
            return name
        suffix = ""
        n = 1
        while name + suffix in used:
            suffix = f"_{n}"
            n += 1
        name += suffix
        used.add(name)
        return name

    for base_name, _addr in bases:
        for item in items:
            if item[0] == "const":
                _kind, name, off = item
                const = unique(ident(base_name, name))
                lines.append(f"pub const {const}:".ljust(52)
                             + f"u32 = {base_name}_BASE + {fmt_off(off)};")
            else:
                _kind, template, off, stride, count, members = item
                param = "n"
                fn = unique(ident(base_name, template))
                expr = f"{base_name}_BASE + {fmt_off(off)} + {param} * {fmt_off(stride)}"
                lines.append(f"// {members[0]}..{members[-1]}")
                lines.append(f"pub fn {fn}({param}: u32) -> u32 {{")
                lines.append(f"    return {expr}")
                lines.append("}")
                lines.append("")
    while lines and lines[-1] == "":
        lines.pop()
    return lines


def build_register_block(registers, bases):
    """Return the generated lines for one output file."""
    body = []
    for name, addr in bases:
        body.append(f"pub const {name}_BASE:".ljust(52) + f"u32 = {fmt_base(addr)};")
    body.append("")
    body.extend(render_registers(plan_registers(registers), bases))
    return body


class FieldEmitter:
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


def build_field_block(peripheral_name, families, reserved, bases=None, offsets=None):
    """Return the generated lines for one peripheral.

    `bases` lists the per-instance <NAME>_BASE constants for merged peripherals,
    `offsets` the shared <PERIPHERAL>_<REG>_OFFSET constants (for legacy merged files);
    both are emitted ahead of the shared field bit-range lines.
    """
    lines = []
    emit = FieldEmitter(reserved)

    for base_name, addr in (bases or []):
        emit.add(lines, base_name + "_BASE", fmt_base(addr))
    if bases:
        lines.append("")

    for reg_name, off in (offsets or []):
        emit.add(lines, f"{peripheral_name}_{sanitize(reg_name)}_OFFSET", f"0x{off:X}")
    if offsets:
        lines.append("")

    for family in families:
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


def build_enum_block(peripheral_name, fields, reserved, reg_tokens):
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
            # the field-range emitter. Without this, enum values literally
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


def generate(periphs, args):
    groups = output_groups(periphs)
    directory = Path(svd.CONST_DIR)
    directory.mkdir(parents=True, exist_ok=True)
    created = 0
    totals = [0, 0, 0]
    all_warnings = []
    reset = args.reset or os.environ.get("SVD_RESET_MANUAL") == "1"
    override = "0" if args.offsets else os.environ.get("SVD_EMIT_BASES_OFFSETS")

    for stem, primary, aliases in groups:
        path = directory / rust_source_filename(stem)
        exists = path.is_file()
        if not exists and not aliases and not args.create:
            print(f"  ! no constants file for peripheral {primary} (skipped)", file=sys.stderr)
            continue

        previous = path.read_text() if exists else None
        skeleton = f"#![allow(dead_code)]\n// {stem}\n"
        content = normalize_blocks(previous if exists else skeleton)
        if args.offsets and reset:
            content = skeleton
        if args.create and not exists:
            created += 1

        peripheral = resolve(periphs, primary)
        registers = collect_register_fields(peripheral)
        offsets = collect_register_offsets(peripheral)
        families = register_families(registers)
        bases = [(primary if aliases else stem, peripheral_base(periphs, primary))]
        bases += [(alias, peripheral_base(periphs, alias)) for alias in aliases]
        source = [f"// Generated from specs/{svd.SVD_NAME} -- do not edit by hand.", ""]
        counts = [0, 0, 0]

        if args.offsets:
            body = build_register_block(offsets, bases)
            content = upsert_block(content, REG_BEGIN, REG_END, source + body)
            counts[0] = sum(line.startswith(("pub const", "pub fn")) for line in body)

        emit_offsets = REG_BEGIN not in content if override is None else override != "0"
        reserved = existing_constants(split_block(content, FIELD_BEGIN, FIELD_END)[0])
        body = build_field_block(stem, families, reserved,
                                 bases if aliases and emit_offsets else None,
                                 offsets if aliases and emit_offsets else None)
        content = upsert_block(content, FIELD_BEGIN, FIELD_END, source + body)
        counts[1] = sum(line.startswith("pub const") for line in body)

        enum_fields = [(reg, name, values) for reg, fields in registers
                       for name, _high, _low, values in fields if values]
        if enum_fields:
            reserved = existing_constants(split_block(content, ENUM_BEGIN, ENUM_END)[0])
            reg_tokens = {reg: family.name for family in families for reg in family.regs}
            body, warnings = build_enum_block(stem, enum_fields, reserved, reg_tokens)
            all_warnings += [f"{primary}: {warning}" for warning in warnings]
            if body:
                content = upsert_block(content, ENUM_BEGIN, ENUM_END, source + body)
                counts[2] = sum(line.startswith("pub const") for line in body)

        write_if_changed(path, content, previous)
        totals = [total + count for total, count in zip(totals, counts)]
        print(f"  {stem:24} registers={len(registers):4} -> "
              f"{counts[0]} accessors, {counts[1]} ranges, {counts[2]} enum values")

    if created:
        print(f"Created {created} constants file(s) in {directory}")
    mod_path = directory / "mod.rs"
    if args.write_mod or not mod_path.exists():
        lines = [f"pub mod {rust_source_stem(stem)};" for stem, _p, _a in groups]
        if (directory / "config.rs").exists():
            lines += ["", "// For non-predefined user values", "pub mod config;"]
        previous = mod_path.read_text() if mod_path.exists() else None
        write_if_changed(mod_path, "\n".join(lines) + "\n", previous)

    print(f"\nTotal: {totals[0]} accessors, {totals[1]} ranges, {totals[2]} enum values")
    if all_warnings:
        print("\nRenames (bare name already taken):")
        for warning in all_warnings:
            print(f"  {warning}")
    print(f"\nDone: {svd.SVD_NAME} -> {directory}")


def main():
    args = parse_args()
    svd.configure_device(args.device)
    if args.create is None:
        args.create = args.device != "RP2040"
    try:
        generate(svd.parse_svd(), args)
    except (OSError, ValueError, RuntimeError) as error:
        raise SystemExit(f"gen_constants: {error}") from error


if __name__ == "__main__":
    main()
