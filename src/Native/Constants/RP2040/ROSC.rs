#![allow(dead_code)]
// ROSC
pub const ROSC_BASE:                                u32 = 0x4006_0000;
pub const ROSC_CTRL:                                u32 = ROSC_BASE + 0x0;
pub const ROSC_FREQA:                               u32 = ROSC_BASE + 0x4;
pub const ROSC_FREQB:                               u32 = ROSC_BASE + 0x8;
pub const ROSC_DORMANT:                             u32 = ROSC_BASE + 0xC;
pub const ROSC_DIV:                                 u32 = ROSC_BASE + 0x10;
pub const ROSC_PHASE:                               u32 = ROSC_BASE + 0x14;
pub const ROSC_STATUS:                              u32 = ROSC_BASE + 0x18;
pub const ROSC_RANDOMBIT:                           u32 = ROSC_BASE + 0x1C;
pub const ROSC_COUNT:                               u32 = ROSC_BASE + 0x20;
