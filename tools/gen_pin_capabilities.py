#!/usr/bin/env python3


"""Generate the <Peripheral>/PinCapabilities.rs pin-mux tables from the SVD.

    python tools/gen_pin_capabilities.py RP2040
    python tools/gen_pin_capabilities.py RP2350 --write

Where the data comes from
-------------------------
The SVD carries the datasheet "GPIO function table" as the enumerated values
of the FUNCSEL field of the GPIO control register (IO_BANK0 GPIO<n>_CTRL on
both RP2040 and RP2350), for example GPIO0/F2 -> `uart0_tx` and GPIO1/F2 ->
`uart0_rx`.  Reading that table gives, for every pin, which peripheral signal
it can drive -- instance included (`uart0` vs `uart1`, pwm slice number ...).

Nothing here is device specific: the generator locates the IO block by looking
for a `GPIO<n>_CTRL` register with a FUNCSEL field, so another chip exposing
the same layout works without editing this file.  Signal names are matched
against the `FAMILIES` table below; a name that matches nothing (JTAG, PIO,
clocks, USB muxing, USB PHY, HSTX, coreSight, ...) is not part of
HAL::PinFunction and is reported as unmapped instead of being emitted.

This tool removes the need to hand-write the tables, but it is intentionally
side-effect free by default: with no mode flag it prints what it *would*
write and touches nothing.  `--write` persists the files.

Output
------
One file per peripheral folder, matching the existing layout:

    src/Native/Drivers/<DEVICE>/<Peripheral>/PinCapabilities.rs

A file is only replaced when it already carries the AUTO-GENERATED marker this
tool writes; a hand-written (or work-in-progress) file is left alone unless
`--force` is given, so running the tool can never clobber curated code.
"""

import argparse
import os
import re
import sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)

# Signal name -> HAL::PinFunction.  Each family owns one FUNCSEL (the F number
# in the datasheet table) and is emitted into its own peripheral folder.
#
#   pattern : how the SVD signal name is split; the "sig" group picks the
#             variant out of `signals`, the "blk" group is the instance index.
#   block   : peripherialBlock for a match (uart0/uart1 -> 0/1, pwm slice ...).
#
# `signals` maps the captured "sig" token to a variant; GPIO has no signal
# token (its pattern only captures the pin) so it maps the empty token.
FAMILIES = (
    {
        "name": "GPIO",
        "folder": "GPIO",
        "label": "SIO",
        "pattern": re.compile(r"^(?:sio|siob_proc)_(?P<pin>\d+)$"),
        "signals": {"": "GPIO"},
        "block": lambda m: 0,
        "block_prefix": None,
    },
    {
        "name": "UART",
        "folder": "UART",
        "label": "UART",
        "pattern": re.compile(r"^uart(?P<blk>\d+)_(?P<sig>tx|rx|cts|rts)$"),
        "signals": {"tx": "UART_TX", "rx": "UART_RX",
                    "cts": "UART_CTS", "rts": "UART_RTS"},
        "block": lambda m: int(m.group("blk")),
        "block_prefix": "uart",
    },
    {
        "name": "SPI",
        "folder": "SPI",
        "label": "SPI",
        "pattern": re.compile(r"^spi(?P<blk>\d+)_(?P<sig>tx|rx|sclk|ss_n)$"),
        "signals": {"tx": "SPI_TX", "rx": "SPI_RX",
                    "sclk": "SPI_SCK", "ss_n": "SPI_CS"},
        "block": lambda m: int(m.group("blk")),
        "block_prefix": "spi",
    },
    {
        "name": "I2C",
        "folder": "I2C",
        "label": "I2C",
        "pattern": re.compile(r"^i2c(?P<blk>\d+)_(?P<sig>sda|scl)$"),
        "signals": {"sda": "I2C_SDA", "scl": "I2C_SCL"},
        "block": lambda m: int(m.group("blk")),
        "block_prefix": "i2c",
    },
    {
        "name": "PWM",
        "folder": "PWM",
        "label": "PWM",
        "pattern": re.compile(r"^pwm_(?P<sig>a|b)_(?P<blk>\d+)$"),
        "signals": {"a": "PWM_A", "b": "PWM_B"},
        "block": lambda m: int(m.group("blk")),
        "block_prefix": None,
    },
)

GPIO_CTRL = re.compile(r"^GPIO(\d+)_CTRL$")
MARKER = "AUTO-GENERATED"
DIGITS = re.compile(r"\d+")


def parse_args():
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("device", nargs="?", default=None,
                        help="SVD device to generate (default: $SVD_DEVICE or RP2040)")
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--write", action="store_true",
                       help="write the generated files (default is a dry run)")
    group.add_argument("--check", action="store_true",
                       help="fail if a generated file is missing or out of date")
    parser.add_argument("--force", action="store_true",
                        help="with --write, also replace files without the "
                             "AUTO-GENERATED marker (hand-written code)")
    return parser.parse_args()


# ==== SVD reading =====================================================

def find_io_bank(periphs):
    """Return (peripheral_name, element) of the block holding GPIO<n>_CTRL."""
    for name, periph in periphs.items():
        for reg in periph.findall("./registers/register"):
            if GPIO_CTRL.fullmatch(reg.findtext("name") or ""):
                return name, periph
    raise SystemExit("no GPIO<n>_CTRL register found -- is this an RP2040/RP2350 SVD?")


def collect_pin_mux(periphs):
    """Return {pin: {funcsel: signal_name}} for every GPIO control register."""
    io_name, io = find_io_bank(periphs)
    pins = {}
    for reg in io.findall("./registers/register"):
        match = GPIO_CTRL.fullmatch(reg.findtext("name") or "")
        if not match:
            continue
        field = [f for f in reg.findall("./fields/field")
                 if f.findtext("name") == "FUNCSEL"]
        if not field:
            continue
        values = {}
        for enum in field[0].findall("./enumeratedValues/enumeratedValue"):
            name = enum.findtext("name")
            value = enum.findtext("value")
            if name is not None and value is not None:
                values[int(value, 0)] = name
        pins[int(match.group(1))] = values
    if not pins:
        raise SystemExit("GPIO<n>_CTRL registers carry no FUNCSEL table")
    return io_name, pins


def match_signal(name):
    """Return (family, variant, block) for an SVD signal name, or None."""
    for family in FAMILIES:
        match = family["pattern"].fullmatch(name)
        if not match:
            continue
        token = match.groupdict().get("sig") or ""
        variant = family["signals"].get(token)
        if variant:
            return family, variant, family["block"](match)
    return None


def collect_capabilities(pins):
    """Return {family_name: {(pin, variant, block): funcsel}}.

    A pin can expose the same peripheral twice (RP2350 puts some UART signals
    on both F2 and F11); the lowest function select wins so a row is unique.
    """
    tables = {family["name"]: {} for family in FAMILIES}
    for pin in sorted(pins):
        for funcsel, name in sorted(pins[pin].items()):
            hit = match_signal(name)
            if hit is None:
                continue
            family, variant, block = hit
            tables[family["name"]].setdefault((pin, variant, block), funcsel)
    return tables


def hal_variants():
    """Return the PinFunction variant names declared in src/HAL/Pin.rs."""
    path = os.path.join(ROOT, "src", "HAL", "Pin.rs")
    text = open(path).read()
    body = re.search(r"enum\s+PinFunction\s*\{(.*?)\}", text, re.S)
    if not body:
        raise SystemExit(f"cannot find enum PinFunction in {path}")
    return [v.strip() for v in body.group(1).split(",") if v.strip()]


# ==== rendering =======================================================

def block_note(family, rows):
    """Comment line describing the peripherialBlock column."""
    blocks = sorted({block for _pin, _variant, block in rows})
    pins = [pin for pin, _variant, _block in rows]
    if family["name"] == "GPIO":
        return (f"// peripherialBlock: 0 = SIO; GPIO{min(pins)}..GPIO{max(pins)} can all "
                f"be used as plain software GPIO.")
    if family["block_prefix"] is None:
        return f"// peripherialBlock: {blocks[0]}..{blocks[-1]} = {family['name']} slice index."
    listed = ", ".join(f"{b} = {family['block_prefix']}{b}" for b in blocks)
    return f"// peripherialBlock: {listed}."


def render(device, family, rows, svd_name, io_name):
    """Return the PinCapabilities.rs text for one peripheral."""
    selects = sorted({funcsel for funcsel in rows.values()})
    funcsels = "/".join(f"F{s}" for s in selects)
    # Variants are padded to the longest name so the table stays column aligned
    # (the same layout the hand-written files used).
    width = max(len(f"PinFunction::{variant},") for _p, variant, _b in rows) + 1

    lines = [
        f"// Pin capabilities of the {device} {family['name']} driver.",
        f"// {MARKER} by tools/gen_pin_capabilities.py -- do not edit by hand.",
        f"// Source: specs/{svd_name}, {io_name} GPIO<n>_CTRL.FUNCSEL {funcsels} = "
        f"{family['label']}",
        "//         (datasheet \"GPIO function table\").",
        block_note(family, rows),
        "",
        "use crate::HAL::Pin::PinCapability;",
        "use crate::HAL::Pin::PinFunction;",
        "",
        "pub static PIN_CAPABILITIES: &[PinCapability] = &[",
    ]
    for row in sorted(rows.items(), key=lambda kv: (kv[0][0], kv[1], kv[0][1])):
        (pin, variant, block), _funcsel = row
        entry = f"PinFunction::{variant},"
        lines.append(f"    PinCapability {{ pin: {pin:>2}, function: {entry:<{width}}"
                     f"peripherialBlock: {block} }},")
    lines.append("];")
    return "\n".join(lines) + "\n"


# ==== output ==========================================================

def target_path(device, family):
    return os.path.join(ROOT, "src", "Native", "Drivers", device,
                        family["folder"], "PinCapabilities.rs")


def emit(path, current, text, args):
    """Print, compare or write one file; return False when --check finds a problem.

    A peripheral whose driver folder does not exist yet is never an error: there
    is no code to feed the table to.  A missing file inside an existing folder
    (or a stale one) is what --check reports.
    """
    folder_exists = os.path.isdir(os.path.dirname(path))

    if args.check:
        if not folder_exists:
            print(f"  - skipped   {path} (peripheral folder does not exist)")
            return True
        if current is None:
            print(f"  ! missing   {path}", file=sys.stderr)
            return False
        if MARKER not in current:
            print(f"  - skipped   {path} (no {MARKER} marker)")
            return True
        if current != text:
            print(f"  ! stale     {path}", file=sys.stderr)
            return False
        print(f"  = up to date {path}")
        return True

    if not args.write:
        print(f"\n==== {os.path.relpath(path, ROOT)} ====")
        print(text, end="")
        return True

    if current is not None and MARKER not in current and not args.force:
        print(f"  - skipped   {path} (no {MARKER} marker; use --force to replace)")
        return True
    if not folder_exists:
        print(f"  - skipped   {path} (peripheral folder does not exist)")
        return True
    with open(path, "w") as fh:
        fh.write(text)
    print(f"  wrote       {path}")
    return True


def main():
    args = parse_args()
    os.environ["SVD_DEVICE"] = args.device or os.environ.get("SVD_DEVICE", "RP2040")
    if args.device is None:
        args.device = os.environ["SVD_DEVICE"]
    sys.path.insert(0, HERE)

    # Imported after SVD_DEVICE is set: svd_common resolves the device profile
    # (SVD path, ...) at import time.
    from svd_common import SVD_NAME, parse_svd

    device = args.device
    io_name, pins = collect_pin_mux(parse_svd())
    tables = collect_capabilities(pins)
    known = hal_variants()

    print(f"{device}: {io_name} GPIO{min(pins)}..GPIO{max(pins)} "
          f"({len(pins)} pins) <- specs/{SVD_NAME}")

    unmapped = Counter()
    ok = True
    empty = []
    for family in FAMILIES:
        rows = tables[family["name"]]
        if not rows:
            empty.append(family["name"])
            continue
        for variant in {v for _p, v, _b in rows}:
            if variant not in known:
                raise SystemExit(f"{family['name']}: PinFunction::{variant} is not "
                                 f"declared in src/HAL/Pin.rs")
        text = render(device, family, rows, SVD_NAME, io_name)
        path = target_path(device, family)
        current = open(path).read() if os.path.isfile(path) else None
        print(f"  {family['name']:4} {len(rows):3} entries")
        ok &= emit(path, current, text, args)

    # Signals outside HAL::PinFunction (JTAG, PIO, clocks, USB muxing, ...).
    for pin in pins.values():
        for name in pin.values():
            if match_signal(name) is None:
                unmapped[DIGITS.sub("", name)] += 1
    if unmapped:
        listed = ", ".join(f"{name or '?'} x{count}" for name, count in
                           sorted(unmapped.items()))
        print(f"unmapped (not in HAL::PinFunction): {listed}", file=sys.stderr)
    if empty:
        print(f"no pin capabilities for: {', '.join(empty)}")

    if not args.write and not args.check:
        print("\nDry run -- nothing written. Re-run with --write to update the files.")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
