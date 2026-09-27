#![allow(dead_code)]
// TIMER

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

pub const TIMER_BASE:                               u32 = 0x4005_4000;

pub const TIMER_TIMEHW:                             u32 = TIMER_BASE + 0x0;
pub const TIMER_TIMELW:                             u32 = TIMER_BASE + 0x4;
pub const TIMER_TIMEHR:                             u32 = TIMER_BASE + 0x8;
pub const TIMER_TIMELR:                             u32 = TIMER_BASE + 0xC;
// ALARM0..ALARM3
pub fn TIMER_ALARM(n: u32) -> u32 {
    return TIMER_BASE + 0x10 + n * 0x4
}

pub const TIMER_ARMED:                              u32 = TIMER_BASE + 0x20;
pub const TIMER_TIMERAWH:                           u32 = TIMER_BASE + 0x24;
pub const TIMER_TIMERAWL:                           u32 = TIMER_BASE + 0x28;
pub const TIMER_DBGPAUSE:                           u32 = TIMER_BASE + 0x2C;
pub const TIMER_PAUSE:                              u32 = TIMER_BASE + 0x30;
pub const TIMER_INTR:                               u32 = TIMER_BASE + 0x34;
pub const TIMER_INTE:                               u32 = TIMER_BASE + 0x38;
pub const TIMER_INTF:                               u32 = TIMER_BASE + 0x3C;
pub const TIMER_INTS:                               u32 = TIMER_BASE + 0x40;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// ARMED
pub const TIMER_ARMED_LOW:                          u32 = 0;
pub const TIMER_ARMED_HIGH:                         u32 = 3;

// DBGPAUSE
pub const TIMER_DBGPAUSE_DBG1_BIT:                  u32 = 2;
pub const TIMER_DBGPAUSE_DBG0_BIT:                  u32 = 1;

// PAUSE
pub const TIMER_PAUSE_BIT:                          u32 = 0;

// INTR
pub const TIMER_INTR_ALARM_3_BIT:                   u32 = 3;
pub const TIMER_INTR_ALARM_2_BIT:                   u32 = 2;
pub const TIMER_INTR_ALARM_1_BIT:                   u32 = 1;
pub const TIMER_INTR_ALARM_0_BIT:                   u32 = 0;

// INTE
pub const TIMER_INTE_ALARM_3_BIT:                   u32 = 3;
pub const TIMER_INTE_ALARM_2_BIT:                   u32 = 2;
pub const TIMER_INTE_ALARM_1_BIT:                   u32 = 1;
pub const TIMER_INTE_ALARM_0_BIT:                   u32 = 0;

// INTF
pub const TIMER_INTF_ALARM_3_BIT:                   u32 = 3;
pub const TIMER_INTF_ALARM_2_BIT:                   u32 = 2;
pub const TIMER_INTF_ALARM_1_BIT:                   u32 = 1;
pub const TIMER_INTF_ALARM_0_BIT:                   u32 = 0;

// INTS
pub const TIMER_INTS_ALARM_3_BIT:                   u32 = 3;
pub const TIMER_INTS_ALARM_2_BIT:                   u32 = 2;
pub const TIMER_INTS_ALARM_1_BIT:                   u32 = 1;
pub const TIMER_INTS_ALARM_0_BIT:                   u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
