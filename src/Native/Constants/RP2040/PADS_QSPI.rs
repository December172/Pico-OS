#![allow(dead_code)]
// PADS_QSPI
pub const PADS_QSPI_BASE:                           u32 = 0x4002_0000;
pub const PADS_QSPI_VOLTAGE_SELECT:                 u32 = PADS_QSPI_BASE + 0x0;
pub const PADS_QSPI_GPIO_QSPI_SCLK:                 u32 = PADS_QSPI_BASE + 0x4;
pub const PADS_QSPI_GPIO_QSPI_SD0:                  u32 = PADS_QSPI_BASE + 0x8;
pub const PADS_QSPI_GPIO_QSPI_SD1:                  u32 = PADS_QSPI_BASE + 0xC;
pub const PADS_QSPI_GPIO_QSPI_SD2:                  u32 = PADS_QSPI_BASE + 0x10;
pub const PADS_QSPI_GPIO_QSPI_SD3:                  u32 = PADS_QSPI_BASE + 0x14;
pub const PADS_QSPI_GPIO_QSPI_SS:                   u32 = PADS_QSPI_BASE + 0x18;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// VOLTAGE_SELECT
pub const PADS_QSPI_VOLTAGE_SELECT_BIT:             u32 = 0;
// GPIO_QSPI_SCLK
pub const PADS_QSPI_GPIO_QSPI_SCLK_OD_BIT:          u32 = 7;
pub const PADS_QSPI_GPIO_QSPI_SCLK_IE_BIT:          u32 = 6;
pub const PADS_QSPI_GPIO_QSPI_SCLK_DRIVE_LOW:       u32 = 4;
pub const PADS_QSPI_GPIO_QSPI_SCLK_DRIVE_HIGH:      u32 = 5;
pub const PADS_QSPI_GPIO_QSPI_SCLK_PUE_BIT:         u32 = 3;
pub const PADS_QSPI_GPIO_QSPI_SCLK_PDE_BIT:         u32 = 2;
pub const PADS_QSPI_GPIO_QSPI_SCLK_SCHMITT_BIT:     u32 = 1;
pub const PADS_QSPI_GPIO_QSPI_SCLK_SLEWFAST_BIT:    u32 = 0;
// GPIO_QSPI_SD0
pub const PADS_QSPI_GPIO_QSPI_SD0_OD_BIT:           u32 = 7;
pub const PADS_QSPI_GPIO_QSPI_SD0_IE_BIT:           u32 = 6;
pub const PADS_QSPI_GPIO_QSPI_SD0_DRIVE_LOW:        u32 = 4;
pub const PADS_QSPI_GPIO_QSPI_SD0_DRIVE_HIGH:       u32 = 5;
pub const PADS_QSPI_GPIO_QSPI_SD0_PUE_BIT:          u32 = 3;
pub const PADS_QSPI_GPIO_QSPI_SD0_PDE_BIT:          u32 = 2;
pub const PADS_QSPI_GPIO_QSPI_SD0_SCHMITT_BIT:      u32 = 1;
pub const PADS_QSPI_GPIO_QSPI_SD0_SLEWFAST_BIT:     u32 = 0;
// GPIO_QSPI_SD1
pub const PADS_QSPI_GPIO_QSPI_SD1_OD_BIT:           u32 = 7;
pub const PADS_QSPI_GPIO_QSPI_SD1_IE_BIT:           u32 = 6;
pub const PADS_QSPI_GPIO_QSPI_SD1_DRIVE_LOW:        u32 = 4;
pub const PADS_QSPI_GPIO_QSPI_SD1_DRIVE_HIGH:       u32 = 5;
pub const PADS_QSPI_GPIO_QSPI_SD1_PUE_BIT:          u32 = 3;
pub const PADS_QSPI_GPIO_QSPI_SD1_PDE_BIT:          u32 = 2;
pub const PADS_QSPI_GPIO_QSPI_SD1_SCHMITT_BIT:      u32 = 1;
pub const PADS_QSPI_GPIO_QSPI_SD1_SLEWFAST_BIT:     u32 = 0;
// GPIO_QSPI_SD2
pub const PADS_QSPI_GPIO_QSPI_SD2_OD_BIT:           u32 = 7;
pub const PADS_QSPI_GPIO_QSPI_SD2_IE_BIT:           u32 = 6;
pub const PADS_QSPI_GPIO_QSPI_SD2_DRIVE_LOW:        u32 = 4;
pub const PADS_QSPI_GPIO_QSPI_SD2_DRIVE_HIGH:       u32 = 5;
pub const PADS_QSPI_GPIO_QSPI_SD2_PUE_BIT:          u32 = 3;
pub const PADS_QSPI_GPIO_QSPI_SD2_PDE_BIT:          u32 = 2;
pub const PADS_QSPI_GPIO_QSPI_SD2_SCHMITT_BIT:      u32 = 1;
pub const PADS_QSPI_GPIO_QSPI_SD2_SLEWFAST_BIT:     u32 = 0;
// GPIO_QSPI_SD3
pub const PADS_QSPI_GPIO_QSPI_SD3_OD_BIT:           u32 = 7;
pub const PADS_QSPI_GPIO_QSPI_SD3_IE_BIT:           u32 = 6;
pub const PADS_QSPI_GPIO_QSPI_SD3_DRIVE_LOW:        u32 = 4;
pub const PADS_QSPI_GPIO_QSPI_SD3_DRIVE_HIGH:       u32 = 5;
pub const PADS_QSPI_GPIO_QSPI_SD3_PUE_BIT:          u32 = 3;
pub const PADS_QSPI_GPIO_QSPI_SD3_PDE_BIT:          u32 = 2;
pub const PADS_QSPI_GPIO_QSPI_SD3_SCHMITT_BIT:      u32 = 1;
pub const PADS_QSPI_GPIO_QSPI_SD3_SLEWFAST_BIT:     u32 = 0;
// GPIO_QSPI_SS
pub const PADS_QSPI_GPIO_QSPI_SS_OD_BIT:            u32 = 7;
pub const PADS_QSPI_GPIO_QSPI_SS_IE_BIT:            u32 = 6;
pub const PADS_QSPI_GPIO_QSPI_SS_DRIVE_LOW:         u32 = 4;
pub const PADS_QSPI_GPIO_QSPI_SS_DRIVE_HIGH:        u32 = 5;
pub const PADS_QSPI_GPIO_QSPI_SS_PUE_BIT:           u32 = 3;
pub const PADS_QSPI_GPIO_QSPI_SS_PDE_BIT:           u32 = 2;
pub const PADS_QSPI_GPIO_QSPI_SS_SCHMITT_BIT:       u32 = 1;
pub const PADS_QSPI_GPIO_QSPI_SS_SLEWFAST_BIT:      u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// VOLTAGE_SELECT: VOLTAGE_SELECT
pub const PADS_QSPI_VOLTAGE_SELECT_3V3:             u32 = 0x0;
pub const PADS_QSPI_VOLTAGE_SELECT_1V8:             u32 = 0x1;

// GPIO_QSPI_SCLK: DRIVE
pub const PADS_QSPI_GPIO_QSPI_SCLK_DRIVE_2MA:       u32 = 0x0;
pub const PADS_QSPI_GPIO_QSPI_SCLK_DRIVE_4MA:       u32 = 0x1;
pub const PADS_QSPI_GPIO_QSPI_SCLK_DRIVE_8MA:       u32 = 0x2;
pub const PADS_QSPI_GPIO_QSPI_SCLK_DRIVE_12MA:      u32 = 0x3;

// GPIO_QSPI_SD0..GPIO_QSPI_SD3: DRIVE
pub const PADS_QSPI_GPIO_QSPI_SD_DRIVE_2MA:         u32 = 0x0;
pub const PADS_QSPI_GPIO_QSPI_SD_DRIVE_4MA:         u32 = 0x1;
pub const PADS_QSPI_GPIO_QSPI_SD_DRIVE_8MA:         u32 = 0x2;
pub const PADS_QSPI_GPIO_QSPI_SD_DRIVE_12MA:        u32 = 0x3;

// GPIO_QSPI_SS: DRIVE
pub const PADS_QSPI_GPIO_QSPI_SS_DRIVE_2MA:         u32 = 0x0;
pub const PADS_QSPI_GPIO_QSPI_SS_DRIVE_4MA:         u32 = 0x1;
pub const PADS_QSPI_GPIO_QSPI_SS_DRIVE_8MA:         u32 = 0x2;
pub const PADS_QSPI_GPIO_QSPI_SS_DRIVE_12MA:        u32 = 0x3;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
