#![allow(dead_code)]
// WATCHDOG
pub const WATCHDOG_BASE:                            u32 = 0x4005_8000;
pub const WATCHDOG_CTRL:                            u32 = WATCHDOG_BASE + 0x0;
pub const WATCHDOG_LOAD:                            u32 = WATCHDOG_BASE + 0x4;
pub const WATCHDOG_REASON:                          u32 = WATCHDOG_BASE + 0x8;
pub const WATCHDOG_SCRATCH0:                        u32 = WATCHDOG_BASE + 0xC;
pub const WATCHDOG_SCRATCH1:                        u32 = WATCHDOG_BASE + 0x10;
pub const WATCHDOG_SCRATCH2:                        u32 = WATCHDOG_BASE + 0x14;
pub const WATCHDOG_SCRATCH3:                        u32 = WATCHDOG_BASE + 0x18;
pub const WATCHDOG_SCRATCH4:                        u32 = WATCHDOG_BASE + 0x1C;
pub const WATCHDOG_SCRATCH5:                        u32 = WATCHDOG_BASE + 0x20;
pub const WATCHDOG_SCRATCH6:                        u32 = WATCHDOG_BASE + 0x24;
pub const WATCHDOG_SCRATCH7:                        u32 = WATCHDOG_BASE + 0x28;
pub const WATCHDOG_TICK:                            u32 = WATCHDOG_BASE + 0x2C;
