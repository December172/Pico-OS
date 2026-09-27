#![allow(dead_code)]
// TRNG

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const TRNG_BASE:                                u32 = 0x400F_0000;

pub const TRNG_RNG_IMR:                             u32 = TRNG_BASE + 0x100;
pub const TRNG_RNG_ISR:                             u32 = TRNG_BASE + 0x104;
pub const TRNG_RNG_ICR:                             u32 = TRNG_BASE + 0x108;
pub const TRNG_TRNG_CONFIG:                         u32 = TRNG_BASE + 0x10C;
pub const TRNG_TRNG_VALID:                          u32 = TRNG_BASE + 0x110;
// EHR_DATA0..EHR_DATA5
pub fn TRNG_EHR_DATA(n: u32) -> u32 {
    return TRNG_BASE + 0x114 + n * 0x4
}

pub const TRNG_RND_SOURCE_ENABLE:                   u32 = TRNG_BASE + 0x12C;
pub const TRNG_SAMPLE_CNT1:                         u32 = TRNG_BASE + 0x130;
pub const TRNG_AUTOCORR_STATISTIC:                  u32 = TRNG_BASE + 0x134;
pub const TRNG_TRNG_DEBUG_CONTROL:                  u32 = TRNG_BASE + 0x138;
pub const TRNG_TRNG_SW_RESET:                       u32 = TRNG_BASE + 0x140;
pub const TRNG_RNG_DEBUG_EN_INPUT:                  u32 = TRNG_BASE + 0x1B4;
pub const TRNG_TRNG_BUSY:                           u32 = TRNG_BASE + 0x1B8;
pub const TRNG_RST_BITS_COUNTER:                    u32 = TRNG_BASE + 0x1BC;
pub const TRNG_RNG_VERSION:                         u32 = TRNG_BASE + 0x1C0;
// RNG_BIST_CNTR_0..RNG_BIST_CNTR_2
pub fn TRNG_RNG_BIST_CNTR(n: u32) -> u32 {
    return TRNG_BASE + 0x1E0 + n * 0x4
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// RNG_IMR
pub const TRNG_RNG_IMR_RESERVED_LOW:                u32 = 4;
pub const TRNG_RNG_IMR_RESERVED_HIGH:               u32 = 31;
pub const TRNG_RNG_IMR_VN_ERR_INT_MASK_BIT:         u32 = 3;
pub const TRNG_RNG_IMR_CRNGT_ERR_INT_MASK_BIT:      u32 = 2;
pub const TRNG_RNG_IMR_AUTOCORR_ERR_INT_MASK_BIT:   u32 = 1;
pub const TRNG_RNG_IMR_EHR_VALID_INT_MASK_BIT:      u32 = 0;

// RNG_ISR, RNG_ICR
pub const TRNG_RNG_RESERVED_LOW:                    u32 = 4;
pub const TRNG_RNG_RESERVED_HIGH:                   u32 = 31;
pub const TRNG_RNG_VN_ERR_BIT:                      u32 = 3;
pub const TRNG_RNG_CRNGT_ERR_BIT:                   u32 = 2;
pub const TRNG_RNG_AUTOCORR_ERR_BIT:                u32 = 1;
pub const TRNG_RNG_EHR_VALID_BIT:                   u32 = 0;

// TRNG_CONFIG
pub const TRNG_TRNG_CONFIG_RESERVED_LOW:            u32 = 2;
pub const TRNG_TRNG_CONFIG_RESERVED_HIGH:           u32 = 31;
pub const TRNG_TRNG_CONFIG_RND_SRC_SEL_LOW:         u32 = 0;
pub const TRNG_TRNG_CONFIG_RND_SRC_SEL_HIGH:        u32 = 1;

// TRNG_VALID
pub const TRNG_TRNG_VALID_RESERVED_LOW:             u32 = 1;
pub const TRNG_TRNG_VALID_RESERVED_HIGH:            u32 = 31;
pub const TRNG_TRNG_VALID_EHR_VALID_BIT:            u32 = 0;

// RND_SOURCE_ENABLE
pub const TRNG_RND_SOURCE_ENABLE_RESERVED_LOW:      u32 = 1;
pub const TRNG_RND_SOURCE_ENABLE_RESERVED_HIGH:     u32 = 31;
pub const TRNG_RND_SOURCE_ENABLE_RND_SRC_EN_BIT:    u32 = 0;

// AUTOCORR_STATISTIC
pub const TRNG_AUTOCORR_STATISTIC_RESERVED_LOW:     u32 = 22;
pub const TRNG_AUTOCORR_STATISTIC_RESERVED_HIGH:    u32 = 31;
pub const TRNG_AUTOCORR_STATISTIC_AUTOCORR_FAILS_LOW:u32 = 14;
pub const TRNG_AUTOCORR_STATISTIC_AUTOCORR_FAILS_HIGH:u32 = 21;
pub const TRNG_AUTOCORR_STATISTIC_AUTOCORR_TRYS_LOW:u32 = 0;
pub const TRNG_AUTOCORR_STATISTIC_AUTOCORR_TRYS_HIGH:u32 = 13;

// TRNG_DEBUG_CONTROL
pub const TRNG_TRNG_DEBUG_CONTROL_AUTO_CORRELATE_BYPASS_BIT:u32 = 3;
pub const TRNG_TRNG_DEBUG_CONTROL_TRNG_CRNGT_BYPASS_BIT:u32 = 2;
pub const TRNG_TRNG_DEBUG_CONTROL_VNC_BYPASS_BIT:   u32 = 1;
pub const TRNG_TRNG_DEBUG_CONTROL_RESERVED_BIT:     u32 = 0;

// TRNG_SW_RESET
pub const TRNG_TRNG_SW_RESET_RESERVED_LOW:          u32 = 1;
pub const TRNG_TRNG_SW_RESET_RESERVED_HIGH:         u32 = 31;
pub const TRNG_TRNG_SW_RESET_BIT:                   u32 = 0;

// RNG_DEBUG_EN_INPUT
pub const TRNG_RNG_DEBUG_EN_INPUT_RESERVED_LOW:     u32 = 1;
pub const TRNG_RNG_DEBUG_EN_INPUT_RESERVED_HIGH:    u32 = 31;
pub const TRNG_RNG_DEBUG_EN_INPUT_RNG_DEBUG_EN_BIT: u32 = 0;

// TRNG_BUSY
pub const TRNG_TRNG_BUSY_RESERVED_LOW:              u32 = 1;
pub const TRNG_TRNG_BUSY_RESERVED_HIGH:             u32 = 31;
pub const TRNG_TRNG_BUSY_BIT:                       u32 = 0;

// RST_BITS_COUNTER
pub const TRNG_RST_BITS_COUNTER_RESERVED_LOW:       u32 = 1;
pub const TRNG_RST_BITS_COUNTER_RESERVED_HIGH:      u32 = 31;
pub const TRNG_RST_BITS_COUNTER_BIT:                u32 = 0;

// RNG_VERSION
pub const TRNG_RNG_VERSION_RESERVED_LOW:            u32 = 8;
pub const TRNG_RNG_VERSION_RESERVED_HIGH:           u32 = 31;
pub const TRNG_RNG_VERSION_RNG_USE_5_SBOXES_BIT:    u32 = 7;
pub const TRNG_RNG_VERSION_RESEEDING_EXISTS_BIT:    u32 = 6;
pub const TRNG_RNG_VERSION_KAT_EXISTS_BIT:          u32 = 5;
pub const TRNG_RNG_VERSION_PRNG_EXISTS_BIT:         u32 = 4;
pub const TRNG_RNG_VERSION_TRNG_TESTS_BYPASS_EN_BIT:u32 = 3;
pub const TRNG_RNG_VERSION_AUTOCORR_EXISTS_BIT:     u32 = 2;
pub const TRNG_RNG_VERSION_CRNGT_EXISTS_BIT:        u32 = 1;
pub const TRNG_RNG_VERSION_EHR_WIDTH_192_BIT:       u32 = 0;

// RNG_BIST_CNTR_0, RNG_BIST_CNTR_1, RNG_BIST_CNTR_2
pub const TRNG_RNG_BIST_CNTR_RESERVED_LOW:          u32 = 22;
pub const TRNG_RNG_BIST_CNTR_RESERVED_HIGH:         u32 = 31;
pub const TRNG_RNG_BIST_CNTR_ROSC_CNTR_VAL_LOW:     u32 = 0;
pub const TRNG_RNG_BIST_CNTR_ROSC_CNTR_VAL_HIGH:    u32 = 21;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
