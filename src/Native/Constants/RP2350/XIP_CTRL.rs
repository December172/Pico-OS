#![allow(dead_code)]
// XIP_CTRL

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const XIP_CTRL_BASE:                            u32 = 0x400C_8000;

pub const XIP_CTRL_CTRL:                            u32 = XIP_CTRL_BASE + 0x0;
pub const XIP_CTRL_STAT:                            u32 = XIP_CTRL_BASE + 0x8;
pub const XIP_CTRL_CTR_HIT:                         u32 = XIP_CTRL_BASE + 0xC;
pub const XIP_CTRL_CTR_ACC:                         u32 = XIP_CTRL_BASE + 0x10;
pub const XIP_CTRL_STREAM_ADDR:                     u32 = XIP_CTRL_BASE + 0x14;
pub const XIP_CTRL_STREAM_CTR:                      u32 = XIP_CTRL_BASE + 0x18;
pub const XIP_CTRL_STREAM_FIFO:                     u32 = XIP_CTRL_BASE + 0x1C;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CTRL
pub const XIP_CTRL_CTRL_WRITABLE_M1_BIT:            u32 = 11;
pub const XIP_CTRL_CTRL_WRITABLE_M0_BIT:            u32 = 10;
pub const XIP_CTRL_CTRL_SPLIT_WAYS_BIT:             u32 = 9;
pub const XIP_CTRL_CTRL_MAINT_NONSEC_BIT:           u32 = 8;
pub const XIP_CTRL_CTRL_NO_UNTRANSLATED_NONSEC_BIT: u32 = 7;
pub const XIP_CTRL_CTRL_NO_UNTRANSLATED_SEC_BIT:    u32 = 6;
pub const XIP_CTRL_CTRL_NO_UNCACHED_NONSEC_BIT:     u32 = 5;
pub const XIP_CTRL_CTRL_NO_UNCACHED_SEC_BIT:        u32 = 4;
pub const XIP_CTRL_CTRL_POWER_DOWN_BIT:             u32 = 3;
pub const XIP_CTRL_CTRL_EN_NONSECURE_BIT:           u32 = 1;
pub const XIP_CTRL_CTRL_EN_SECURE_BIT:              u32 = 0;

// STAT
pub const XIP_CTRL_STAT_FIFO_FULL_BIT:              u32 = 2;
pub const XIP_CTRL_STAT_FIFO_EMPTY_BIT:             u32 = 1;

// STREAM_ADDR
pub const XIP_CTRL_STREAM_ADDR_LOW:                 u32 = 2;
pub const XIP_CTRL_STREAM_ADDR_HIGH:                u32 = 31;

// STREAM_CTR
pub const XIP_CTRL_STREAM_CTR_LOW:                  u32 = 0;
pub const XIP_CTRL_STREAM_CTR_HIGH:                 u32 = 21;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
