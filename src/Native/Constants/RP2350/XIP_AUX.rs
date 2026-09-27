#![allow(dead_code)]
// XIP_AUX

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const XIP_AUX_BASE:                             u32 = 0x5050_0000;

pub const XIP_AUX_STREAM:                           u32 = XIP_AUX_BASE + 0x0;
pub const XIP_AUX_QMI_DIRECT_TX:                    u32 = XIP_AUX_BASE + 0x4;
pub const XIP_AUX_QMI_DIRECT_RX:                    u32 = XIP_AUX_BASE + 0x8;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// QMI_DIRECT_TX
pub const XIP_AUX_QMI_DIRECT_TX_NOPUSH_BIT:         u32 = 20;
pub const XIP_AUX_QMI_DIRECT_TX_OE_BIT:             u32 = 19;
pub const XIP_AUX_QMI_DIRECT_TX_DWIDTH_BIT:         u32 = 18;
pub const XIP_AUX_QMI_DIRECT_TX_IWIDTH_LOW:         u32 = 16;
pub const XIP_AUX_QMI_DIRECT_TX_IWIDTH_HIGH:        u32 = 17;
pub const XIP_AUX_QMI_DIRECT_TX_DATA_LOW:           u32 = 0;
pub const XIP_AUX_QMI_DIRECT_TX_DATA_HIGH:          u32 = 15;

// QMI_DIRECT_RX
pub const XIP_AUX_QMI_DIRECT_RX_LOW:                u32 = 0;
pub const XIP_AUX_QMI_DIRECT_RX_HIGH:               u32 = 15;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// QMI_DIRECT_TX: IWIDTH
pub const XIP_AUX_QMI_DIRECT_TX_IWIDTH_S:           u32 = 0x0;
pub const XIP_AUX_QMI_DIRECT_TX_IWIDTH_D:           u32 = 0x1;
pub const XIP_AUX_QMI_DIRECT_TX_IWIDTH_Q:           u32 = 0x2;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
