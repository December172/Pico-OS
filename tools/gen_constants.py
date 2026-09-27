#!/usr/bin/env python3


"""Regenerate every constants file for an SVD device in one shot.

    python tools/gen_constants.py RP2350
    python tools/gen_constants.py RP2040

The driver selects the device (via SVD_DEVICE), optionally creates the missing
per-peripheral skeleton files, then runs the focused generators in order:

    1. tools/gen_register_offsets.py  -- BASE / register address accessors
    2. tools/gen_field_ranges.py      -- per-field bit ranges
    3. tools/gen_enum_values.py       -- enumerated register values

The register-address pass is skipped for RP2040 by default: those files carry
hand-curated base/offset declarations that the generator would duplicate, so
only the two block generators are re-run there.  Pass --offsets to force it or
--no-offsets to skip it for any device.

The generator blocks are delimited and replaced in place, so running the driver
twice produces identical output.  `mod.rs` is only written when it does not yet
exist (or with --write-mod), so a curated module list is never clobbered.
"""

import argparse
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("device", nargs="?", default="RP2350",
                        help="SVD device to generate (default: RP2350)")
    parser.add_argument("--offsets", dest="offsets", action="store_true", default=None,
                        help="force the register-address pass")
    parser.add_argument("--no-offsets", dest="offsets", action="store_false",
                        help="skip the register-address pass")
    parser.add_argument("--no-create", dest="create", action="store_false", default=None,
                        help="do not create missing per-peripheral files")
    parser.add_argument("--reset", action="store_true",
                        help="drop hand-curated declarations before regenerating "
                             "(one-time migration to fully generated files)")
    parser.add_argument("--write-mod", action="store_true",
                        help="rewrite mod.rs even when it already exists")
    return parser.parse_args()


def main():
    args = parse_args()
    os.environ["SVD_DEVICE"] = args.device
    sys.path.insert(0, HERE)

    # Imported after SVD_DEVICE is set: svd_common resolves the device profile
    # (SVD path, constants dir, merge map) at import time.
    from svd_common import CONST_DIR, SVD_NAME, output_groups, parse_svd

    if args.offsets is None:
        args.offsets = args.device != "RP2040"
    if args.create is None:
        # Only the generated-from-scratch devices need skeleton files; RP2040
        # already ships a curated file per peripheral.
        args.create = args.device != "RP2040"

    groups = output_groups(parse_svd())
    os.makedirs(CONST_DIR, exist_ok=True)

    if args.create:
        created = 0
        for stem, _primary, _aliases in groups:
            path = os.path.join(CONST_DIR, stem + ".rs")
            if not os.path.exists(path):
                with open(path, "w") as fh:
                    fh.write(f"#![allow(dead_code)]\n// {stem}\n")
                created += 1
        if created:
            print(f"created {created} skeleton file(s) in {CONST_DIR}")

    env = dict(os.environ)
    env["SVD_EMIT_BASES_OFFSETS"] = "1" if args.offsets is False else "0"
    if args.offsets:
        # The driver owns the register-address layout for this device, so let
        # the offsets pass replace previously generated content.
        env["SVD_FORCE_OFFSETS"] = "1"
    if args.reset:
        if not args.offsets:
            raise SystemExit("--reset requires the register-address pass (--offsets)")
        env["SVD_RESET_MANUAL"] = "1"

    tools = []
    if args.offsets:
        tools.append("gen_register_offsets.py")
    tools += ["gen_field_ranges.py", "gen_enum_values.py"]

    for tool in tools:
        print(f"\n== {tool} ({args.device}) ==")
        subprocess.run([sys.executable, os.path.join(HERE, tool)], env=env, check=True)

    mod_path = os.path.join(CONST_DIR, "mod.rs")
    if args.write_mod or not os.path.exists(mod_path):
        lines = [f"pub mod {stem};" for stem, _p, _a in groups]
        if os.path.exists(os.path.join(CONST_DIR, "Config.rs")):
            lines += ["", "// For non-predefined user values", "pub mod Config;"]
        with open(mod_path, "w") as fh:
            fh.write("\n".join(lines) + "\n")
        print(f"\nwrote {mod_path}")

    print(f"\nDone: {SVD_NAME} -> {CONST_DIR}")


if __name__ == "__main__":
    main()
