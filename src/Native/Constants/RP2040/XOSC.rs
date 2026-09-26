#![allow(dead_code)]
// XOSC
pub const XOSC_BASE:                                u32 = 0x4002_4000;
pub const XOSC_CTRL:                                u32 = XOSC_BASE + 0x0;
pub const XOSC_STATUS:                              u32 = XOSC_BASE + 0x4;
pub const XOSC_DORMANT:                             u32 = XOSC_BASE + 0x8;
pub const XOSC_STARTUP:                             u32 = XOSC_BASE + 0xC;
pub const XOSC_COUNT:                               u32 = XOSC_BASE + 0x1C;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CTRL
pub const XOSC_CTRL_ENABLE_LOW:                     u32 = 12;
pub const XOSC_CTRL_ENABLE_HIGH:                    u32 = 23;
pub const XOSC_CTRL_FREQ_RANGE_LOW:                 u32 = 0;
pub const XOSC_CTRL_FREQ_RANGE_HIGH:                u32 = 11;
// STATUS
pub const XOSC_STATUS_STABLE_BIT:                   u32 = 31;
pub const XOSC_STATUS_BADWRITE_BIT:                 u32 = 24;
pub const XOSC_STATUS_ENABLED_BIT:                  u32 = 12;
pub const XOSC_STATUS_FREQ_RANGE_LOW:               u32 = 0;
pub const XOSC_STATUS_FREQ_RANGE_HIGH:              u32 = 1;
// DORMANT
pub const XOSC_DORMANT_LOW:                         u32 = 0;
pub const XOSC_DORMANT_HIGH:                        u32 = 31;
// STARTUP
pub const XOSC_STARTUP_X4_BIT:                      u32 = 20;
pub const XOSC_STARTUP_DELAY_LOW:                   u32 = 0;
pub const XOSC_STARTUP_DELAY_HIGH:                  u32 = 13;
// COUNT
pub const XOSC_COUNT_LOW:                           u32 = 0;
pub const XOSC_COUNT_HIGH:                          u32 = 7;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CTRL: ENABLE
pub const XOSC_CTRL_ENABLE_DISABLE:                 u32 = 0xD1E;
pub const XOSC_CTRL_ENABLE_ENABLE:                  u32 = 0xFAB;

// CTRL: FREQ_RANGE
pub const XOSC_CTRL_FREQ_RANGE_1_15MHZ:             u32 = 0xAA0;
pub const XOSC_CTRL_FREQ_RANGE_RESERVED_1:          u32 = 0xAA1;
pub const XOSC_CTRL_FREQ_RANGE_RESERVED_2:          u32 = 0xAA2;
pub const XOSC_CTRL_FREQ_RANGE_RESERVED_3:          u32 = 0xAA3;

// STATUS: FREQ_RANGE
pub const XOSC_STATUS_FREQ_RANGE_1_15MHZ:           u32 = 0x0;
pub const XOSC_STATUS_FREQ_RANGE_RESERVED_1:        u32 = 0x1;
pub const XOSC_STATUS_FREQ_RANGE_RESERVED_2:        u32 = 0x2;
pub const XOSC_STATUS_FREQ_RANGE_RESERVED_3:        u32 = 0x3;

// DORMANT: DORMANT
pub const XOSC_DORMANT_DORMANT:                     u32 = 0x636F6D61;
pub const XOSC_DORMANT_WAKE:                        u32 = 0x77616B65;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
