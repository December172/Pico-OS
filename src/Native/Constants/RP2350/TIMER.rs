#![allow(dead_code)]
// TIMER

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const TIMER0_BASE:                              u32 = 0x400B_0000;
pub const TIMER1_BASE:                              u32 = 0x400B_8000;

pub const TIMER_TIMEHW_OFFSET:                      u32 = 0x0;
pub const TIMER_TIMELW_OFFSET:                      u32 = 0x4;
pub const TIMER_TIMEHR_OFFSET:                      u32 = 0x8;
pub const TIMER_TIMELR_OFFSET:                      u32 = 0xC;
// ALARM0..ALARM3
pub fn TIMER_ALARM_OFFSET(n: u32) -> u32 {
    return 0x10 + n * 0x4
}

pub const TIMER_ARMED_OFFSET:                       u32 = 0x20;
pub const TIMER_TIMERAWH_OFFSET:                    u32 = 0x24;
pub const TIMER_TIMERAWL_OFFSET:                    u32 = 0x28;
pub const TIMER_DBGPAUSE_OFFSET:                    u32 = 0x2C;
pub const TIMER_PAUSE_OFFSET:                       u32 = 0x30;
pub const TIMER_LOCKED_OFFSET:                      u32 = 0x34;
pub const TIMER_SOURCE_OFFSET:                      u32 = 0x38;
pub const TIMER_INTR_OFFSET:                        u32 = 0x3C;
pub const TIMER_INTE_OFFSET:                        u32 = 0x40;
pub const TIMER_INTF_OFFSET:                        u32 = 0x44;
pub const TIMER_INTS_OFFSET:                        u32 = 0x48;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// ARMED
pub const TIMER_ARMED_LOW:                          u32 = 0;
pub const TIMER_ARMED_HIGH:                         u32 = 3;

// DBGPAUSE
pub const TIMER_DBGPAUSE_DBG1_BIT:                  u32 = 2;
pub const TIMER_DBGPAUSE_DBG0_BIT:                  u32 = 1;

// PAUSE
pub const TIMER_PAUSE_BIT:                          u32 = 0;

// LOCKED
pub const TIMER_LOCKED_BIT:                         u32 = 0;

// SOURCE
pub const TIMER_SOURCE_CLK_SYS_BIT:                 u32 = 0;

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

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// SOURCE: CLK_SYS
pub const TIMER_SOURCE_CLK_SYS_TICK:                u32 = 0x0;
pub const TIMER_SOURCE_CLK_SYS_CLK_SYS:             u32 = 0x1;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
