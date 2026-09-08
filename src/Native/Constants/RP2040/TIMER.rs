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
