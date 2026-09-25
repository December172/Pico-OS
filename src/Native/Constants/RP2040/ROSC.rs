#![allow(dead_code)]
// ROSC
pub const ROSC_BASE:                                u32 = 0x4006_0000;
pub const ROSC_CTRL:                                u32 = ROSC_BASE + 0x0;
pub const ROSC_FREQA:                               u32 = ROSC_BASE + 0x4;
pub const ROSC_FREQB:                               u32 = ROSC_BASE + 0x8;
pub const ROSC_DORMANT:                             u32 = ROSC_BASE + 0xC;
pub const ROSC_DIV:                                 u32 = ROSC_BASE + 0x10;
pub const ROSC_PHASE:                               u32 = ROSC_BASE + 0x14;
pub const ROSC_STATUS:                              u32 = ROSC_BASE + 0x18;
pub const ROSC_RANDOMBIT:                           u32 = ROSC_BASE + 0x1C;
pub const ROSC_COUNT:                               u32 = ROSC_BASE + 0x20;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CTRL
pub const ROSC_CTRL_ENABLE_LOW:                     u32 = 12;
pub const ROSC_CTRL_ENABLE_HIGH:                    u32 = 23;
pub const ROSC_CTRL_FREQ_RANGE_LOW:                 u32 = 0;
pub const ROSC_CTRL_FREQ_RANGE_HIGH:                u32 = 11;
// FREQA
pub const ROSC_FREQA_PASSWD_LOW:                    u32 = 16;
pub const ROSC_FREQA_PASSWD_HIGH:                   u32 = 31;
pub const ROSC_FREQA_DS3_LOW:                       u32 = 12;
pub const ROSC_FREQA_DS3_HIGH:                      u32 = 14;
pub const ROSC_FREQA_DS2_LOW:                       u32 = 8;
pub const ROSC_FREQA_DS2_HIGH:                      u32 = 10;
pub const ROSC_FREQA_DS1_LOW:                       u32 = 4;
pub const ROSC_FREQA_DS1_HIGH:                      u32 = 6;
pub const ROSC_FREQA_DS0_LOW:                       u32 = 0;
pub const ROSC_FREQA_DS0_HIGH:                      u32 = 2;
// FREQB
pub const ROSC_FREQB_PASSWD_LOW:                    u32 = 16;
pub const ROSC_FREQB_PASSWD_HIGH:                   u32 = 31;
pub const ROSC_FREQB_DS7_LOW:                       u32 = 12;
pub const ROSC_FREQB_DS7_HIGH:                      u32 = 14;
pub const ROSC_FREQB_DS6_LOW:                       u32 = 8;
pub const ROSC_FREQB_DS6_HIGH:                      u32 = 10;
pub const ROSC_FREQB_DS5_LOW:                       u32 = 4;
pub const ROSC_FREQB_DS5_HIGH:                      u32 = 6;
pub const ROSC_FREQB_DS4_LOW:                       u32 = 0;
pub const ROSC_FREQB_DS4_HIGH:                      u32 = 2;
// DORMANT
pub const ROSC_DORMANT_LOW:                         u32 = 0;
pub const ROSC_DORMANT_HIGH:                        u32 = 31;
// DIV
pub const ROSC_DIV_LOW:                             u32 = 0;
pub const ROSC_DIV_HIGH:                            u32 = 11;
// PHASE
pub const ROSC_PHASE_PASSWD_LOW:                    u32 = 4;
pub const ROSC_PHASE_PASSWD_HIGH:                   u32 = 11;
pub const ROSC_PHASE_ENABLE_BIT:                    u32 = 3;
pub const ROSC_PHASE_FLIP_BIT:                      u32 = 2;
pub const ROSC_PHASE_SHIFT_LOW:                     u32 = 0;
pub const ROSC_PHASE_SHIFT_HIGH:                    u32 = 1;
// STATUS
pub const ROSC_STATUS_STABLE_BIT:                   u32 = 31;
pub const ROSC_STATUS_BADWRITE_BIT:                 u32 = 24;
pub const ROSC_STATUS_DIV_RUNNING_BIT:              u32 = 16;
pub const ROSC_STATUS_ENABLED_BIT:                  u32 = 12;
// RANDOMBIT
pub const ROSC_RANDOMBIT_BIT:                       u32 = 0;
// COUNT
pub const ROSC_COUNT_LOW:                           u32 = 0;
pub const ROSC_COUNT_HIGH:                          u32 = 7;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
