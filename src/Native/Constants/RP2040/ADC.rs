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
