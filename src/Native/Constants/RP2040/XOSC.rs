#![allow(dead_code)]
// XOSC
pub const XOSC_BASE:                                u32 = 0x4002_4000;
pub const XOSC_CTRL:                                u32 = XOSC_BASE + 0x0;
pub const XOSC_STATUS:                              u32 = XOSC_BASE + 0x4;
pub const XOSC_DORMANT:                             u32 = XOSC_BASE + 0x8;
pub const XOSC_STARTUP:                             u32 = XOSC_BASE + 0xC;
pub const XOSC_COUNT:                               u32 = XOSC_BASE + 0x1C;
