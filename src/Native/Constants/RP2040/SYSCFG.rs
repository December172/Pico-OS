#![allow(dead_code)]
// SYSCFG
pub const SYSCFG_BASE:                              u32 = 0x4000_4000;
pub const SYSCFG_PROC0_NMI_MASK:                    u32 = SYSCFG_BASE + 0x0;
pub const SYSCFG_PROC1_NMI_MASK:                    u32 = SYSCFG_BASE + 0x4;
pub const SYSCFG_PROC_CONFIG:                       u32 = SYSCFG_BASE + 0x8;
pub const SYSCFG_PROC_IN_SYNC_BYPASS:               u32 = SYSCFG_BASE + 0xC;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI:            u32 = SYSCFG_BASE + 0x10;
pub const SYSCFG_DBGFORCE:                          u32 = SYSCFG_BASE + 0x14;
pub const SYSCFG_MEMPOWERDOWN:                      u32 = SYSCFG_BASE + 0x18;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// PROC0_NMI_MASK
pub const SYSCFG_PROC0_NMI_MASK_LOW:                u32 = 0;
pub const SYSCFG_PROC0_NMI_MASK_HIGH:               u32 = 31;
// PROC1_NMI_MASK
pub const SYSCFG_PROC1_NMI_MASK_LOW:                u32 = 0;
pub const SYSCFG_PROC1_NMI_MASK_HIGH:               u32 = 31;
// PROC_CONFIG
pub const SYSCFG_PROC_CONFIG_PROC1_DAP_INSTID_LOW:  u32 = 28;
pub const SYSCFG_PROC_CONFIG_PROC1_DAP_INSTID_HIGH: u32 = 31;
pub const SYSCFG_PROC_CONFIG_PROC0_DAP_INSTID_LOW:  u32 = 24;
pub const SYSCFG_PROC_CONFIG_PROC0_DAP_INSTID_HIGH: u32 = 27;
pub const SYSCFG_PROC_CONFIG_PROC1_HALTED_BIT:      u32 = 1;
pub const SYSCFG_PROC_CONFIG_PROC0_HALTED_BIT:      u32 = 0;
// PROC_IN_SYNC_BYPASS
pub const SYSCFG_PROC_IN_SYNC_BYPASS_LOW:           u32 = 0;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HIGH:          u32 = 29;
// PROC_IN_SYNC_BYPASS_HI
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_LOW:        u32 = 0;
pub const SYSCFG_PROC_IN_SYNC_BYPASS_HI_HIGH:       u32 = 5;
// DBGFORCE
pub const SYSCFG_DBGFORCE_PROC1_ATTACH_BIT:         u32 = 7;
pub const SYSCFG_DBGFORCE_PROC1_SWCLK_BIT:          u32 = 6;
pub const SYSCFG_DBGFORCE_PROC1_SWDI_BIT:           u32 = 5;
pub const SYSCFG_DBGFORCE_PROC1_SWDO_BIT:           u32 = 4;
pub const SYSCFG_DBGFORCE_PROC0_ATTACH_BIT:         u32 = 3;
pub const SYSCFG_DBGFORCE_PROC0_SWCLK_BIT:          u32 = 2;
pub const SYSCFG_DBGFORCE_PROC0_SWDI_BIT:           u32 = 1;
pub const SYSCFG_DBGFORCE_PROC0_SWDO_BIT:           u32 = 0;
// MEMPOWERDOWN
pub const SYSCFG_MEMPOWERDOWN_ROM_BIT:              u32 = 7;
pub const SYSCFG_MEMPOWERDOWN_USB_BIT:              u32 = 6;
pub const SYSCFG_MEMPOWERDOWN_SRAM5_BIT:            u32 = 5;
pub const SYSCFG_MEMPOWERDOWN_SRAM4_BIT:            u32 = 4;
pub const SYSCFG_MEMPOWERDOWN_SRAM3_BIT:            u32 = 3;
pub const SYSCFG_MEMPOWERDOWN_SRAM2_BIT:            u32 = 2;
pub const SYSCFG_MEMPOWERDOWN_SRAM1_BIT:            u32 = 1;
pub const SYSCFG_MEMPOWERDOWN_SRAM0_BIT:            u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
