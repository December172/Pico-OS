#![allow(dead_code)]
// VREG_AND_CHIP_RESET
pub const VREG_AND_CHIP_RESET_BASE:                 u32 = 0x4006_4000;
pub const VREG_AND_CHIP_RESET_VREG:                 u32 = VREG_AND_CHIP_RESET_BASE + 0x0;
pub const VREG_AND_CHIP_RESET_BOD:                  u32 = VREG_AND_CHIP_RESET_BASE + 0x4;
pub const VREG_AND_CHIP_RESET_CHIP_RESET:           u32 = VREG_AND_CHIP_RESET_BASE + 0x8;
