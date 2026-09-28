#![allow(dead_code)]
// PWM

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const PWM_BASE:                                 u32 = 0x400A_8000;

// CH0_CSR..CH11_CSR
pub fn PWM_CH_CSR(n: u32) -> u32 {
    return PWM_BASE + 0x0 + n * 0x14
}

// CH0_DIV..CH11_DIV
pub fn PWM_CH_DIV(n: u32) -> u32 {
    return PWM_BASE + 0x4 + n * 0x14
}

// CH0_CTR..CH11_CTR
pub fn PWM_CH_CTR(n: u32) -> u32 {
    return PWM_BASE + 0x8 + n * 0x14
}

// CH0_CC..CH11_CC
pub fn PWM_CH_CC(n: u32) -> u32 {
    return PWM_BASE + 0xC + n * 0x14
}

// CH0_TOP..CH11_TOP
pub fn PWM_CH_TOP(n: u32) -> u32 {
    return PWM_BASE + 0x10 + n * 0x14
}

pub const PWM_EN:                                   u32 = PWM_BASE + 0xF0;
pub const PWM_INTR:                                 u32 = PWM_BASE + 0xF4;
// IRQ0_INTE..IRQ1_INTE
pub fn PWM_IRQ_INTE(n: u32) -> u32 {
    return PWM_BASE + 0xF8 + n * 0xC
}

// IRQ0_INTF..IRQ1_INTF
pub fn PWM_IRQ_INTF(n: u32) -> u32 {
    return PWM_BASE + 0xFC + n * 0xC
}

// IRQ0_INTS..IRQ1_INTS
pub fn PWM_IRQ_INTS(n: u32) -> u32 {
    return PWM_BASE + 0x100 + n * 0xC
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CH0_CSR..CH11_CSR
pub const PWM_CH_CSR_PH_ADV_BIT:                    u32 = 7;
pub const PWM_CH_CSR_PH_RET_BIT:                    u32 = 6;
pub const PWM_CH_CSR_DIVMODE_LOW:                   u32 = 4;
pub const PWM_CH_CSR_DIVMODE_HIGH:                  u32 = 5;
pub const PWM_CH_CSR_B_INV_BIT:                     u32 = 3;
pub const PWM_CH_CSR_A_INV_BIT:                     u32 = 2;
pub const PWM_CH_CSR_PH_CORRECT_BIT:                u32 = 1;
pub const PWM_CH_CSR_EN_BIT:                        u32 = 0;

// CH0_DIV..CH11_DIV
pub const PWM_CH_DIV_INT_LOW:                       u32 = 4;
pub const PWM_CH_DIV_INT_HIGH:                      u32 = 11;
pub const PWM_CH_DIV_FRAC_LOW:                      u32 = 0;
pub const PWM_CH_DIV_FRAC_HIGH:                     u32 = 3;

// CH0_CTR..CH11_CTR
pub const PWM_CH_CTR_LOW:                           u32 = 0;
pub const PWM_CH_CTR_HIGH:                          u32 = 15;

// CH0_CC..CH11_CC
pub const PWM_CH_CC_B_LOW:                          u32 = 16;
pub const PWM_CH_CC_B_HIGH:                         u32 = 31;
pub const PWM_CH_CC_A_LOW:                          u32 = 0;
pub const PWM_CH_CC_A_HIGH:                         u32 = 15;

// CH0_TOP..CH11_TOP
pub const PWM_CH_TOP_LOW:                           u32 = 0;
pub const PWM_CH_TOP_HIGH:                          u32 = 15;

// EN
pub const PWM_EN_CH11_BIT:                          u32 = 11;
pub const PWM_EN_CH10_BIT:                          u32 = 10;
pub const PWM_EN_CH9_BIT:                           u32 = 9;
pub const PWM_EN_CH8_BIT:                           u32 = 8;
pub const PWM_EN_CH7_BIT:                           u32 = 7;
pub const PWM_EN_CH6_BIT:                           u32 = 6;
pub const PWM_EN_CH5_BIT:                           u32 = 5;
pub const PWM_EN_CH4_BIT:                           u32 = 4;
pub const PWM_EN_CH3_BIT:                           u32 = 3;
pub const PWM_EN_CH2_BIT:                           u32 = 2;
pub const PWM_EN_CH1_BIT:                           u32 = 1;
pub const PWM_EN_CH0_BIT:                           u32 = 0;

// INTR
pub const PWM_INTR_CH11_BIT:                        u32 = 11;
pub const PWM_INTR_CH10_BIT:                        u32 = 10;
pub const PWM_INTR_CH9_BIT:                         u32 = 9;
pub const PWM_INTR_CH8_BIT:                         u32 = 8;
pub const PWM_INTR_CH7_BIT:                         u32 = 7;
pub const PWM_INTR_CH6_BIT:                         u32 = 6;
pub const PWM_INTR_CH5_BIT:                         u32 = 5;
pub const PWM_INTR_CH4_BIT:                         u32 = 4;
pub const PWM_INTR_CH3_BIT:                         u32 = 3;
pub const PWM_INTR_CH2_BIT:                         u32 = 2;
pub const PWM_INTR_CH1_BIT:                         u32 = 1;
pub const PWM_INTR_CH0_BIT:                         u32 = 0;

// IRQ0_INTE, IRQ1_INTE
pub const PWM_IRQ_INTE_CH11_BIT:                    u32 = 11;
pub const PWM_IRQ_INTE_CH10_BIT:                    u32 = 10;
pub const PWM_IRQ_INTE_CH9_BIT:                     u32 = 9;
pub const PWM_IRQ_INTE_CH8_BIT:                     u32 = 8;
pub const PWM_IRQ_INTE_CH7_BIT:                     u32 = 7;
pub const PWM_IRQ_INTE_CH6_BIT:                     u32 = 6;
pub const PWM_IRQ_INTE_CH5_BIT:                     u32 = 5;
pub const PWM_IRQ_INTE_CH4_BIT:                     u32 = 4;
pub const PWM_IRQ_INTE_CH3_BIT:                     u32 = 3;
pub const PWM_IRQ_INTE_CH2_BIT:                     u32 = 2;
pub const PWM_IRQ_INTE_CH1_BIT:                     u32 = 1;
pub const PWM_IRQ_INTE_CH0_BIT:                     u32 = 0;

// IRQ0_INTF, IRQ1_INTF
pub const PWM_IRQ_INTF_CH11_BIT:                    u32 = 11;
pub const PWM_IRQ_INTF_CH10_BIT:                    u32 = 10;
pub const PWM_IRQ_INTF_CH9_BIT:                     u32 = 9;
pub const PWM_IRQ_INTF_CH8_BIT:                     u32 = 8;
pub const PWM_IRQ_INTF_CH7_BIT:                     u32 = 7;
pub const PWM_IRQ_INTF_CH6_BIT:                     u32 = 6;
pub const PWM_IRQ_INTF_CH5_BIT:                     u32 = 5;
pub const PWM_IRQ_INTF_CH4_BIT:                     u32 = 4;
pub const PWM_IRQ_INTF_CH3_BIT:                     u32 = 3;
pub const PWM_IRQ_INTF_CH2_BIT:                     u32 = 2;
pub const PWM_IRQ_INTF_CH1_BIT:                     u32 = 1;
pub const PWM_IRQ_INTF_CH0_BIT:                     u32 = 0;

// IRQ0_INTS, IRQ1_INTS
pub const PWM_IRQ_INTS_CH11_BIT:                    u32 = 11;
pub const PWM_IRQ_INTS_CH10_BIT:                    u32 = 10;
pub const PWM_IRQ_INTS_CH9_BIT:                     u32 = 9;
pub const PWM_IRQ_INTS_CH8_BIT:                     u32 = 8;
pub const PWM_IRQ_INTS_CH7_BIT:                     u32 = 7;
pub const PWM_IRQ_INTS_CH6_BIT:                     u32 = 6;
pub const PWM_IRQ_INTS_CH5_BIT:                     u32 = 5;
pub const PWM_IRQ_INTS_CH4_BIT:                     u32 = 4;
pub const PWM_IRQ_INTS_CH3_BIT:                     u32 = 3;
pub const PWM_IRQ_INTS_CH2_BIT:                     u32 = 2;
pub const PWM_IRQ_INTS_CH1_BIT:                     u32 = 1;
pub const PWM_IRQ_INTS_CH0_BIT:                     u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CH0_CSR..CH11_CSR: DIVMODE
pub const PWM_CH_CSR_DIVMODE_DIV:                   u32 = 0x0;
pub const PWM_CH_CSR_DIVMODE_LEVEL:                 u32 = 0x1;
pub const PWM_CH_CSR_DIVMODE_RISE:                  u32 = 0x2;
pub const PWM_CH_CSR_DIVMODE_FALL:                  u32 = 0x3;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
