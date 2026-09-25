#![allow(dead_code)]
// RTC
pub const RTC_BASE:                                 u32 = 0x4005_C000;
pub const RTC_CLKDIV_M1:                            u32 = RTC_BASE + 0x0;
pub const RTC_SETUP_0:                              u32 = RTC_BASE + 0x4;
pub const RTC_SETUP_1:                              u32 = RTC_BASE + 0x8;
pub const RTC_CTRL:                                 u32 = RTC_BASE + 0xC;
pub const RTC_IRQ_SETUP_0:                          u32 = RTC_BASE + 0x10;
pub const RTC_IRQ_SETUP_1:                          u32 = RTC_BASE + 0x14;
pub const RTC_RTC_1:                                u32 = RTC_BASE + 0x18;
pub const RTC_RTC_0:                                u32 = RTC_BASE + 0x1C;
pub const RTC_INTR:                                 u32 = RTC_BASE + 0x20;
pub const RTC_INTE:                                 u32 = RTC_BASE + 0x24;
pub const RTC_INTF:                                 u32 = RTC_BASE + 0x28;
pub const RTC_INTS:                                 u32 = RTC_BASE + 0x2C;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CLKDIV_M1
pub const RTC_CLKDIV_M1_LOW:                        u32 = 0;
pub const RTC_CLKDIV_M1_HIGH:                       u32 = 15;
// SETUP_0
pub const RTC_SETUP_0_YEAR_LOW:                     u32 = 12;
pub const RTC_SETUP_0_YEAR_HIGH:                    u32 = 23;
pub const RTC_SETUP_0_MONTH_LOW:                    u32 = 8;
pub const RTC_SETUP_0_MONTH_HIGH:                   u32 = 11;
pub const RTC_SETUP_0_DAY_LOW:                      u32 = 0;
pub const RTC_SETUP_0_DAY_HIGH:                     u32 = 4;
// SETUP_1
pub const RTC_SETUP_1_DOTW_LOW:                     u32 = 24;
pub const RTC_SETUP_1_DOTW_HIGH:                    u32 = 26;
pub const RTC_SETUP_1_HOUR_LOW:                     u32 = 16;
pub const RTC_SETUP_1_HOUR_HIGH:                    u32 = 20;
pub const RTC_SETUP_1_MIN_LOW:                      u32 = 8;
pub const RTC_SETUP_1_MIN_HIGH:                     u32 = 13;
pub const RTC_SETUP_1_SEC_LOW:                      u32 = 0;
pub const RTC_SETUP_1_SEC_HIGH:                     u32 = 5;
// CTRL
pub const RTC_CTRL_FORCE_NOTLEAPYEAR_BIT:           u32 = 8;
pub const RTC_CTRL_LOAD_BIT:                        u32 = 4;
pub const RTC_CTRL_RTC_ACTIVE_BIT:                  u32 = 1;
pub const RTC_CTRL_RTC_ENABLE_BIT:                  u32 = 0;
// IRQ_SETUP_0
pub const RTC_IRQ_SETUP_0_MATCH_ACTIVE_BIT:         u32 = 29;
pub const RTC_IRQ_SETUP_0_MATCH_ENA_BIT:            u32 = 28;
pub const RTC_IRQ_SETUP_0_YEAR_ENA_BIT:             u32 = 26;
pub const RTC_IRQ_SETUP_0_MONTH_ENA_BIT:            u32 = 25;
pub const RTC_IRQ_SETUP_0_DAY_ENA_BIT:              u32 = 24;
pub const RTC_IRQ_SETUP_0_YEAR_LOW:                 u32 = 12;
pub const RTC_IRQ_SETUP_0_YEAR_HIGH:                u32 = 23;
pub const RTC_IRQ_SETUP_0_MONTH_LOW:                u32 = 8;
pub const RTC_IRQ_SETUP_0_MONTH_HIGH:               u32 = 11;
pub const RTC_IRQ_SETUP_0_DAY_LOW:                  u32 = 0;
pub const RTC_IRQ_SETUP_0_DAY_HIGH:                 u32 = 4;
// IRQ_SETUP_1
pub const RTC_IRQ_SETUP_1_DOTW_ENA_BIT:             u32 = 31;
pub const RTC_IRQ_SETUP_1_HOUR_ENA_BIT:             u32 = 30;
pub const RTC_IRQ_SETUP_1_MIN_ENA_BIT:              u32 = 29;
pub const RTC_IRQ_SETUP_1_SEC_ENA_BIT:              u32 = 28;
pub const RTC_IRQ_SETUP_1_DOTW_LOW:                 u32 = 24;
pub const RTC_IRQ_SETUP_1_DOTW_HIGH:                u32 = 26;
pub const RTC_IRQ_SETUP_1_HOUR_LOW:                 u32 = 16;
pub const RTC_IRQ_SETUP_1_HOUR_HIGH:                u32 = 20;
pub const RTC_IRQ_SETUP_1_MIN_LOW:                  u32 = 8;
pub const RTC_IRQ_SETUP_1_MIN_HIGH:                 u32 = 13;
pub const RTC_IRQ_SETUP_1_SEC_LOW:                  u32 = 0;
pub const RTC_IRQ_SETUP_1_SEC_HIGH:                 u32 = 5;
// RTC_1
pub const RTC_RTC_1_YEAR_LOW:                       u32 = 12;
pub const RTC_RTC_1_YEAR_HIGH:                      u32 = 23;
pub const RTC_RTC_1_MONTH_LOW:                      u32 = 8;
pub const RTC_RTC_1_MONTH_HIGH:                     u32 = 11;
pub const RTC_RTC_1_DAY_LOW:                        u32 = 0;
pub const RTC_RTC_1_DAY_HIGH:                       u32 = 4;
// RTC_0
pub const RTC_RTC_0_DOTW_LOW:                       u32 = 24;
pub const RTC_RTC_0_DOTW_HIGH:                      u32 = 26;
pub const RTC_RTC_0_HOUR_LOW:                       u32 = 16;
pub const RTC_RTC_0_HOUR_HIGH:                      u32 = 20;
pub const RTC_RTC_0_MIN_LOW:                        u32 = 8;
pub const RTC_RTC_0_MIN_HIGH:                       u32 = 13;
pub const RTC_RTC_0_SEC_LOW:                        u32 = 0;
pub const RTC_RTC_0_SEC_HIGH:                       u32 = 5;
// INTR
pub const RTC_INTR_RTC_BIT:                         u32 = 0;
// INTE
pub const RTC_INTE_RTC_BIT:                         u32 = 0;
// INTF
pub const RTC_INTF_RTC_BIT:                         u32 = 0;
// INTS
pub const RTC_INTS_RTC_BIT:                         u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
