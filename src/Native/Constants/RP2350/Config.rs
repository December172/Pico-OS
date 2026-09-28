#![allow(dead_code)]
pub const GPIO_MAX_PIN:      u8  = 47;
pub const GPIO_HI_PIN_START: u8  = 32;

/// Crystal frequency driving the XOSC. Unit is in Hz.
pub const XOSC_BASE_FREQ:    u32 = 12_000_000;

/// Nominal ROSC frequency. Unit is in Hz.
pub const ROSC_BASE_FREQ:    u32 = 6_500_000;

/// PLL_SYS_FBDIV: in MHz
pub const PLL_SYS_FBDIV:     u32 = 125;
pub const PLL_SYS_POSTDIV1:  u32 = 5;
pub const PLL_SYS_POSTDIV2:  u32 = 2;

pub const PLL_USB_FBDIV:     u32 = 120;
pub const PLL_USB_POSTDIV1:  u32 = 6;
pub const PLL_USB_POSTDIV2:  u32 = 5;
