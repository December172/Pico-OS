#![allow(dead_code)]
// BOOTRAM

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const BOOTRAM_BASE:                             u32 = 0x400E_0000;

// WRITE_ONCE0..WRITE_ONCE1
pub fn BOOTRAM_WRITE_ONCE(n: u32) -> u32 {
    return BOOTRAM_BASE + 0x800 + n * 0x4
}

pub const BOOTRAM_BOOTLOCK_STAT:                    u32 = BOOTRAM_BASE + 0x808;
// BOOTLOCK0..BOOTLOCK7
pub fn BOOTRAM_BOOTLOCK(n: u32) -> u32 {
    return BOOTRAM_BASE + 0x80C + n * 0x4
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// BOOTLOCK_STAT
pub const BOOTRAM_BOOTLOCK_STAT_LOW:                u32 = 0;
pub const BOOTRAM_BOOTLOCK_STAT_HIGH:               u32 = 7;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
