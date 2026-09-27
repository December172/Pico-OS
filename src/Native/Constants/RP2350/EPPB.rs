#![allow(dead_code)]
// EPPB

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const EPPB_BASE:                                u32 = 0xE008_0000;

// NMI_MASK0..NMI_MASK1
pub fn EPPB_NMI_MASK(n: u32) -> u32 {
    return EPPB_BASE + 0x0 + n * 0x4
}

pub const EPPB_SLEEPCTRL:                           u32 = EPPB_BASE + 0x8;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// NMI_MASK1
pub const EPPB_NMI_MASK1_LOW:                       u32 = 0;
pub const EPPB_NMI_MASK1_HIGH:                      u32 = 19;

// SLEEPCTRL
pub const EPPB_SLEEPCTRL_WICENACK_BIT:              u32 = 2;
pub const EPPB_SLEEPCTRL_WICENREQ_BIT:              u32 = 1;
pub const EPPB_SLEEPCTRL_LIGHT_SLEEP_BIT:           u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
