#![allow(dead_code)]
// PLL_USB
pub const PLL_USB_BASE:                             u32 = 0x4002_C000;
pub const PLL_USB_CS:                               u32 = PLL_USB_BASE + 0x0;
pub const PLL_USB_PWR:                              u32 = PLL_USB_BASE + 0x4;
pub const PLL_USB_FBDIV_INT:                        u32 = PLL_USB_BASE + 0x8;
pub const PLL_USB_PRIM:                             u32 = PLL_USB_BASE + 0xC;
