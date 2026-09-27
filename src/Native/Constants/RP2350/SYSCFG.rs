#![allow(dead_code)]
// SYSCFG

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const SYSCFG_BASE:                              u32 = 0x4000_8000;

pub const SYSCFG_PROC_CONFIG:                       u32 = SYSCFG_BASE + 0x0;
pub const SYSCFG_PROC_IN_SYNC_BYPASS:               u32 = SYSCFG_BASE + 0x4;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI:            u32 = SYSCFG_BASE + 0x8;
pub const SYSCFG_DBGFORCE:                          u32 = SYSCFG_BASE + 0xC;
pub const SYSCFG_MEMPOWERDOWN:                      u32 = SYSCFG_BASE + 0x10;
pub const SYSCFG_AUXCTRL:                           u32 = SYSCFG_BASE + 0x14;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// PROC_CONFIG
pub const SYSCFG_PROC_CONFIG_PROC1_HALTED_BIT:      u32 = 1;
pub const SYSCFG_PROC_CONFIG_PROC0_HALTED_BIT:      u32 = 0;

// PROC_IN_SYNC_BYPASS_HI
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_QSPI_SD_LOW:u32 = 28;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_QSPI_SD_HIGH:u32 = 31;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_QSPI_CSN_BIT:u32 = 27;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_QSPI_SCK_BIT:u32 = 26;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_USB_DM_BIT: u32 = 25;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_USB_DP_BIT: u32 = 24;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_GPIO_LOW:   u32 = 0;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_GPIO_HIGH:  u32 = 15;

// DBGFORCE
pub const SYSCFG_DBGFORCE_ATTACH_BIT:               u32 = 3;
pub const SYSCFG_DBGFORCE_SWCLK_BIT:                u32 = 2;
pub const SYSCFG_DBGFORCE_SWDI_BIT:                 u32 = 1;
pub const SYSCFG_DBGFORCE_SWDO_BIT:                 u32 = 0;

// MEMPOWERDOWN
pub const SYSCFG_MEMPOWERDOWN_BOOTRAM_BIT:          u32 = 12;
pub const SYSCFG_MEMPOWERDOWN_ROM_BIT:              u32 = 11;
pub const SYSCFG_MEMPOWERDOWN_USB_BIT:              u32 = 10;
pub const SYSCFG_MEMPOWERDOWN_SRAM9_BIT:            u32 = 9;
pub const SYSCFG_MEMPOWERDOWN_SRAM8_BIT:            u32 = 8;
pub const SYSCFG_MEMPOWERDOWN_SRAM7_BIT:            u32 = 7;
pub const SYSCFG_MEMPOWERDOWN_SRAM6_BIT:            u32 = 6;
pub const SYSCFG_MEMPOWERDOWN_SRAM5_BIT:            u32 = 5;
pub const SYSCFG_MEMPOWERDOWN_SRAM4_BIT:            u32 = 4;
pub const SYSCFG_MEMPOWERDOWN_SRAM3_BIT:            u32 = 3;
pub const SYSCFG_MEMPOWERDOWN_SRAM2_BIT:            u32 = 2;
pub const SYSCFG_MEMPOWERDOWN_SRAM1_BIT:            u32 = 1;
pub const SYSCFG_MEMPOWERDOWN_SRAM0_BIT:            u32 = 0;

// AUXCTRL
pub const SYSCFG_AUXCTRL_LOW:                       u32 = 0;
pub const SYSCFG_AUXCTRL_HIGH:                      u32 = 7;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
