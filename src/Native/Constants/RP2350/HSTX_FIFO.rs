#![allow(dead_code)]
// HSTX_FIFO

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const HSTX_FIFO_BASE:                           u32 = 0x5060_0000;

pub const HSTX_FIFO_STAT:                           u32 = HSTX_FIFO_BASE + 0x0;
pub const HSTX_FIFO_FIFO:                           u32 = HSTX_FIFO_BASE + 0x4;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// STAT
pub const HSTX_FIFO_STAT_WOF_BIT:                   u32 = 10;
pub const HSTX_FIFO_STAT_EMPTY_BIT:                 u32 = 9;
pub const HSTX_FIFO_STAT_FULL_BIT:                  u32 = 8;
pub const HSTX_FIFO_STAT_LEVEL_LOW:                 u32 = 0;
pub const HSTX_FIFO_STAT_LEVEL_HIGH:                u32 = 7;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
