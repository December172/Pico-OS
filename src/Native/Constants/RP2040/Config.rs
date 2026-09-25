#![allow(dead_code)]
pub const GPIO_MAX_PIN:      u8  = 29;
pub const GPIO_HI_PIN_START: u8  = 30;
pub const XOSC_BASE_FREQ:    u32 = 12_000_000;

/// <FREQ_OUTPUT> = XOSC_BASE_FREQ * PLL_SYS_FBDIV / (PLL_SYS_POSTDIV1 * PLL_SYS_POSTDIV2)
/// as of POSTDIVs only have 3 bits [18:16] and [14:12],
/// we can only choose POSTDIVx <= 7 :(
/// TODO: Equip tools/vcocalc.py to generate these oscillator-specific constants

/// PLL_SYS_FBDIV: in MHz
pub const PLL_SYS_FBDIV:     u32 = 125;
pub const PLL_SYS_POSTDIV1:  u32 = 5;
pub const PLL_SYS_POSTDIV2:  u32 = 2;

pub const PLL_USB_FBDIV:     u32 = 120;
pub const PLL_USB_POSTDIV1:  u32 = 6;
pub const PLL_USB_POSTDIV2:  u32 = 5;