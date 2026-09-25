#![allow(dead_code)]
// XIP_CTRL
pub const XIP_CTRL_BASE:                            u32 = 0x1400_0000;
pub const XIP_CTRL_CTRL:                            u32 = XIP_CTRL_BASE + 0x0;
pub const XIP_CTRL_FLUSH:                           u32 = XIP_CTRL_BASE + 0x4;
pub const XIP_CTRL_STAT:                            u32 = XIP_CTRL_BASE + 0x8;
pub const XIP_CTRL_CTR_HIT:                         u32 = XIP_CTRL_BASE + 0xC;
pub const XIP_CTRL_CTR_ACC:                         u32 = XIP_CTRL_BASE + 0x10;
pub const XIP_CTRL_STREAM_ADDR:                     u32 = XIP_CTRL_BASE + 0x14;
pub const XIP_CTRL_STREAM_CTR:                      u32 = XIP_CTRL_BASE + 0x18;
pub const XIP_CTRL_STREAM_FIFO:                     u32 = XIP_CTRL_BASE + 0x1C;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CTRL
pub const XIP_CTRL_CTRL_POWER_DOWN_BIT:             u32 = 3;
pub const XIP_CTRL_CTRL_ERR_BADWRITE_BIT:           u32 = 1;
pub const XIP_CTRL_CTRL_EN_BIT:                     u32 = 0;
// FLUSH
pub const XIP_CTRL_FLUSH_BIT:                       u32 = 0;
// STAT
pub const XIP_CTRL_STAT_FIFO_FULL_BIT:              u32 = 2;
pub const XIP_CTRL_STAT_FIFO_EMPTY_BIT:             u32 = 1;
pub const XIP_CTRL_STAT_FLUSH_READY_BIT:            u32 = 0;
// CTR_HIT
pub const XIP_CTRL_CTR_HIT_LOW:                     u32 = 0;
pub const XIP_CTRL_CTR_HIT_HIGH:                    u32 = 31;
// CTR_ACC
pub const XIP_CTRL_CTR_ACC_LOW:                     u32 = 0;
pub const XIP_CTRL_CTR_ACC_HIGH:                    u32 = 31;
// STREAM_ADDR
pub const XIP_CTRL_STREAM_ADDR_LOW:                 u32 = 2;
pub const XIP_CTRL_STREAM_ADDR_HIGH:                u32 = 31;
// STREAM_CTR
pub const XIP_CTRL_STREAM_CTR_LOW:                  u32 = 0;
pub const XIP_CTRL_STREAM_CTR_HIGH:                 u32 = 21;
// STREAM_FIFO
pub const XIP_CTRL_STREAM_FIFO_LOW:                 u32 = 0;
pub const XIP_CTRL_STREAM_FIFO_HIGH:                u32 = 31;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
