#!/usr/bin/env python3


"""Generate register-address accessors for the active SVD device.

For every peripheral this script writes the register-address section of the
matching src/Native/Constants/<DEVICE>/<PERIPHERAL>.rs file.  Every accessor is
an absolute address: one `pub const` per register, or one `pub fn` for a
collapsed instance family:

    pub const RESETS_RESET:   u32 = RESETS_BASE + 0x0;
    pub fn IO_BANK0_GPIO_CTRL(pin: u32) -> u32 { ... }

Peripherals sharing one file (several `derivedFrom` instances) emit one
`<INSTANCE>_BASE` and one absolute accessor set per instance, so an alias is
addressed directly as `<INSTANCE>_<REG>`:

    pub const PIO0_BASE:      u32 = 0x5020_0000;
    pub const PIO1_BASE:      u32 = 0x5030_0000;
    pub const PIO0_CTRL:      u32 = PIO0_BASE + 0x0;
    pub fn PIO0_INSTR_MEM(n: u32) -> u32 { ... }

Collapsing repeated addresses
-----------------------------
A run of registers whose addresses form an arithmetic progression is replaced
by a single function `base + i * stride`.  Registers are grouped by their
digit-free name; groups carrying more than one varying digit run (the DMA
channel x alias matrix, SIO interp0/interp1, the IRQ summary blocks) try each
run as the function index and keep the split that collapses the most registers,
so `CH0_AL1_CTRL..CH11_AL3_CTRL` still becomes `DMA_CH_AL1_CTRL(ch)`.  A single
run only collapses when its index takes exactly the values `0..count-1`.

The generated block is delimited so it can be regenerated idempotently, and is
written *before* the field bit-range and enumerated-value blocks.  The active
device is chosen with the SVD_DEVICE environment variable (default RP2040).

Migrating hand-curated files
----------------------------
A file that declares addresses outside this block is left untouched (so the
legacy RP2040 files were preserved).  Set SVD_RESET_MANUAL=1 to drop the old
declarations and rebuild the file from its skeleton -- this is how the RP2040
constants were converted to fully generated files.  tools/gen_constants.py
exposes it as `--reset`.
"""

import os
import re
import sys

from svd_common import (
    CONST_DIR,
    ENUM_BEGIN,
    ENUM_END,
    REG_BEGIN as BEGIN,
    REG_END as END,
    SVD_NAME,
    collect_register_offsets,
    ident,
    output_groups,
    parse_svd,
    peripheral_base,
    resolve,
    sanitize,
    split_block,
    token,
    upsert_block,
)

DIGITS = re.compile(r"\d+")
# A hand-curated declaration already sitting in the file header -- the offsets
# pass must not duplicate it.
MANUAL = re.compile(r"^\s*pub\s+(?:const|fn)\b", re.M)


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


def render(items, bases):
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


def build_block(stem, primary, aliases, registers, bases):
    """Return the generated lines for one output file."""
    body = []
    for name, addr in bases:
        body.append(f"pub const {name}_BASE:".ljust(52) + f"u32 = {fmt_base(addr)};")
    body.append("")
    body.extend(render(plan_registers(registers), bases))
    return body


def main():
    periphs = parse_svd()
    force = os.environ.get("SVD_FORCE_OFFSETS") == "1"
    # One-time migration: drop hand-curated declarations and rebuild the file
    # from the skeleton (used to convert the legacy RP2040 constant files).
    reset = os.environ.get("SVD_RESET_MANUAL") == "1"

    total = 0
    skipped = []
    for stem, primary, aliases in output_groups(periphs):
        path = os.path.join(CONST_DIR, stem + ".rs")
        exists = os.path.isfile(path)
        if not exists and not aliases:
            print(f"  ! no constants file for peripheral {primary} (skipped)", file=sys.stderr)
            continue

        peripheral = resolve(periphs, primary)
        registers = collect_register_offsets(peripheral)

        if merged_names := aliases:
            bases = [(primary, peripheral_base(periphs, primary))]
            bases += [(a, peripheral_base(periphs, a)) for a in merged_names]
        else:
            bases = [(stem, peripheral_base(periphs, primary))]

        skeleton = f"#![allow(dead_code)]\n// {stem}\n"
        if reset:
            content = skeleton
        elif exists:
            content = open(path).read()
        else:
            content = skeleton

        # Refuse to touch a file that already declares addresses outside our
        # own block and has not been migrated yet (a hand-curated RP2040 file,
        # or a legacy merged file whose offsets live in the field block).
        # Once a file carries our REG block it is ours to regenerate; the
        # driver also sets SVD_FORCE_OFFSETS=1.
        if not reset and BEGIN not in content:
            check = split_block(content, ENUM_BEGIN, ENUM_END)[0]
            if MANUAL.search(check) and not force:
                skipped.append(stem)
                continue

        body = build_block(stem, primary, aliases, registers, bases)

        block = "\n".join(
            [f"// Generated from specs/{SVD_NAME} -- do not edit by hand.", ""]
            + body
        )
        content = upsert_block(content, BEGIN, END, block.split("\n"))
        with open(path, "w") as fh:
            fh.write(content)

        n = len([l for l in body if l.startswith(("pub const", "pub fn"))])
        total += n
        print(f"  {stem:24} registers={len(registers):4}  -> {n} accessors")

    print(f"\nTotal generated accessors: {total}")
    if skipped:
        print(f"kept {len(skipped)} hand-curated file(s): {', '.join(skipped)}")


if __name__ == "__main__":
    main()
