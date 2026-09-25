#![allow(dead_code)]
// VREG_AND_CHIP_RESET
pub const VREG_AND_CHIP_RESET_BASE:                 u32 = 0x4006_4000;
pub const VREG_AND_CHIP_RESET_VREG:                 u32 = VREG_AND_CHIP_RESET_BASE + 0x0;
pub const VREG_AND_CHIP_RESET_BOD:                  u32 = VREG_AND_CHIP_RESET_BASE + 0x4;
pub const VREG_AND_CHIP_RESET_CHIP_RESET:           u32 = VREG_AND_CHIP_RESET_BASE + 0x8;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// VREG
pub const VREG_AND_CHIP_RESET_VREG_ROK_BIT:         u32 = 12;
pub const VREG_AND_CHIP_RESET_VREG_VSEL_LOW:        u32 = 4;
pub const VREG_AND_CHIP_RESET_VREG_VSEL_HIGH:       u32 = 7;
pub const VREG_AND_CHIP_RESET_VREG_HIZ_BIT:         u32 = 1;
pub const VREG_AND_CHIP_RESET_VREG_EN_BIT:          u32 = 0;
// BOD
pub const VREG_AND_CHIP_RESET_BOD_VSEL_LOW:         u32 = 4;
pub const VREG_AND_CHIP_RESET_BOD_VSEL_HIGH:        u32 = 7;
pub const VREG_AND_CHIP_RESET_BOD_EN_BIT:           u32 = 0;
// CHIP_RESET
pub const VREG_AND_CHIP_RESET_CHIP_RESET_PSM_RESTART_FLAG_BIT:u32 = 24;
pub const VREG_AND_CHIP_RESET_CHIP_RESET_HAD_PSM_RESTART_BIT:u32 = 20;
pub const VREG_AND_CHIP_RESET_CHIP_RESET_HAD_RUN_BIT:u32 = 16;
pub const VREG_AND_CHIP_RESET_CHIP_RESET_HAD_POR_BIT:u32 = 8;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
