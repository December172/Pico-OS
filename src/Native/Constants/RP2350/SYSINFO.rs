#![allow(dead_code)]
// SYSINFO

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const SYSINFO_BASE:                             u32 = 0x4000_0000;

pub const SYSINFO_CHIP_ID:                          u32 = SYSINFO_BASE + 0x0;
pub const SYSINFO_PACKAGE_SEL:                      u32 = SYSINFO_BASE + 0x4;
pub const SYSINFO_PLATFORM:                         u32 = SYSINFO_BASE + 0x8;
pub const SYSINFO_GITREF_RP2350:                    u32 = SYSINFO_BASE + 0x14;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CHIP_ID
pub const SYSINFO_CHIP_ID_REVISION_LOW:             u32 = 28;
pub const SYSINFO_CHIP_ID_REVISION_HIGH:            u32 = 31;
pub const SYSINFO_CHIP_ID_PART_LOW:                 u32 = 12;
pub const SYSINFO_CHIP_ID_PART_HIGH:                u32 = 27;
pub const SYSINFO_CHIP_ID_MANUFACTURER_LOW:         u32 = 1;
pub const SYSINFO_CHIP_ID_MANUFACTURER_HIGH:        u32 = 11;
pub const SYSINFO_CHIP_ID_STOP_BIT_BIT:             u32 = 0;

// PACKAGE_SEL
pub const SYSINFO_PACKAGE_SEL_BIT:                  u32 = 0;

// PLATFORM
pub const SYSINFO_PLATFORM_GATESIM_BIT:             u32 = 4;
pub const SYSINFO_PLATFORM_BATCHSIM_BIT:            u32 = 3;
pub const SYSINFO_PLATFORM_HDLSIM_BIT:              u32 = 2;
pub const SYSINFO_PLATFORM_ASIC_BIT:                u32 = 1;
pub const SYSINFO_PLATFORM_FPGA_BIT:                u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
