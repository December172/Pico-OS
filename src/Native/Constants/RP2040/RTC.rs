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
