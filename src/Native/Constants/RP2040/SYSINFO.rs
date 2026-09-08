#![allow(dead_code)]
// SYSINFO
pub const SYSINFO_BASE:                             u32 = 0x4000_0000;
pub const SYSINFO_CHIP_ID:                          u32 = SYSINFO_BASE + 0x0;
pub const SYSINFO_PLATFORM:                         u32 = SYSINFO_BASE + 0x4;
pub const SYSINFO_GITREF_RP2040:                    u32 = SYSINFO_BASE + 0x40;
