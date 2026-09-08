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
