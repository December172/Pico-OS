#![allow(dead_code)]
// PWM
pub const PWM_BASE:                                 u32 = 0x4005_0000;
pub const PWM_CH0_CSR:                              u32 = PWM_BASE + 0x0;
pub const PWM_CH0_DIV:                              u32 = PWM_BASE + 0x4;
pub const PWM_CH0_CTR:                              u32 = PWM_BASE + 0x8;
pub const PWM_CH0_CC:                               u32 = PWM_BASE + 0xC;
pub const PWM_CH0_TOP:                              u32 = PWM_BASE + 0x10;
pub const PWM_CH1_CSR:                              u32 = PWM_BASE + 0x14;
pub const PWM_CH1_DIV:                              u32 = PWM_BASE + 0x18;
pub const PWM_CH1_CTR:                              u32 = PWM_BASE + 0x1C;
pub const PWM_CH1_CC:                               u32 = PWM_BASE + 0x20;
pub const PWM_CH1_TOP:                              u32 = PWM_BASE + 0x24;
pub const PWM_CH2_CSR:                              u32 = PWM_BASE + 0x28;
pub const PWM_CH2_DIV:                              u32 = PWM_BASE + 0x2C;
pub const PWM_CH2_CTR:                              u32 = PWM_BASE + 0x30;
pub const PWM_CH2_CC:                               u32 = PWM_BASE + 0x34;
pub const PWM_CH2_TOP:                              u32 = PWM_BASE + 0x38;
pub const PWM_CH3_CSR:                              u32 = PWM_BASE + 0x3C;
pub const PWM_CH3_DIV:                              u32 = PWM_BASE + 0x40;
pub const PWM_CH3_CTR:                              u32 = PWM_BASE + 0x44;
pub const PWM_CH3_CC:                               u32 = PWM_BASE + 0x48;
pub const PWM_CH3_TOP:                              u32 = PWM_BASE + 0x4C;
pub const PWM_CH4_CSR:                              u32 = PWM_BASE + 0x50;
pub const PWM_CH4_DIV:                              u32 = PWM_BASE + 0x54;
pub const PWM_CH4_CTR:                              u32 = PWM_BASE + 0x58;
pub const PWM_CH4_CC:                               u32 = PWM_BASE + 0x5C;
pub const PWM_CH4_TOP:                              u32 = PWM_BASE + 0x60;
pub const PWM_CH5_CSR:                              u32 = PWM_BASE + 0x64;
pub const PWM_CH5_DIV:                              u32 = PWM_BASE + 0x68;
pub const PWM_CH5_CTR:                              u32 = PWM_BASE + 0x6C;
pub const PWM_CH5_CC:                               u32 = PWM_BASE + 0x70;
pub const PWM_CH5_TOP:                              u32 = PWM_BASE + 0x74;
pub const PWM_CH6_CSR:                              u32 = PWM_BASE + 0x78;
pub const PWM_CH6_DIV:                              u32 = PWM_BASE + 0x7C;
pub const PWM_CH6_CTR:                              u32 = PWM_BASE + 0x80;
pub const PWM_CH6_CC:                               u32 = PWM_BASE + 0x84;
pub const PWM_CH6_TOP:                              u32 = PWM_BASE + 0x88;
pub const PWM_CH7_CSR:                              u32 = PWM_BASE + 0x8C;
pub const PWM_CH7_DIV:                              u32 = PWM_BASE + 0x90;
pub const PWM_CH7_CTR:                              u32 = PWM_BASE + 0x94;
pub const PWM_CH7_CC:                               u32 = PWM_BASE + 0x98;
pub const PWM_CH7_TOP:                              u32 = PWM_BASE + 0x9C;
pub const PWM_EN:                                   u32 = PWM_BASE + 0xA0;
pub const PWM_INTR:                                 u32 = PWM_BASE + 0xA4;
pub const PWM_INTE:                                 u32 = PWM_BASE + 0xA8;
pub const PWM_INTF:                                 u32 = PWM_BASE + 0xAC;
pub const PWM_INTS:                                 u32 = PWM_BASE + 0xB0;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CH0_CSR
pub const PWM_CH0_CSR_PH_ADV_BIT:                   u32 = 7;
pub const PWM_CH0_CSR_PH_RET_BIT:                   u32 = 6;
pub const PWM_CH0_CSR_DIVMODE_LOW:                  u32 = 4;
pub const PWM_CH0_CSR_DIVMODE_HIGH:                 u32 = 5;
pub const PWM_CH0_CSR_B_INV_BIT:                    u32 = 3;
pub const PWM_CH0_CSR_A_INV_BIT:                    u32 = 2;
pub const PWM_CH0_CSR_PH_CORRECT_BIT:               u32 = 1;
pub const PWM_CH0_CSR_EN_BIT:                       u32 = 0;
// CH0_DIV
pub const PWM_CH0_DIV_INT_LOW:                      u32 = 4;
pub const PWM_CH0_DIV_INT_HIGH:                     u32 = 11;
pub const PWM_CH0_DIV_FRAC_LOW:                     u32 = 0;
pub const PWM_CH0_DIV_FRAC_HIGH:                    u32 = 3;
// CH0_CTR
pub const PWM_CH0_CTR_LOW:                          u32 = 0;
pub const PWM_CH0_CTR_HIGH:                         u32 = 15;
// CH0_CC
pub const PWM_CH0_CC_B_LOW:                         u32 = 16;
pub const PWM_CH0_CC_B_HIGH:                        u32 = 31;
pub const PWM_CH0_CC_A_LOW:                         u32 = 0;
pub const PWM_CH0_CC_A_HIGH:                        u32 = 15;
// CH0_TOP
pub const PWM_CH0_TOP_LOW:                          u32 = 0;
pub const PWM_CH0_TOP_HIGH:                         u32 = 15;
// CH1_CSR
pub const PWM_CH1_CSR_PH_ADV_BIT:                   u32 = 7;
pub const PWM_CH1_CSR_PH_RET_BIT:                   u32 = 6;
pub const PWM_CH1_CSR_DIVMODE_LOW:                  u32 = 4;
pub const PWM_CH1_CSR_DIVMODE_HIGH:                 u32 = 5;
pub const PWM_CH1_CSR_B_INV_BIT:                    u32 = 3;
pub const PWM_CH1_CSR_A_INV_BIT:                    u32 = 2;
pub const PWM_CH1_CSR_PH_CORRECT_BIT:               u32 = 1;
pub const PWM_CH1_CSR_EN_BIT:                       u32 = 0;
// CH1_DIV
pub const PWM_CH1_DIV_INT_LOW:                      u32 = 4;
pub const PWM_CH1_DIV_INT_HIGH:                     u32 = 11;
pub const PWM_CH1_DIV_FRAC_LOW:                     u32 = 0;
pub const PWM_CH1_DIV_FRAC_HIGH:                    u32 = 3;
// CH1_CTR
pub const PWM_CH1_CTR_LOW:                          u32 = 0;
pub const PWM_CH1_CTR_HIGH:                         u32 = 15;
// CH1_CC
pub const PWM_CH1_CC_B_LOW:                         u32 = 16;
pub const PWM_CH1_CC_B_HIGH:                        u32 = 31;
pub const PWM_CH1_CC_A_LOW:                         u32 = 0;
pub const PWM_CH1_CC_A_HIGH:                        u32 = 15;
// CH1_TOP
pub const PWM_CH1_TOP_LOW:                          u32 = 0;
pub const PWM_CH1_TOP_HIGH:                         u32 = 15;
// CH2_CSR
pub const PWM_CH2_CSR_PH_ADV_BIT:                   u32 = 7;
pub const PWM_CH2_CSR_PH_RET_BIT:                   u32 = 6;
pub const PWM_CH2_CSR_DIVMODE_LOW:                  u32 = 4;
pub const PWM_CH2_CSR_DIVMODE_HIGH:                 u32 = 5;
pub const PWM_CH2_CSR_B_INV_BIT:                    u32 = 3;
pub const PWM_CH2_CSR_A_INV_BIT:                    u32 = 2;
pub const PWM_CH2_CSR_PH_CORRECT_BIT:               u32 = 1;
pub const PWM_CH2_CSR_EN_BIT:                       u32 = 0;
// CH2_DIV
pub const PWM_CH2_DIV_INT_LOW:                      u32 = 4;
pub const PWM_CH2_DIV_INT_HIGH:                     u32 = 11;
pub const PWM_CH2_DIV_FRAC_LOW:                     u32 = 0;
pub const PWM_CH2_DIV_FRAC_HIGH:                    u32 = 3;
// CH2_CTR
pub const PWM_CH2_CTR_LOW:                          u32 = 0;
pub const PWM_CH2_CTR_HIGH:                         u32 = 15;
// CH2_CC
pub const PWM_CH2_CC_B_LOW:                         u32 = 16;
pub const PWM_CH2_CC_B_HIGH:                        u32 = 31;
pub const PWM_CH2_CC_A_LOW:                         u32 = 0;
pub const PWM_CH2_CC_A_HIGH:                        u32 = 15;
// CH2_TOP
pub const PWM_CH2_TOP_LOW:                          u32 = 0;
pub const PWM_CH2_TOP_HIGH:                         u32 = 15;
// CH3_CSR
pub const PWM_CH3_CSR_PH_ADV_BIT:                   u32 = 7;
pub const PWM_CH3_CSR_PH_RET_BIT:                   u32 = 6;
pub const PWM_CH3_CSR_DIVMODE_LOW:                  u32 = 4;
pub const PWM_CH3_CSR_DIVMODE_HIGH:                 u32 = 5;
pub const PWM_CH3_CSR_B_INV_BIT:                    u32 = 3;
pub const PWM_CH3_CSR_A_INV_BIT:                    u32 = 2;
pub const PWM_CH3_CSR_PH_CORRECT_BIT:               u32 = 1;
pub const PWM_CH3_CSR_EN_BIT:                       u32 = 0;
// CH3_DIV
pub const PWM_CH3_DIV_INT_LOW:                      u32 = 4;
pub const PWM_CH3_DIV_INT_HIGH:                     u32 = 11;
pub const PWM_CH3_DIV_FRAC_LOW:                     u32 = 0;
pub const PWM_CH3_DIV_FRAC_HIGH:                    u32 = 3;
// CH3_CTR
pub const PWM_CH3_CTR_LOW:                          u32 = 0;
pub const PWM_CH3_CTR_HIGH:                         u32 = 15;
// CH3_CC
pub const PWM_CH3_CC_B_LOW:                         u32 = 16;
pub const PWM_CH3_CC_B_HIGH:                        u32 = 31;
pub const PWM_CH3_CC_A_LOW:                         u32 = 0;
pub const PWM_CH3_CC_A_HIGH:                        u32 = 15;
// CH3_TOP
pub const PWM_CH3_TOP_LOW:                          u32 = 0;
pub const PWM_CH3_TOP_HIGH:                         u32 = 15;
// CH4_CSR
pub const PWM_CH4_CSR_PH_ADV_BIT:                   u32 = 7;
pub const PWM_CH4_CSR_PH_RET_BIT:                   u32 = 6;
pub const PWM_CH4_CSR_DIVMODE_LOW:                  u32 = 4;
pub const PWM_CH4_CSR_DIVMODE_HIGH:                 u32 = 5;
pub const PWM_CH4_CSR_B_INV_BIT:                    u32 = 3;
pub const PWM_CH4_CSR_A_INV_BIT:                    u32 = 2;
pub const PWM_CH4_CSR_PH_CORRECT_BIT:               u32 = 1;
pub const PWM_CH4_CSR_EN_BIT:                       u32 = 0;
// CH4_DIV
pub const PWM_CH4_DIV_INT_LOW:                      u32 = 4;
pub const PWM_CH4_DIV_INT_HIGH:                     u32 = 11;
pub const PWM_CH4_DIV_FRAC_LOW:                     u32 = 0;
pub const PWM_CH4_DIV_FRAC_HIGH:                    u32 = 3;
// CH4_CTR
pub const PWM_CH4_CTR_LOW:                          u32 = 0;
pub const PWM_CH4_CTR_HIGH:                         u32 = 15;
// CH4_CC
pub const PWM_CH4_CC_B_LOW:                         u32 = 16;
pub const PWM_CH4_CC_B_HIGH:                        u32 = 31;
pub const PWM_CH4_CC_A_LOW:                         u32 = 0;
pub const PWM_CH4_CC_A_HIGH:                        u32 = 15;
// CH4_TOP
pub const PWM_CH4_TOP_LOW:                          u32 = 0;
pub const PWM_CH4_TOP_HIGH:                         u32 = 15;
// CH5_CSR
pub const PWM_CH5_CSR_PH_ADV_BIT:                   u32 = 7;
pub const PWM_CH5_CSR_PH_RET_BIT:                   u32 = 6;
pub const PWM_CH5_CSR_DIVMODE_LOW:                  u32 = 4;
pub const PWM_CH5_CSR_DIVMODE_HIGH:                 u32 = 5;
pub const PWM_CH5_CSR_B_INV_BIT:                    u32 = 3;
pub const PWM_CH5_CSR_A_INV_BIT:                    u32 = 2;
pub const PWM_CH5_CSR_PH_CORRECT_BIT:               u32 = 1;
pub const PWM_CH5_CSR_EN_BIT:                       u32 = 0;
// CH5_DIV
pub const PWM_CH5_DIV_INT_LOW:                      u32 = 4;
pub const PWM_CH5_DIV_INT_HIGH:                     u32 = 11;
pub const PWM_CH5_DIV_FRAC_LOW:                     u32 = 0;
pub const PWM_CH5_DIV_FRAC_HIGH:                    u32 = 3;
// CH5_CTR
pub const PWM_CH5_CTR_LOW:                          u32 = 0;
pub const PWM_CH5_CTR_HIGH:                         u32 = 15;
// CH5_CC
pub const PWM_CH5_CC_B_LOW:                         u32 = 16;
pub const PWM_CH5_CC_B_HIGH:                        u32 = 31;
pub const PWM_CH5_CC_A_LOW:                         u32 = 0;
pub const PWM_CH5_CC_A_HIGH:                        u32 = 15;
// CH5_TOP
pub const PWM_CH5_TOP_LOW:                          u32 = 0;
pub const PWM_CH5_TOP_HIGH:                         u32 = 15;
// CH6_CSR
pub const PWM_CH6_CSR_PH_ADV_BIT:                   u32 = 7;
pub const PWM_CH6_CSR_PH_RET_BIT:                   u32 = 6;
pub const PWM_CH6_CSR_DIVMODE_LOW:                  u32 = 4;
pub const PWM_CH6_CSR_DIVMODE_HIGH:                 u32 = 5;
pub const PWM_CH6_CSR_B_INV_BIT:                    u32 = 3;
pub const PWM_CH6_CSR_A_INV_BIT:                    u32 = 2;
pub const PWM_CH6_CSR_PH_CORRECT_BIT:               u32 = 1;
pub const PWM_CH6_CSR_EN_BIT:                       u32 = 0;
// CH6_DIV
pub const PWM_CH6_DIV_INT_LOW:                      u32 = 4;
pub const PWM_CH6_DIV_INT_HIGH:                     u32 = 11;
pub const PWM_CH6_DIV_FRAC_LOW:                     u32 = 0;
pub const PWM_CH6_DIV_FRAC_HIGH:                    u32 = 3;
// CH6_CTR
pub const PWM_CH6_CTR_LOW:                          u32 = 0;
pub const PWM_CH6_CTR_HIGH:                         u32 = 15;
// CH6_CC
pub const PWM_CH6_CC_B_LOW:                         u32 = 16;
pub const PWM_CH6_CC_B_HIGH:                        u32 = 31;
pub const PWM_CH6_CC_A_LOW:                         u32 = 0;
pub const PWM_CH6_CC_A_HIGH:                        u32 = 15;
// CH6_TOP
pub const PWM_CH6_TOP_LOW:                          u32 = 0;
pub const PWM_CH6_TOP_HIGH:                         u32 = 15;
// CH7_CSR
pub const PWM_CH7_CSR_PH_ADV_BIT:                   u32 = 7;
pub const PWM_CH7_CSR_PH_RET_BIT:                   u32 = 6;
pub const PWM_CH7_CSR_DIVMODE_LOW:                  u32 = 4;
pub const PWM_CH7_CSR_DIVMODE_HIGH:                 u32 = 5;
pub const PWM_CH7_CSR_B_INV_BIT:                    u32 = 3;
pub const PWM_CH7_CSR_A_INV_BIT:                    u32 = 2;
pub const PWM_CH7_CSR_PH_CORRECT_BIT:               u32 = 1;
pub const PWM_CH7_CSR_EN_BIT:                       u32 = 0;
// CH7_DIV
pub const PWM_CH7_DIV_INT_LOW:                      u32 = 4;
pub const PWM_CH7_DIV_INT_HIGH:                     u32 = 11;
pub const PWM_CH7_DIV_FRAC_LOW:                     u32 = 0;
pub const PWM_CH7_DIV_FRAC_HIGH:                    u32 = 3;
// CH7_CTR
pub const PWM_CH7_CTR_LOW:                          u32 = 0;
pub const PWM_CH7_CTR_HIGH:                         u32 = 15;
// CH7_CC
pub const PWM_CH7_CC_B_LOW:                         u32 = 16;
pub const PWM_CH7_CC_B_HIGH:                        u32 = 31;
pub const PWM_CH7_CC_A_LOW:                         u32 = 0;
pub const PWM_CH7_CC_A_HIGH:                        u32 = 15;
// CH7_TOP
pub const PWM_CH7_TOP_LOW:                          u32 = 0;
pub const PWM_CH7_TOP_HIGH:                         u32 = 15;
// EN
pub const PWM_EN_CH7_BIT:                           u32 = 7;
pub const PWM_EN_CH6_BIT:                           u32 = 6;
pub const PWM_EN_CH5_BIT:                           u32 = 5;
pub const PWM_EN_CH4_BIT:                           u32 = 4;
pub const PWM_EN_CH3_BIT:                           u32 = 3;
pub const PWM_EN_CH2_BIT:                           u32 = 2;
pub const PWM_EN_CH1_BIT:                           u32 = 1;
pub const PWM_EN_CH0_BIT:                           u32 = 0;
// INTR
pub const PWM_INTR_CH7_BIT:                         u32 = 7;
pub const PWM_INTR_CH6_BIT:                         u32 = 6;
pub const PWM_INTR_CH5_BIT:                         u32 = 5;
pub const PWM_INTR_CH4_BIT:                         u32 = 4;
pub const PWM_INTR_CH3_BIT:                         u32 = 3;
pub const PWM_INTR_CH2_BIT:                         u32 = 2;
pub const PWM_INTR_CH1_BIT:                         u32 = 1;
pub const PWM_INTR_CH0_BIT:                         u32 = 0;
// INTE
pub const PWM_INTE_CH7_BIT:                         u32 = 7;
pub const PWM_INTE_CH6_BIT:                         u32 = 6;
pub const PWM_INTE_CH5_BIT:                         u32 = 5;
pub const PWM_INTE_CH4_BIT:                         u32 = 4;
pub const PWM_INTE_CH3_BIT:                         u32 = 3;
pub const PWM_INTE_CH2_BIT:                         u32 = 2;
pub const PWM_INTE_CH1_BIT:                         u32 = 1;
pub const PWM_INTE_CH0_BIT:                         u32 = 0;
// INTF
pub const PWM_INTF_CH7_BIT:                         u32 = 7;
pub const PWM_INTF_CH6_BIT:                         u32 = 6;
pub const PWM_INTF_CH5_BIT:                         u32 = 5;
pub const PWM_INTF_CH4_BIT:                         u32 = 4;
pub const PWM_INTF_CH3_BIT:                         u32 = 3;
pub const PWM_INTF_CH2_BIT:                         u32 = 2;
pub const PWM_INTF_CH1_BIT:                         u32 = 1;
pub const PWM_INTF_CH0_BIT:                         u32 = 0;
// INTS
pub const PWM_INTS_CH7_BIT:                         u32 = 7;
pub const PWM_INTS_CH6_BIT:                         u32 = 6;
pub const PWM_INTS_CH5_BIT:                         u32 = 5;
pub const PWM_INTS_CH4_BIT:                         u32 = 4;
pub const PWM_INTS_CH3_BIT:                         u32 = 3;
pub const PWM_INTS_CH2_BIT:                         u32 = 2;
pub const PWM_INTS_CH1_BIT:                         u32 = 1;
pub const PWM_INTS_CH0_BIT:                         u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
