#![allow(dead_code)]
// SYSINFO
pub const SYSINFO_BASE:                             u32 = 0x4000_0000;
pub const SYSINFO_CHIP_ID:                          u32 = SYSINFO_BASE + 0x0;
pub const SYSINFO_PLATFORM:                         u32 = SYSINFO_BASE + 0x4;
pub const SYSINFO_GITREF_RP2040:                    u32 = SYSINFO_BASE + 0x40;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CHIP_ID
pub const SYSINFO_CHIP_ID_REVISION_LOW:             u32 = 28;
pub const SYSINFO_CHIP_ID_REVISION_HIGH:            u32 = 31;
pub const SYSINFO_CHIP_ID_PART_LOW:                 u32 = 12;
pub const SYSINFO_CHIP_ID_PART_HIGH:                u32 = 27;
pub const SYSINFO_CHIP_ID_MANUFACTURER_LOW:         u32 = 0;
pub const SYSINFO_CHIP_ID_MANUFACTURER_HIGH:        u32 = 11;
// PLATFORM
pub const SYSINFO_PLATFORM_ASIC_BIT:                u32 = 1;
pub const SYSINFO_PLATFORM_FPGA_BIT:                u32 = 0;
// GITREF_RP2040
pub const SYSINFO_GITREF_RP2040_LOW:                u32 = 0;
pub const SYSINFO_GITREF_RP2040_HIGH:               u32 = 31;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
