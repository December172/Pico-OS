#!/usr/bin/env python3

import argparse
import math
import sys

# Fixed hardware parameters
fbdiv_range = range(16, 320 + 1)
postdiv_range = range(1, 7 + 1)
refdiv_min = 1
refdiv_max = 63


def validRefdiv(string):
    if ((int(string) < refdiv_min) or (int(string) > refdiv_max)):
        raise argparse.ArgumentTypeError("REFDIV must be in the range {} to {}".format(refdiv_min, refdiv_max))
    return int(string)


def positive_frequency(string):
    value = float(string)
    if not math.isfinite(value) or value <= 0:
        raise argparse.ArgumentTypeError("frequency must be finite and greater than zero")
    return value

parser = argparse.ArgumentParser(description="PLL parameter calculator")
parser.add_argument("--input", "-i", default=12, help="Input (reference) frequency. Default 12 MHz", type=positive_frequency)
parser.add_argument("--ref-min", default=5, help="Override minimum reference frequency. Default 5 MHz", type=positive_frequency)
parser.add_argument("--vco-max", default=1600, help="Override maximum VCO frequency. Default 1600 MHz", type=positive_frequency)
parser.add_argument("--vco-min", default=750, help="Override minimum VCO frequency. Default 750 MHz", type=positive_frequency)
parser.add_argument("--lock-refdiv", help="Lock REFDIV to specified number in the range {} to {}".format(refdiv_min, refdiv_max), type=validRefdiv)
parser.add_argument("--low-vco", "-l", action="store_true", help="Use a lower VCO frequency when possible. This reduces power consumption, at the cost of increased jitter")
parser.add_argument("output", help="Output frequency in MHz.", type=positive_frequency)
args = parser.parse_args()

if args.vco_min > args.vco_max:
    parser.error("--vco-min must not exceed --vco-max")
if args.input < args.ref_min:
    parser.error("input frequency is below --ref-min")
refdiv_range = range(refdiv_min, min(refdiv_max, int(args.input / args.ref_min)) + 1)
if args.lock_refdiv:
    if args.input / args.lock_refdiv < args.ref_min:
        parser.error("locked REFDIV would put the reference below --ref-min")
    print("Locking REFDIV to", args.lock_refdiv)
    refdiv_range = [args.lock_refdiv]

best = (0, 0, 0, 0, 0, 0)
best_margin = math.inf

for refdiv in refdiv_range:
    for fbdiv in fbdiv_range:
        vco = args.input / refdiv * fbdiv
        if vco < args.vco_min or vco > args.vco_max:
            continue
        # Prefer larger pd1:pd2 ratios; reversed pairs have the same output.
        for pd2 in postdiv_range:
            for pd1 in range(pd2, postdiv_range.stop):
                out = vco / pd1 / pd2
                margin = abs(out - args.output)
                vco_is_better = vco < best[5] if args.low_vco else vco > best[5]
                if ((vco * 1000) % (pd1 * pd2)):
                    continue
                if margin < best_margin or (abs(margin - best_margin) < 1e-9 and vco_is_better):
                    best = (out, fbdiv, pd1, pd2, refdiv, vco)
                    best_margin = margin

best_out, best_fbdiv, best_pd1, best_pd2, best_refdiv, best_vco = best

if best_out <= 0:
    sys.exit("No solution found")

print("Requested: {} MHz".format(args.output))
print("Achieved:  {} MHz".format(best_out))
print("REFDIV:    {}".format(best_refdiv))
print("FBDIV:     {} (VCO = {} MHz)".format(best_fbdiv, best_vco))
print("PD1:       {}".format(best_pd1))
print("PD2:       {}".format(best_pd2))
