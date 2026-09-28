#![allow(dead_code)]
// PADS_QSPI

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

pub const PADS_QSPI_BASE:                           u32 = 0x4002_0000;

pub const PADS_QSPI_VOLTAGE_SELECT:                 u32 = PADS_QSPI_BASE + 0x0;
pub const PADS_QSPI_GPIO_QSPI_SCLK:                 u32 = PADS_QSPI_BASE + 0x4;
// GPIO_QSPI_SD0..GPIO_QSPI_SD3
pub fn PADS_QSPI_GPIO_QSPI_SD(n: u32) -> u32 {
    return PADS_QSPI_BASE + 0x8 + n * 0x4
}

pub const PADS_QSPI_GPIO_QSPI_SS:                   u32 = PADS_QSPI_BASE + 0x18;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// VOLTAGE_SELECT
pub const PADS_QSPI_VOLTAGE_SELECT_BIT:             u32 = 0;

// GPIO_QSPI_SCLK, GPIO_QSPI_SD0, GPIO_QSPI_SD1, GPIO_QSPI_SD2, GPIO_QSPI_SD3, GPIO_QSPI_SS
pub const PADS_QSPI_GPIO_QSPI_OD_BIT:               u32 = 7;
pub const PADS_QSPI_GPIO_QSPI_IE_BIT:               u32 = 6;
pub const PADS_QSPI_GPIO_QSPI_DRIVE_LOW:            u32 = 4;
pub const PADS_QSPI_GPIO_QSPI_DRIVE_HIGH:           u32 = 5;
pub const PADS_QSPI_GPIO_QSPI_PUE_BIT:              u32 = 3;
pub const PADS_QSPI_GPIO_QSPI_PDE_BIT:              u32 = 2;
pub const PADS_QSPI_GPIO_QSPI_SCHMITT_BIT:          u32 = 1;
pub const PADS_QSPI_GPIO_QSPI_SLEWFAST_BIT:         u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// VOLTAGE_SELECT: VOLTAGE_SELECT
pub const PADS_QSPI_VOLTAGE_SELECT_3V3:             u32 = 0x0;
pub const PADS_QSPI_VOLTAGE_SELECT_1V8:             u32 = 0x1;

// GPIO_QSPI_SCLK..GPIO_QSPI_SS: DRIVE
pub const PADS_QSPI_GPIO_QSPI_DRIVE_2MA:            u32 = 0x0;
pub const PADS_QSPI_GPIO_QSPI_DRIVE_4MA:            u32 = 0x1;
pub const PADS_QSPI_GPIO_QSPI_DRIVE_8MA:            u32 = 0x2;
pub const PADS_QSPI_GPIO_QSPI_DRIVE_12MA:           u32 = 0x3;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
