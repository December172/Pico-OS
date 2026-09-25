#![allow(dead_code)]
// TIMER
pub const TIMER_BASE:                               u32 = 0x4005_4000;
pub const TIMER_TIMEHW:                             u32 = TIMER_BASE + 0x0;
pub const TIMER_TIMELW:                             u32 = TIMER_BASE + 0x4;
pub const TIMER_TIMEHR:                             u32 = TIMER_BASE + 0x8;
pub const TIMER_TIMELR:                             u32 = TIMER_BASE + 0xC;
pub const TIMER_ALARM0:                             u32 = TIMER_BASE + 0x10;
pub const TIMER_ALARM1:                             u32 = TIMER_BASE + 0x14;
pub const TIMER_ALARM2:                             u32 = TIMER_BASE + 0x18;
pub const TIMER_ALARM3:                             u32 = TIMER_BASE + 0x1C;
pub const TIMER_ARMED:                              u32 = TIMER_BASE + 0x20;
pub const TIMER_TIMERAWH:                           u32 = TIMER_BASE + 0x24;
pub const TIMER_TIMERAWL:                           u32 = TIMER_BASE + 0x28;
pub const TIMER_DBGPAUSE:                           u32 = TIMER_BASE + 0x2C;
pub const TIMER_PAUSE:                              u32 = TIMER_BASE + 0x30;
pub const TIMER_INTR:                               u32 = TIMER_BASE + 0x34;
pub const TIMER_INTE:                               u32 = TIMER_BASE + 0x38;
pub const TIMER_INTF:                               u32 = TIMER_BASE + 0x3C;
pub const TIMER_INTS:                               u32 = TIMER_BASE + 0x40;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// TIMEHW
pub const TIMER_TIMEHW_LOW:                         u32 = 0;
pub const TIMER_TIMEHW_HIGH:                        u32 = 31;
// TIMELW
pub const TIMER_TIMELW_LOW:                         u32 = 0;
pub const TIMER_TIMELW_HIGH:                        u32 = 31;
// TIMEHR
pub const TIMER_TIMEHR_LOW:                         u32 = 0;
pub const TIMER_TIMEHR_HIGH:                        u32 = 31;
// TIMELR
pub const TIMER_TIMELR_LOW:                         u32 = 0;
pub const TIMER_TIMELR_HIGH:                        u32 = 31;
// ALARM0
pub const TIMER_ALARM0_LOW:                         u32 = 0;
pub const TIMER_ALARM0_HIGH:                        u32 = 31;
// ALARM1
pub const TIMER_ALARM1_LOW:                         u32 = 0;
pub const TIMER_ALARM1_HIGH:                        u32 = 31;
// ALARM2
pub const TIMER_ALARM2_LOW:                         u32 = 0;
pub const TIMER_ALARM2_HIGH:                        u32 = 31;
// ALARM3
pub const TIMER_ALARM3_LOW:                         u32 = 0;
pub const TIMER_ALARM3_HIGH:                        u32 = 31;
// ARMED
pub const TIMER_ARMED_LOW:                          u32 = 0;
pub const TIMER_ARMED_HIGH:                         u32 = 3;
// TIMERAWH
pub const TIMER_TIMERAWH_LOW:                       u32 = 0;
pub const TIMER_TIMERAWH_HIGH:                      u32 = 31;
// TIMERAWL
pub const TIMER_TIMERAWL_LOW:                       u32 = 0;
pub const TIMER_TIMERAWL_HIGH:                      u32 = 31;
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
