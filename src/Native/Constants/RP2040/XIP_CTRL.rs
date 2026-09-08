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
