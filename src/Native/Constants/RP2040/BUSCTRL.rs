#![allow(dead_code)]
// BUSCTRL
pub const BUSCTRL_BASE:                             u32 = 0x4003_0000;
pub const BUSCTRL_BUS_PRIORITY:                     u32 = BUSCTRL_BASE + 0x0;
pub const BUSCTRL_BUS_PRIORITY_ACK:                 u32 = BUSCTRL_BASE + 0x4;
pub fn BUSCTRL_PERFCTR(n: u32) -> u32 {
    return BUSCTRL_BASE + 0x8 + n * 8
}

pub fn BUSCTRL_PERFSEL(n: u32) -> u32 {
    return BUSCTRL_BASE + 0xC + n * 8
}

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// BUS_PRIORITY
pub const BUSCTRL_BUS_PRIORITY_DMA_W_BIT:           u32 = 12;
pub const BUSCTRL_BUS_PRIORITY_DMA_R_BIT:           u32 = 8;
pub const BUSCTRL_BUS_PRIORITY_PROC1_BIT:           u32 = 4;
pub const BUSCTRL_BUS_PRIORITY_PROC0_BIT:           u32 = 0;
// BUS_PRIORITY_ACK
pub const BUSCTRL_BUS_PRIORITY_ACK_BIT:             u32 = 0;
// PERFCTR0
pub const BUSCTRL_PERFCTR0_LOW:                     u32 = 0;
pub const BUSCTRL_PERFCTR0_HIGH:                    u32 = 23;
// PERFSEL0
pub const BUSCTRL_PERFSEL0_LOW:                     u32 = 0;
pub const BUSCTRL_PERFSEL0_HIGH:                    u32 = 4;
// PERFCTR1
pub const BUSCTRL_PERFCTR1_LOW:                     u32 = 0;
pub const BUSCTRL_PERFCTR1_HIGH:                    u32 = 23;
// PERFSEL1
pub const BUSCTRL_PERFSEL1_LOW:                     u32 = 0;
pub const BUSCTRL_PERFSEL1_HIGH:                    u32 = 4;
// PERFCTR2
pub const BUSCTRL_PERFCTR2_LOW:                     u32 = 0;
pub const BUSCTRL_PERFCTR2_HIGH:                    u32 = 23;
// PERFSEL2
pub const BUSCTRL_PERFSEL2_LOW:                     u32 = 0;
pub const BUSCTRL_PERFSEL2_HIGH:                    u32 = 4;
// PERFCTR3
pub const BUSCTRL_PERFCTR3_LOW:                     u32 = 0;
pub const BUSCTRL_PERFCTR3_HIGH:                    u32 = 23;
// PERFSEL3
pub const BUSCTRL_PERFSEL3_LOW:                     u32 = 0;
pub const BUSCTRL_PERFSEL3_HIGH:                    u32 = 4;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// PERFSEL0..PERFSEL3: PERFSEL
pub const BUSCTRL_PERFSEL_APB_CONTESTED:            u32 = 0x0;
pub const BUSCTRL_PERFSEL_APB:                      u32 = 0x1;
pub const BUSCTRL_PERFSEL_FASTPERI_CONTESTED:       u32 = 0x2;
pub const BUSCTRL_PERFSEL_FASTPERI:                 u32 = 0x3;
pub const BUSCTRL_PERFSEL_SRAM5_CONTESTED:          u32 = 0x4;
pub const BUSCTRL_PERFSEL_SRAM5:                    u32 = 0x5;
pub const BUSCTRL_PERFSEL_SRAM4_CONTESTED:          u32 = 0x6;
pub const BUSCTRL_PERFSEL_SRAM4:                    u32 = 0x7;
pub const BUSCTRL_PERFSEL_SRAM3_CONTESTED:          u32 = 0x8;
pub const BUSCTRL_PERFSEL_SRAM3:                    u32 = 0x9;
pub const BUSCTRL_PERFSEL_SRAM2_CONTESTED:          u32 = 0xA;
pub const BUSCTRL_PERFSEL_SRAM2:                    u32 = 0xB;
pub const BUSCTRL_PERFSEL_SRAM1_CONTESTED:          u32 = 0xC;
pub const BUSCTRL_PERFSEL_SRAM1:                    u32 = 0xD;
pub const BUSCTRL_PERFSEL_SRAM0_CONTESTED:          u32 = 0xE;
pub const BUSCTRL_PERFSEL_SRAM0:                    u32 = 0xF;
pub const BUSCTRL_PERFSEL_XIP_MAIN_CONTESTED:       u32 = 0x10;
pub const BUSCTRL_PERFSEL_XIP_MAIN:                 u32 = 0x11;
pub const BUSCTRL_PERFSEL_ROM_CONTESTED:            u32 = 0x12;
pub const BUSCTRL_PERFSEL_ROM:                      u32 = 0x13;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
