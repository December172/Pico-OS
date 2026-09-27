#![allow(dead_code)]
// HSTX_CTRL

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const HSTX_CTRL_BASE:                           u32 = 0x400C_0000;

pub const HSTX_CTRL_CSR:                            u32 = HSTX_CTRL_BASE + 0x0;
// BIT0..BIT7
pub fn HSTX_CTRL_BIT(n: u32) -> u32 {
    return HSTX_CTRL_BASE + 0x4 + n * 0x4
}

pub const HSTX_CTRL_EXPAND_SHIFT:                   u32 = HSTX_CTRL_BASE + 0x24;
pub const HSTX_CTRL_EXPAND_TMDS:                    u32 = HSTX_CTRL_BASE + 0x28;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CSR
pub const HSTX_CTRL_CSR_CLKDIV_LOW:                 u32 = 28;
pub const HSTX_CTRL_CSR_CLKDIV_HIGH:                u32 = 31;
pub const HSTX_CTRL_CSR_CLKPHASE_LOW:               u32 = 24;
pub const HSTX_CTRL_CSR_CLKPHASE_HIGH:              u32 = 27;
pub const HSTX_CTRL_CSR_N_SHIFTS_LOW:               u32 = 16;
pub const HSTX_CTRL_CSR_N_SHIFTS_HIGH:              u32 = 20;
pub const HSTX_CTRL_CSR_SHIFT_LOW:                  u32 = 8;
pub const HSTX_CTRL_CSR_SHIFT_HIGH:                 u32 = 12;
pub const HSTX_CTRL_CSR_COUPLED_SEL_LOW:            u32 = 5;
pub const HSTX_CTRL_CSR_COUPLED_SEL_HIGH:           u32 = 6;
pub const HSTX_CTRL_CSR_COUPLED_MODE_BIT:           u32 = 4;
pub const HSTX_CTRL_CSR_EXPAND_EN_BIT:              u32 = 1;
pub const HSTX_CTRL_CSR_EN_BIT:                     u32 = 0;

// BIT0, BIT1, BIT2, BIT3, BIT4, BIT5, BIT6, BIT7
pub const HSTX_CTRL_BIT_CLK_BIT:                    u32 = 17;
pub const HSTX_CTRL_BIT_INV_BIT:                    u32 = 16;
pub const HSTX_CTRL_BIT_SEL_N_LOW:                  u32 = 8;
pub const HSTX_CTRL_BIT_SEL_N_HIGH:                 u32 = 12;
pub const HSTX_CTRL_BIT_SEL_P_LOW:                  u32 = 0;
pub const HSTX_CTRL_BIT_SEL_P_HIGH:                 u32 = 4;

// EXPAND_SHIFT
pub const HSTX_CTRL_EXPAND_SHIFT_ENC_N_SHIFTS_LOW:  u32 = 24;
pub const HSTX_CTRL_EXPAND_SHIFT_ENC_N_SHIFTS_HIGH: u32 = 28;
pub const HSTX_CTRL_EXPAND_SHIFT_ENC_SHIFT_LOW:     u32 = 16;
pub const HSTX_CTRL_EXPAND_SHIFT_ENC_SHIFT_HIGH:    u32 = 20;
pub const HSTX_CTRL_EXPAND_SHIFT_RAW_N_SHIFTS_LOW:  u32 = 8;
pub const HSTX_CTRL_EXPAND_SHIFT_RAW_N_SHIFTS_HIGH: u32 = 12;
pub const HSTX_CTRL_EXPAND_SHIFT_RAW_SHIFT_LOW:     u32 = 0;
pub const HSTX_CTRL_EXPAND_SHIFT_RAW_SHIFT_HIGH:    u32 = 4;

// EXPAND_TMDS
pub const HSTX_CTRL_EXPAND_TMDS_L2_NBITS_LOW:       u32 = 21;
pub const HSTX_CTRL_EXPAND_TMDS_L2_NBITS_HIGH:      u32 = 23;
pub const HSTX_CTRL_EXPAND_TMDS_L1_NBITS_LOW:       u32 = 13;
pub const HSTX_CTRL_EXPAND_TMDS_L1_NBITS_HIGH:      u32 = 15;
pub const HSTX_CTRL_EXPAND_TMDS_L0_NBITS_LOW:       u32 = 5;
pub const HSTX_CTRL_EXPAND_TMDS_L0_NBITS_HIGH:      u32 = 7;
pub const HSTX_CTRL_EXPAND_TMDS_L2_ROT_LOW:         u32 = 16;
pub const HSTX_CTRL_EXPAND_TMDS_L2_ROT_HIGH:        u32 = 20;
pub const HSTX_CTRL_EXPAND_TMDS_L1_ROT_LOW:         u32 = 8;
pub const HSTX_CTRL_EXPAND_TMDS_L1_ROT_HIGH:        u32 = 12;
pub const HSTX_CTRL_EXPAND_TMDS_L0_ROT_LOW:         u32 = 0;
pub const HSTX_CTRL_EXPAND_TMDS_L0_ROT_HIGH:        u32 = 4;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
