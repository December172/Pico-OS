#![allow(dead_code)]
// PSM
pub const PSM_BASE:                                 u32 = 0x4001_0000;
pub const PSM_FRCE_ON:                              u32 = PSM_BASE + 0x0;
pub const PSM_FRCE_OFF:                             u32 = PSM_BASE + 0x4;
pub const PSM_WDSEL:                                u32 = PSM_BASE + 0x8;
pub const PSM_DONE:                                 u32 = PSM_BASE + 0xC;
