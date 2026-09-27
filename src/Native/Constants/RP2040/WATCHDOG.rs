#![allow(dead_code)]
// WATCHDOG

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

pub const WATCHDOG_BASE:                            u32 = 0x4005_8000;

pub const WATCHDOG_CTRL:                            u32 = WATCHDOG_BASE + 0x0;
pub const WATCHDOG_LOAD:                            u32 = WATCHDOG_BASE + 0x4;
pub const WATCHDOG_REASON:                          u32 = WATCHDOG_BASE + 0x8;
// SCRATCH0..SCRATCH7
pub fn WATCHDOG_SCRATCH(n: u32) -> u32 {
    return WATCHDOG_BASE + 0xC + n * 0x4
}

pub const WATCHDOG_TICK:                            u32 = WATCHDOG_BASE + 0x2C;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CTRL
pub const WATCHDOG_CTRL_TRIGGER_BIT:                u32 = 31;
pub const WATCHDOG_CTRL_ENABLE_BIT:                 u32 = 30;
pub const WATCHDOG_CTRL_PAUSE_DBG1_BIT:             u32 = 26;
pub const WATCHDOG_CTRL_PAUSE_DBG0_BIT:             u32 = 25;
pub const WATCHDOG_CTRL_PAUSE_JTAG_BIT:             u32 = 24;
pub const WATCHDOG_CTRL_TIME_LOW:                   u32 = 0;
pub const WATCHDOG_CTRL_TIME_HIGH:                  u32 = 23;

// LOAD
pub const WATCHDOG_LOAD_LOW:                        u32 = 0;
pub const WATCHDOG_LOAD_HIGH:                       u32 = 23;

// REASON
pub const WATCHDOG_REASON_FORCE_BIT:                u32 = 1;
pub const WATCHDOG_REASON_TIMER_BIT:                u32 = 0;

// TICK
pub const WATCHDOG_TICK_COUNT_LOW:                  u32 = 11;
pub const WATCHDOG_TICK_COUNT_HIGH:                 u32 = 19;
pub const WATCHDOG_TICK_RUNNING_BIT:                u32 = 10;
pub const WATCHDOG_TICK_ENABLE_BIT:                 u32 = 9;
pub const WATCHDOG_TICK_CYCLES_LOW:                 u32 = 0;
pub const WATCHDOG_TICK_CYCLES_HIGH:                u32 = 8;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
