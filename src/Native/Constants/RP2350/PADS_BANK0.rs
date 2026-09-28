#![allow(dead_code)]
// PADS_BANK0

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const PADS_BANK0_BASE:                          u32 = 0x4003_8000;

pub const PADS_BANK0_VOLTAGE_SELECT:                u32 = PADS_BANK0_BASE + 0x0;
// GPIO0..GPIO47
pub fn PADS_BANK0_GPIO(n: u32) -> u32 {
    return PADS_BANK0_BASE + 0x4 + n * 0x4
}

pub const PADS_BANK0_SWCLK:                         u32 = PADS_BANK0_BASE + 0xC4;
pub const PADS_BANK0_SWD:                           u32 = PADS_BANK0_BASE + 0xC8;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// VOLTAGE_SELECT
pub const PADS_BANK0_VOLTAGE_SELECT_BIT:            u32 = 0;

// GPIO0..GPIO47
pub const PADS_BANK0_GPIO_ISO_BIT:                  u32 = 8;
pub const PADS_BANK0_GPIO_OD_BIT:                   u32 = 7;
pub const PADS_BANK0_GPIO_IE_BIT:                   u32 = 6;
pub const PADS_BANK0_GPIO_DRIVE_LOW:                u32 = 4;
pub const PADS_BANK0_GPIO_DRIVE_HIGH:               u32 = 5;
pub const PADS_BANK0_GPIO_PUE_BIT:                  u32 = 3;
pub const PADS_BANK0_GPIO_PDE_BIT:                  u32 = 2;
pub const PADS_BANK0_GPIO_SCHMITT_BIT:              u32 = 1;
pub const PADS_BANK0_GPIO_SLEWFAST_BIT:             u32 = 0;

// SWCLK
pub const PADS_BANK0_SWCLK_ISO_BIT:                 u32 = 8;
pub const PADS_BANK0_SWCLK_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_SWCLK_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_SWCLK_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_SWCLK_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_SWCLK_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_SWCLK_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_SWCLK_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_SWCLK_SLEWFAST_BIT:            u32 = 0;

// SWD
pub const PADS_BANK0_SWD_ISO_BIT:                   u32 = 8;
pub const PADS_BANK0_SWD_OD_BIT:                    u32 = 7;
pub const PADS_BANK0_SWD_IE_BIT:                    u32 = 6;
pub const PADS_BANK0_SWD_DRIVE_LOW:                 u32 = 4;
pub const PADS_BANK0_SWD_DRIVE_HIGH:                u32 = 5;
pub const PADS_BANK0_SWD_PUE_BIT:                   u32 = 3;
pub const PADS_BANK0_SWD_PDE_BIT:                   u32 = 2;
pub const PADS_BANK0_SWD_SCHMITT_BIT:               u32 = 1;
pub const PADS_BANK0_SWD_SLEWFAST_BIT:              u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// VOLTAGE_SELECT: VOLTAGE_SELECT
pub const PADS_BANK0_VOLTAGE_SELECT_3V3:            u32 = 0x0;
pub const PADS_BANK0_VOLTAGE_SELECT_1V8:            u32 = 0x1;

// GPIO0..GPIO47: DRIVE
pub const PADS_BANK0_GPIO_DRIVE_2MA:                u32 = 0x0;
pub const PADS_BANK0_GPIO_DRIVE_4MA:                u32 = 0x1;
pub const PADS_BANK0_GPIO_DRIVE_8MA:                u32 = 0x2;
pub const PADS_BANK0_GPIO_DRIVE_12MA:               u32 = 0x3;

// SWCLK: DRIVE
pub const PADS_BANK0_SWCLK_DRIVE_2MA:               u32 = 0x0;
pub const PADS_BANK0_SWCLK_DRIVE_4MA:               u32 = 0x1;
pub const PADS_BANK0_SWCLK_DRIVE_8MA:               u32 = 0x2;
pub const PADS_BANK0_SWCLK_DRIVE_12MA:              u32 = 0x3;

// SWD: DRIVE
pub const PADS_BANK0_SWD_DRIVE_2MA:                 u32 = 0x0;
pub const PADS_BANK0_SWD_DRIVE_4MA:                 u32 = 0x1;
pub const PADS_BANK0_SWD_DRIVE_8MA:                 u32 = 0x2;
pub const PADS_BANK0_SWD_DRIVE_12MA:                u32 = 0x3;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
