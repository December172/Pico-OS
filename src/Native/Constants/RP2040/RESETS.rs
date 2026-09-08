#![allow(dead_code)]
// RESETS
pub const RESETS_BASE:                              u32 = 0x4000_C000;
pub const RESETS_RESET:                             u32 = RESETS_BASE + 0x0;
pub const RESETS_WDSEL:                             u32 = RESETS_BASE + 0x4;
pub const RESETS_RESET_DONE:                        u32 = RESETS_BASE + 0x8;
