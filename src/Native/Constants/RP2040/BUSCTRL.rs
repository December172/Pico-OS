#![allow(dead_code)]
// BUSCTRL
pub const BUSCTRL_BASE:                             u32 = 0x4003_0000;
pub const BUSCTRL_BUS_PRIORITY:                     u32 = BUSCTRL_BASE + 0x0;
pub const BUSCTRL_BUS_PRIORITY_ACK:                 u32 = BUSCTRL_BASE + 0x4;
pub const BUSCTRL_PERFCTR0:                         u32 = BUSCTRL_BASE + 0x8;
pub const BUSCTRL_PERFSEL0:                         u32 = BUSCTRL_BASE + 0xC;
pub const BUSCTRL_PERFCTR1:                         u32 = BUSCTRL_BASE + 0x10;
pub const BUSCTRL_PERFSEL1:                         u32 = BUSCTRL_BASE + 0x14;
pub const BUSCTRL_PERFCTR2:                         u32 = BUSCTRL_BASE + 0x18;
pub const BUSCTRL_PERFSEL2:                         u32 = BUSCTRL_BASE + 0x1C;
pub const BUSCTRL_PERFCTR3:                         u32 = BUSCTRL_BASE + 0x20;
pub const BUSCTRL_PERFSEL3:                         u32 = BUSCTRL_BASE + 0x24;

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
