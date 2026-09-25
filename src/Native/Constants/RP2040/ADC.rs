#![allow(dead_code)]
// ADC
pub const ADC_BASE:                                 u32 = 0x4004_C000;
pub const ADC_CS:                                   u32 = ADC_BASE + 0x0;
pub const ADC_RESULT:                               u32 = ADC_BASE + 0x4;
pub const ADC_FCS:                                  u32 = ADC_BASE + 0x8;
pub const ADC_FIFO:                                 u32 = ADC_BASE + 0xC;
pub const ADC_DIV:                                  u32 = ADC_BASE + 0x10;
pub const ADC_INTR:                                 u32 = ADC_BASE + 0x14;
pub const ADC_INTE:                                 u32 = ADC_BASE + 0x18;
pub const ADC_INTF:                                 u32 = ADC_BASE + 0x1C;
pub const ADC_INTS:                                 u32 = ADC_BASE + 0x20;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CS
pub const ADC_CS_RROBIN_LOW:                        u32 = 16;
pub const ADC_CS_RROBIN_HIGH:                       u32 = 20;
pub const ADC_CS_AINSEL_LOW:                        u32 = 12;
pub const ADC_CS_AINSEL_HIGH:                       u32 = 14;
pub const ADC_CS_ERR_STICKY_BIT:                    u32 = 10;
pub const ADC_CS_ERR_BIT:                           u32 = 9;
pub const ADC_CS_READY_BIT:                         u32 = 8;
pub const ADC_CS_START_MANY_BIT:                    u32 = 3;
pub const ADC_CS_START_ONCE_BIT:                    u32 = 2;
pub const ADC_CS_TS_EN_BIT:                         u32 = 1;
pub const ADC_CS_EN_BIT:                            u32 = 0;
// RESULT
pub const ADC_RESULT_LOW:                           u32 = 0;
pub const ADC_RESULT_HIGH:                          u32 = 11;
// FCS
pub const ADC_FCS_THRESH_LOW:                       u32 = 24;
pub const ADC_FCS_THRESH_HIGH:                      u32 = 27;
pub const ADC_FCS_LEVEL_LOW:                        u32 = 16;
pub const ADC_FCS_LEVEL_HIGH:                       u32 = 19;
pub const ADC_FCS_OVER_BIT:                         u32 = 11;
pub const ADC_FCS_UNDER_BIT:                        u32 = 10;
pub const ADC_FCS_FULL_BIT:                         u32 = 9;
pub const ADC_FCS_EMPTY_BIT:                        u32 = 8;
pub const ADC_FCS_DREQ_EN_BIT:                      u32 = 3;
pub const ADC_FCS_ERR_BIT:                          u32 = 2;
pub const ADC_FCS_SHIFT_BIT:                        u32 = 1;
pub const ADC_FCS_EN_BIT:                           u32 = 0;
// FIFO
pub const ADC_FIFO_ERR_BIT:                         u32 = 15;
pub const ADC_FIFO_VAL_LOW:                         u32 = 0;
pub const ADC_FIFO_VAL_HIGH:                        u32 = 11;
// DIV
pub const ADC_DIV_INT_LOW:                          u32 = 8;
pub const ADC_DIV_INT_HIGH:                         u32 = 23;
pub const ADC_DIV_FRAC_LOW:                         u32 = 0;
pub const ADC_DIV_FRAC_HIGH:                        u32 = 7;
// INTR
pub const ADC_INTR_FIFO_BIT:                        u32 = 0;
// INTE
pub const ADC_INTE_FIFO_BIT:                        u32 = 0;
// INTF
pub const ADC_INTF_FIFO_BIT:                        u32 = 0;
// INTS
pub const ADC_INTS_FIFO_BIT:                        u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
