#![allow(dead_code)]
// CLOCKS

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const CLOCKS_BASE:                              u32 = 0x4001_0000;

// CLK_GPOUT0_CTRL..CLK_GPOUT3_CTRL
pub fn CLOCKS_CLK_GPOUT_CTRL(n: u32) -> u32 {
    return CLOCKS_BASE + 0x0 + n * 0xC
}

// CLK_GPOUT0_DIV..CLK_GPOUT3_DIV
pub fn CLOCKS_CLK_GPOUT_DIV(n: u32) -> u32 {
    return CLOCKS_BASE + 0x4 + n * 0xC
}

// CLK_GPOUT0_SELECTED..CLK_GPOUT3_SELECTED
pub fn CLOCKS_CLK_GPOUT_SELECTED(n: u32) -> u32 {
    return CLOCKS_BASE + 0x8 + n * 0xC
}

pub const CLOCKS_CLK_REF_CTRL:                      u32 = CLOCKS_BASE + 0x30;
pub const CLOCKS_CLK_REF_DIV:                       u32 = CLOCKS_BASE + 0x34;
pub const CLOCKS_CLK_REF_SELECTED:                  u32 = CLOCKS_BASE + 0x38;
pub const CLOCKS_CLK_SYS_CTRL:                      u32 = CLOCKS_BASE + 0x3C;
pub const CLOCKS_CLK_SYS_DIV:                       u32 = CLOCKS_BASE + 0x40;
pub const CLOCKS_CLK_SYS_SELECTED:                  u32 = CLOCKS_BASE + 0x44;
pub const CLOCKS_CLK_PERI_CTRL:                     u32 = CLOCKS_BASE + 0x48;
pub const CLOCKS_CLK_PERI_DIV:                      u32 = CLOCKS_BASE + 0x4C;
pub const CLOCKS_CLK_PERI_SELECTED:                 u32 = CLOCKS_BASE + 0x50;
pub const CLOCKS_CLK_HSTX_CTRL:                     u32 = CLOCKS_BASE + 0x54;
pub const CLOCKS_CLK_HSTX_DIV:                      u32 = CLOCKS_BASE + 0x58;
pub const CLOCKS_CLK_HSTX_SELECTED:                 u32 = CLOCKS_BASE + 0x5C;
pub const CLOCKS_CLK_USB_CTRL:                      u32 = CLOCKS_BASE + 0x60;
pub const CLOCKS_CLK_USB_DIV:                       u32 = CLOCKS_BASE + 0x64;
pub const CLOCKS_CLK_USB_SELECTED:                  u32 = CLOCKS_BASE + 0x68;
pub const CLOCKS_CLK_ADC_CTRL:                      u32 = CLOCKS_BASE + 0x6C;
pub const CLOCKS_CLK_ADC_DIV:                       u32 = CLOCKS_BASE + 0x70;
pub const CLOCKS_CLK_ADC_SELECTED:                  u32 = CLOCKS_BASE + 0x74;
pub const CLOCKS_DFTCLK_XOSC_CTRL:                  u32 = CLOCKS_BASE + 0x78;
pub const CLOCKS_DFTCLK_ROSC_CTRL:                  u32 = CLOCKS_BASE + 0x7C;
pub const CLOCKS_DFTCLK_LPOSC_CTRL:                 u32 = CLOCKS_BASE + 0x80;
pub const CLOCKS_CLK_SYS_RESUS_CTRL:                u32 = CLOCKS_BASE + 0x84;
pub const CLOCKS_CLK_SYS_RESUS_STATUS:              u32 = CLOCKS_BASE + 0x88;
pub const CLOCKS_FC0_REF_KHZ:                       u32 = CLOCKS_BASE + 0x8C;
pub const CLOCKS_FC0_MIN_KHZ:                       u32 = CLOCKS_BASE + 0x90;
pub const CLOCKS_FC0_MAX_KHZ:                       u32 = CLOCKS_BASE + 0x94;
pub const CLOCKS_FC0_DELAY:                         u32 = CLOCKS_BASE + 0x98;
pub const CLOCKS_FC0_INTERVAL:                      u32 = CLOCKS_BASE + 0x9C;
pub const CLOCKS_FC0_SRC:                           u32 = CLOCKS_BASE + 0xA0;
pub const CLOCKS_FC0_STATUS:                        u32 = CLOCKS_BASE + 0xA4;
pub const CLOCKS_FC0_RESULT:                        u32 = CLOCKS_BASE + 0xA8;
// WAKE_EN0..WAKE_EN1
pub fn CLOCKS_WAKE_EN(n: u32) -> u32 {
    return CLOCKS_BASE + 0xAC + n * 0x4
}

// SLEEP_EN0..SLEEP_EN1
pub fn CLOCKS_SLEEP_EN(ep: u32) -> u32 {
    return CLOCKS_BASE + 0xB4 + ep * 0x4
}

// ENABLED0..ENABLED1
pub fn CLOCKS_ENABLED(n: u32) -> u32 {
    return CLOCKS_BASE + 0xBC + n * 0x4
}

pub const CLOCKS_INTR:                              u32 = CLOCKS_BASE + 0xC4;
pub const CLOCKS_INTE:                              u32 = CLOCKS_BASE + 0xC8;
pub const CLOCKS_INTF:                              u32 = CLOCKS_BASE + 0xCC;
pub const CLOCKS_INTS:                              u32 = CLOCKS_BASE + 0xD0;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CLK_GPOUT0_CTRL, CLK_GPOUT1_CTRL, CLK_GPOUT2_CTRL, CLK_GPOUT3_CTRL
pub const CLOCKS_CLK_GPOUT_CTRL_ENABLED_BIT:        u32 = 28;
pub const CLOCKS_CLK_GPOUT_CTRL_NUDGE_BIT:          u32 = 20;
pub const CLOCKS_CLK_GPOUT_CTRL_PHASE_LOW:          u32 = 16;
pub const CLOCKS_CLK_GPOUT_CTRL_PHASE_HIGH:         u32 = 17;
pub const CLOCKS_CLK_GPOUT_CTRL_DC50_BIT:           u32 = 12;
pub const CLOCKS_CLK_GPOUT_CTRL_ENABLE_BIT:         u32 = 11;
pub const CLOCKS_CLK_GPOUT_CTRL_KILL_BIT:           u32 = 10;
pub const CLOCKS_CLK_GPOUT_CTRL_AUXSRC_LOW:         u32 = 5;
pub const CLOCKS_CLK_GPOUT_CTRL_AUXSRC_HIGH:        u32 = 8;

// CLK_GPOUT0_DIV, CLK_GPOUT1_DIV, CLK_GPOUT2_DIV, CLK_GPOUT3_DIV, CLK_SYS_DIV
pub const CLOCKS_CLK_DIV_INT_LOW:                   u32 = 16;
pub const CLOCKS_CLK_DIV_INT_HIGH:                  u32 = 31;
pub const CLOCKS_CLK_DIV_FRAC_LOW:                  u32 = 0;
pub const CLOCKS_CLK_DIV_FRAC_HIGH:                 u32 = 15;

// CLK_GPOUT0_SELECTED, CLK_GPOUT1_SELECTED, CLK_GPOUT2_SELECTED, CLK_GPOUT3_SELECTED
pub const CLOCKS_CLK_GPOUT_SELECTED_BIT:            u32 = 0;

// CLK_REF_CTRL
pub const CLOCKS_CLK_REF_CTRL_AUXSRC_LOW:           u32 = 5;
pub const CLOCKS_CLK_REF_CTRL_AUXSRC_HIGH:          u32 = 6;
pub const CLOCKS_CLK_REF_CTRL_SRC_LOW:              u32 = 0;
pub const CLOCKS_CLK_REF_CTRL_SRC_HIGH:             u32 = 1;

// CLK_REF_DIV
pub const CLOCKS_CLK_REF_DIV_INT_LOW:               u32 = 16;
pub const CLOCKS_CLK_REF_DIV_INT_HIGH:              u32 = 23;

// CLK_REF_SELECTED
pub const CLOCKS_CLK_REF_SELECTED_LOW:              u32 = 0;
pub const CLOCKS_CLK_REF_SELECTED_HIGH:             u32 = 3;

// CLK_SYS_CTRL
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_LOW:           u32 = 5;
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_HIGH:          u32 = 7;
pub const CLOCKS_CLK_SYS_CTRL_SRC_BIT:              u32 = 0;

// CLK_SYS_SELECTED
pub const CLOCKS_CLK_SYS_SELECTED_LOW:              u32 = 0;
pub const CLOCKS_CLK_SYS_SELECTED_HIGH:             u32 = 1;

// CLK_PERI_CTRL
pub const CLOCKS_CLK_PERI_CTRL_ENABLED_BIT:         u32 = 28;
pub const CLOCKS_CLK_PERI_CTRL_ENABLE_BIT:          u32 = 11;
pub const CLOCKS_CLK_PERI_CTRL_KILL_BIT:            u32 = 10;
pub const CLOCKS_CLK_PERI_CTRL_AUXSRC_LOW:          u32 = 5;
pub const CLOCKS_CLK_PERI_CTRL_AUXSRC_HIGH:         u32 = 7;

// CLK_PERI_DIV
pub const CLOCKS_CLK_PERI_DIV_INT_LOW:              u32 = 16;
pub const CLOCKS_CLK_PERI_DIV_INT_HIGH:             u32 = 17;

// CLK_HSTX_DIV
pub const CLOCKS_CLK_HSTX_DIV_INT_LOW:              u32 = 16;
pub const CLOCKS_CLK_HSTX_DIV_INT_HIGH:             u32 = 17;

// CLK_PERI_SELECTED
pub const CLOCKS_CLK_PERI_SELECTED_BIT:             u32 = 0;

// CLK_HSTX_CTRL, CLK_USB_CTRL, CLK_ADC_CTRL
pub const CLOCKS_CLK_CTRL_ENABLED_BIT:              u32 = 28;
pub const CLOCKS_CLK_CTRL_NUDGE_BIT:                u32 = 20;
pub const CLOCKS_CLK_CTRL_PHASE_LOW:                u32 = 16;
pub const CLOCKS_CLK_CTRL_PHASE_HIGH:               u32 = 17;
pub const CLOCKS_CLK_CTRL_ENABLE_BIT:               u32 = 11;
pub const CLOCKS_CLK_CTRL_KILL_BIT:                 u32 = 10;
pub const CLOCKS_CLK_CTRL_AUXSRC_LOW:               u32 = 5;
pub const CLOCKS_CLK_CTRL_AUXSRC_HIGH:              u32 = 7;

// CLK_HSTX_SELECTED
pub const CLOCKS_CLK_HSTX_SELECTED_BIT:             u32 = 0;

// CLK_USB_DIV
pub const CLOCKS_CLK_USB_DIV_INT_LOW:               u32 = 16;
pub const CLOCKS_CLK_USB_DIV_INT_HIGH:              u32 = 19;

// CLK_ADC_DIV
pub const CLOCKS_CLK_ADC_DIV_INT_LOW:               u32 = 16;
pub const CLOCKS_CLK_ADC_DIV_INT_HIGH:              u32 = 19;

// CLK_USB_SELECTED
pub const CLOCKS_CLK_USB_SELECTED_BIT:              u32 = 0;

// CLK_ADC_SELECTED
pub const CLOCKS_CLK_ADC_SELECTED_BIT:              u32 = 0;

// DFTCLK_XOSC_CTRL, DFTCLK_ROSC_CTRL, DFTCLK_LPOSC_CTRL
pub const CLOCKS_DFTCLK_CTRL_SRC_LOW:               u32 = 0;
pub const CLOCKS_DFTCLK_CTRL_SRC_HIGH:              u32 = 1;

// CLK_SYS_RESUS_CTRL
pub const CLOCKS_CLK_SYS_RESUS_CTRL_CLEAR_BIT:      u32 = 16;
pub const CLOCKS_CLK_SYS_RESUS_CTRL_FRCE_BIT:       u32 = 12;
pub const CLOCKS_CLK_SYS_RESUS_CTRL_ENABLE_BIT:     u32 = 8;
pub const CLOCKS_CLK_SYS_RESUS_CTRL_TIMEOUT_LOW:    u32 = 0;
pub const CLOCKS_CLK_SYS_RESUS_CTRL_TIMEOUT_HIGH:   u32 = 7;

// CLK_SYS_RESUS_STATUS
pub const CLOCKS_CLK_SYS_RESUS_STATUS_RESUSSED_BIT: u32 = 0;

// FC0_REF_KHZ
pub const CLOCKS_FC0_REF_KHZ_LOW:                   u32 = 0;
pub const CLOCKS_FC0_REF_KHZ_HIGH:                  u32 = 19;

// FC0_MIN_KHZ
pub const CLOCKS_FC0_MIN_KHZ_LOW:                   u32 = 0;
pub const CLOCKS_FC0_MIN_KHZ_HIGH:                  u32 = 24;

// FC0_MAX_KHZ
pub const CLOCKS_FC0_MAX_KHZ_LOW:                   u32 = 0;
pub const CLOCKS_FC0_MAX_KHZ_HIGH:                  u32 = 24;

// FC0_DELAY
pub const CLOCKS_FC0_DELAY_LOW:                     u32 = 0;
pub const CLOCKS_FC0_DELAY_HIGH:                    u32 = 2;

// FC0_INTERVAL
pub const CLOCKS_FC0_INTERVAL_LOW:                  u32 = 0;
pub const CLOCKS_FC0_INTERVAL_HIGH:                 u32 = 3;

// FC0_SRC
pub const CLOCKS_FC0_SRC_LOW:                       u32 = 0;
pub const CLOCKS_FC0_SRC_HIGH:                      u32 = 7;

// FC0_STATUS
pub const CLOCKS_FC0_STATUS_DIED_BIT:               u32 = 28;
pub const CLOCKS_FC0_STATUS_FAST_BIT:               u32 = 24;
pub const CLOCKS_FC0_STATUS_SLOW_BIT:               u32 = 20;
pub const CLOCKS_FC0_STATUS_FAIL_BIT:               u32 = 16;
pub const CLOCKS_FC0_STATUS_WAITING_BIT:            u32 = 12;
pub const CLOCKS_FC0_STATUS_RUNNING_BIT:            u32 = 8;
pub const CLOCKS_FC0_STATUS_DONE_BIT:               u32 = 4;
pub const CLOCKS_FC0_STATUS_PASS_BIT:               u32 = 0;

// FC0_RESULT
pub const CLOCKS_FC0_RESULT_KHZ_LOW:                u32 = 5;
pub const CLOCKS_FC0_RESULT_KHZ_HIGH:               u32 = 29;
pub const CLOCKS_FC0_RESULT_FRAC_LOW:               u32 = 0;
pub const CLOCKS_FC0_RESULT_FRAC_HIGH:              u32 = 4;

// WAKE_EN0
pub const CLOCKS_WAKE_EN0_CLK_SYS_SIO_BIT:          u32 = 31;
pub const CLOCKS_WAKE_EN0_CLK_SYS_SHA256_BIT:       u32 = 30;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PSM_BIT:          u32 = 29;
pub const CLOCKS_WAKE_EN0_CLK_SYS_ROSC_BIT:         u32 = 28;
pub const CLOCKS_WAKE_EN0_CLK_SYS_ROM_BIT:          u32 = 27;
pub const CLOCKS_WAKE_EN0_CLK_SYS_RESETS_BIT:       u32 = 26;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PWM_BIT:          u32 = 25;
pub const CLOCKS_WAKE_EN0_CLK_SYS_POWMAN_BIT:       u32 = 24;
pub const CLOCKS_WAKE_EN0_CLK_REF_POWMAN_BIT:       u32 = 23;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PLL_USB_BIT:      u32 = 22;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PLL_SYS_BIT:      u32 = 21;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PIO2_BIT:         u32 = 20;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PIO1_BIT:         u32 = 19;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PIO0_BIT:         u32 = 18;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PADS_BIT:         u32 = 17;
pub const CLOCKS_WAKE_EN0_CLK_SYS_OTP_BIT:          u32 = 16;
pub const CLOCKS_WAKE_EN0_CLK_REF_OTP_BIT:          u32 = 15;
pub const CLOCKS_WAKE_EN0_CLK_SYS_JTAG_BIT:         u32 = 14;
pub const CLOCKS_WAKE_EN0_CLK_SYS_IO_BIT:           u32 = 13;
pub const CLOCKS_WAKE_EN0_CLK_SYS_I2C1_BIT:         u32 = 12;
pub const CLOCKS_WAKE_EN0_CLK_SYS_I2C0_BIT:         u32 = 11;
pub const CLOCKS_WAKE_EN0_CLK_SYS_HSTX_BIT:         u32 = 10;
pub const CLOCKS_WAKE_EN0_CLK_HSTX_BIT:             u32 = 9;
pub const CLOCKS_WAKE_EN0_CLK_SYS_GLITCH_DETECTOR_BIT:u32 = 8;
pub const CLOCKS_WAKE_EN0_CLK_SYS_DMA_BIT:          u32 = 7;
pub const CLOCKS_WAKE_EN0_CLK_SYS_BUSFABRIC_BIT:    u32 = 6;
pub const CLOCKS_WAKE_EN0_CLK_SYS_BUSCTRL_BIT:      u32 = 5;
pub const CLOCKS_WAKE_EN0_CLK_SYS_BOOTRAM_BIT:      u32 = 4;
pub const CLOCKS_WAKE_EN0_CLK_SYS_ADC_BIT:          u32 = 3;
pub const CLOCKS_WAKE_EN0_CLK_ADC_BIT:              u32 = 2;
pub const CLOCKS_WAKE_EN0_CLK_SYS_ACCESSCTRL_BIT:   u32 = 1;
pub const CLOCKS_WAKE_EN0_CLK_SYS_CLOCKS_BIT:       u32 = 0;

// SLEEP_EN0
pub const CLOCKS_SLEEP_EN0_CLK_SYS_SIO_BIT:         u32 = 31;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_SHA256_BIT:      u32 = 30;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PSM_BIT:         u32 = 29;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_ROSC_BIT:        u32 = 28;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_ROM_BIT:         u32 = 27;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_RESETS_BIT:      u32 = 26;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PWM_BIT:         u32 = 25;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_POWMAN_BIT:      u32 = 24;
pub const CLOCKS_SLEEP_EN0_CLK_REF_POWMAN_BIT:      u32 = 23;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_USB_BIT:     u32 = 22;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_SYS_BIT:     u32 = 21;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PIO2_BIT:        u32 = 20;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PIO1_BIT:        u32 = 19;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PIO0_BIT:        u32 = 18;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PADS_BIT:        u32 = 17;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_OTP_BIT:         u32 = 16;
pub const CLOCKS_SLEEP_EN0_CLK_REF_OTP_BIT:         u32 = 15;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_JTAG_BIT:        u32 = 14;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_IO_BIT:          u32 = 13;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_I2C1_BIT:        u32 = 12;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_I2C0_BIT:        u32 = 11;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_HSTX_BIT:        u32 = 10;
pub const CLOCKS_SLEEP_EN0_CLK_HSTX_BIT:            u32 = 9;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_GLITCH_DETECTOR_BIT:u32 = 8;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_DMA_BIT:         u32 = 7;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_BUSFABRIC_BIT:   u32 = 6;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_BUSCTRL_BIT:     u32 = 5;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_BOOTRAM_BIT:     u32 = 4;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_ADC_BIT:         u32 = 3;
pub const CLOCKS_SLEEP_EN0_CLK_ADC_BIT:             u32 = 2;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_ACCESSCTRL_BIT:  u32 = 1;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_CLOCKS_BIT:      u32 = 0;

// ENABLED0
pub const CLOCKS_ENABLED0_CLK_SYS_SIO_BIT:          u32 = 31;
pub const CLOCKS_ENABLED0_CLK_SYS_SHA256_BIT:       u32 = 30;
pub const CLOCKS_ENABLED0_CLK_SYS_PSM_BIT:          u32 = 29;
pub const CLOCKS_ENABLED0_CLK_SYS_ROSC_BIT:         u32 = 28;
pub const CLOCKS_ENABLED0_CLK_SYS_ROM_BIT:          u32 = 27;
pub const CLOCKS_ENABLED0_CLK_SYS_RESETS_BIT:       u32 = 26;
pub const CLOCKS_ENABLED0_CLK_SYS_PWM_BIT:          u32 = 25;
pub const CLOCKS_ENABLED0_CLK_SYS_POWMAN_BIT:       u32 = 24;
pub const CLOCKS_ENABLED0_CLK_REF_POWMAN_BIT:       u32 = 23;
pub const CLOCKS_ENABLED0_CLK_SYS_PLL_USB_BIT:      u32 = 22;
pub const CLOCKS_ENABLED0_CLK_SYS_PLL_SYS_BIT:      u32 = 21;
pub const CLOCKS_ENABLED0_CLK_SYS_PIO2_BIT:         u32 = 20;
pub const CLOCKS_ENABLED0_CLK_SYS_PIO1_BIT:         u32 = 19;
pub const CLOCKS_ENABLED0_CLK_SYS_PIO0_BIT:         u32 = 18;
pub const CLOCKS_ENABLED0_CLK_SYS_PADS_BIT:         u32 = 17;
pub const CLOCKS_ENABLED0_CLK_SYS_OTP_BIT:          u32 = 16;
pub const CLOCKS_ENABLED0_CLK_REF_OTP_BIT:          u32 = 15;
pub const CLOCKS_ENABLED0_CLK_SYS_JTAG_BIT:         u32 = 14;
pub const CLOCKS_ENABLED0_CLK_SYS_IO_BIT:           u32 = 13;
pub const CLOCKS_ENABLED0_CLK_SYS_I2C1_BIT:         u32 = 12;
pub const CLOCKS_ENABLED0_CLK_SYS_I2C0_BIT:         u32 = 11;
pub const CLOCKS_ENABLED0_CLK_SYS_HSTX_BIT:         u32 = 10;
pub const CLOCKS_ENABLED0_CLK_HSTX_BIT:             u32 = 9;
pub const CLOCKS_ENABLED0_CLK_SYS_GLITCH_DETECTOR_BIT:u32 = 8;
pub const CLOCKS_ENABLED0_CLK_SYS_DMA_BIT:          u32 = 7;
pub const CLOCKS_ENABLED0_CLK_SYS_BUSFABRIC_BIT:    u32 = 6;
pub const CLOCKS_ENABLED0_CLK_SYS_BUSCTRL_BIT:      u32 = 5;
pub const CLOCKS_ENABLED0_CLK_SYS_BOOTRAM_BIT:      u32 = 4;
pub const CLOCKS_ENABLED0_CLK_SYS_ADC_BIT:          u32 = 3;
pub const CLOCKS_ENABLED0_CLK_ADC_BIT:              u32 = 2;
pub const CLOCKS_ENABLED0_CLK_SYS_ACCESSCTRL_BIT:   u32 = 1;
pub const CLOCKS_ENABLED0_CLK_SYS_CLOCKS_BIT:       u32 = 0;

// WAKE_EN1
pub const CLOCKS_WAKE_EN1_CLK_SYS_XOSC_BIT:         u32 = 30;
pub const CLOCKS_WAKE_EN1_CLK_SYS_XIP_BIT:          u32 = 29;
pub const CLOCKS_WAKE_EN1_CLK_SYS_WATCHDOG_BIT:     u32 = 28;
pub const CLOCKS_WAKE_EN1_CLK_USB_BIT:              u32 = 27;
pub const CLOCKS_WAKE_EN1_CLK_SYS_USBCTRL_BIT:      u32 = 26;
pub const CLOCKS_WAKE_EN1_CLK_SYS_UART1_BIT:        u32 = 25;
pub const CLOCKS_WAKE_EN1_CLK_SYS_UART0_BIT:        u32 = 23;
pub const CLOCKS_WAKE_EN1_CLK_PERI_UART1_BIT:       u32 = 24;
pub const CLOCKS_WAKE_EN1_CLK_PERI_UART0_BIT:       u32 = 22;
pub const CLOCKS_WAKE_EN1_CLK_SYS_TRNG_BIT:         u32 = 21;
pub const CLOCKS_WAKE_EN1_CLK_SYS_TIMER1_BIT:       u32 = 20;
pub const CLOCKS_WAKE_EN1_CLK_SYS_TIMER0_BIT:       u32 = 19;
pub const CLOCKS_WAKE_EN1_CLK_SYS_TICKS_BIT:        u32 = 18;
pub const CLOCKS_WAKE_EN1_CLK_REF_TICKS_BIT:        u32 = 17;
pub const CLOCKS_WAKE_EN1_CLK_SYS_TBMAN_BIT:        u32 = 16;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SYSINFO_BIT:      u32 = 15;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SYSCFG_BIT:       u32 = 14;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM9_BIT:        u32 = 13;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM8_BIT:        u32 = 12;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM7_BIT:        u32 = 11;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM6_BIT:        u32 = 10;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM5_BIT:        u32 = 9;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM4_BIT:        u32 = 8;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM3_BIT:        u32 = 7;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM2_BIT:        u32 = 6;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM1_BIT:        u32 = 5;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM0_BIT:        u32 = 4;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SPI1_BIT:         u32 = 3;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SPI0_BIT:         u32 = 1;
pub const CLOCKS_WAKE_EN1_CLK_PERI_SPI1_BIT:        u32 = 2;
pub const CLOCKS_WAKE_EN1_CLK_PERI_SPI0_BIT:        u32 = 0;

// SLEEP_EN1
pub const CLOCKS_SLEEP_EN1_CLK_SYS_XOSC_BIT:        u32 = 30;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_XIP_BIT:         u32 = 29;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_WATCHDOG_BIT:    u32 = 28;
pub const CLOCKS_SLEEP_EN1_CLK_USB_BIT:             u32 = 27;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_USBCTRL_BIT:     u32 = 26;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_UART1_BIT:       u32 = 25;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_UART0_BIT:       u32 = 23;
pub const CLOCKS_SLEEP_EN1_CLK_PERI_UART1_BIT:      u32 = 24;
pub const CLOCKS_SLEEP_EN1_CLK_PERI_UART0_BIT:      u32 = 22;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_TRNG_BIT:        u32 = 21;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_TIMER1_BIT:      u32 = 20;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_TIMER0_BIT:      u32 = 19;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_TICKS_BIT:       u32 = 18;
pub const CLOCKS_SLEEP_EN1_CLK_REF_TICKS_BIT:       u32 = 17;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_TBMAN_BIT:       u32 = 16;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SYSINFO_BIT:     u32 = 15;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SYSCFG_BIT:      u32 = 14;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM9_BIT:       u32 = 13;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM8_BIT:       u32 = 12;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM7_BIT:       u32 = 11;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM6_BIT:       u32 = 10;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM5_BIT:       u32 = 9;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM4_BIT:       u32 = 8;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM3_BIT:       u32 = 7;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM2_BIT:       u32 = 6;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM1_BIT:       u32 = 5;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM0_BIT:       u32 = 4;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SPI1_BIT:        u32 = 3;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SPI0_BIT:        u32 = 1;
pub const CLOCKS_SLEEP_EN1_CLK_PERI_SPI1_BIT:       u32 = 2;
pub const CLOCKS_SLEEP_EN1_CLK_PERI_SPI0_BIT:       u32 = 0;

// ENABLED1
pub const CLOCKS_ENABLED1_CLK_SYS_XOSC_BIT:         u32 = 30;
pub const CLOCKS_ENABLED1_CLK_SYS_XIP_BIT:          u32 = 29;
pub const CLOCKS_ENABLED1_CLK_SYS_WATCHDOG_BIT:     u32 = 28;
pub const CLOCKS_ENABLED1_CLK_USB_BIT:              u32 = 27;
pub const CLOCKS_ENABLED1_CLK_SYS_USBCTRL_BIT:      u32 = 26;
pub const CLOCKS_ENABLED1_CLK_SYS_UART1_BIT:        u32 = 25;
pub const CLOCKS_ENABLED1_CLK_SYS_UART0_BIT:        u32 = 23;
pub const CLOCKS_ENABLED1_CLK_PERI_UART1_BIT:       u32 = 24;
pub const CLOCKS_ENABLED1_CLK_PERI_UART0_BIT:       u32 = 22;
pub const CLOCKS_ENABLED1_CLK_SYS_TRNG_BIT:         u32 = 21;
pub const CLOCKS_ENABLED1_CLK_SYS_TIMER1_BIT:       u32 = 20;
pub const CLOCKS_ENABLED1_CLK_SYS_TIMER0_BIT:       u32 = 19;
pub const CLOCKS_ENABLED1_CLK_SYS_TICKS_BIT:        u32 = 18;
pub const CLOCKS_ENABLED1_CLK_REF_TICKS_BIT:        u32 = 17;
pub const CLOCKS_ENABLED1_CLK_SYS_TBMAN_BIT:        u32 = 16;
pub const CLOCKS_ENABLED1_CLK_SYS_SYSINFO_BIT:      u32 = 15;
pub const CLOCKS_ENABLED1_CLK_SYS_SYSCFG_BIT:       u32 = 14;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM9_BIT:        u32 = 13;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM8_BIT:        u32 = 12;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM7_BIT:        u32 = 11;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM6_BIT:        u32 = 10;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM5_BIT:        u32 = 9;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM4_BIT:        u32 = 8;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM3_BIT:        u32 = 7;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM2_BIT:        u32 = 6;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM1_BIT:        u32 = 5;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM0_BIT:        u32 = 4;
pub const CLOCKS_ENABLED1_CLK_SYS_SPI1_BIT:         u32 = 3;
pub const CLOCKS_ENABLED1_CLK_SYS_SPI0_BIT:         u32 = 1;
pub const CLOCKS_ENABLED1_CLK_PERI_SPI1_BIT:        u32 = 2;
pub const CLOCKS_ENABLED1_CLK_PERI_SPI0_BIT:        u32 = 0;

// INTR
pub const CLOCKS_INTR_CLK_SYS_RESUS_BIT:            u32 = 0;

// INTE
pub const CLOCKS_INTE_CLK_SYS_RESUS_BIT:            u32 = 0;

// INTF
pub const CLOCKS_INTF_CLK_SYS_RESUS_BIT:            u32 = 0;

// INTS
pub const CLOCKS_INTS_CLK_SYS_RESUS_BIT:            u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CLK_GPOUT0_CTRL, CLK_GPOUT1_CTRL, CLK_GPOUT2_CTRL, CLK_GPOUT3_CTRL.AUXSRC [8:5]
pub const CLOCKS_CLK_GPOUT_PLL_SYS_AUXSOURCE:       u32 = 0x0;
pub const CLOCKS_CLK_GPOUT_GPIN0_AUXSOURCE:         u32 = 0x1;
pub const CLOCKS_CLK_GPOUT_GPIN1_AUXSOURCE:         u32 = 0x2;
pub const CLOCKS_CLK_GPOUT_PLL_USB_AUXSOURCE:       u32 = 0x3;
pub const CLOCKS_CLK_GPOUT_PLL_USB_PRIMARY_REF_OPCG_AUXSOURCE:u32 = 0x4;
pub const CLOCKS_CLK_GPOUT_ROSC_AUXSOURCE:          u32 = 0x5;
pub const CLOCKS_CLK_GPOUT_XOSC_AUXSOURCE:          u32 = 0x6;
pub const CLOCKS_CLK_GPOUT_LPOSC_AUXSOURCE:         u32 = 0x7;
pub const CLOCKS_CLK_GPOUT_CLK_SYS_AUXSOURCE:       u32 = 0x8;
pub const CLOCKS_CLK_GPOUT_CLK_USB_AUXSOURCE:       u32 = 0x9;
pub const CLOCKS_CLK_GPOUT_CLK_ADC_AUXSOURCE:       u32 = 0xA;
pub const CLOCKS_CLK_GPOUT_CLK_REF_AUXSOURCE:       u32 = 0xB;
pub const CLOCKS_CLK_GPOUT_CLK_PERI_AUXSOURCE:      u32 = 0xC;
pub const CLOCKS_CLK_GPOUT_CLK_HSTX_AUXSOURCE:      u32 = 0xD;
pub const CLOCKS_CLK_GPOUT_OTP_CLK2FC_AUXSOURCE:    u32 = 0xE;
pub const CLOCKS_CLK_GPOUT_ROSC_PH_AUXSOURCE:       u32 = 0x5;

// CLK_REF_CTRL.AUXSRC [6:5]
pub const CLOCKS_CLK_REF_PLL_USB_AUXSOURCE:         u32 = 0x0;
pub const CLOCKS_CLK_REF_GPIN0_AUXSOURCE:           u32 = 0x1;
pub const CLOCKS_CLK_REF_GPIN1_AUXSOURCE:           u32 = 0x2;
pub const CLOCKS_CLK_REF_PLL_USB_PRIMARY_REF_OPCG_AUXSOURCE:u32 = 0x3;

// CLK_REF_CTRL.SRC [1:0]
pub const CLOCKS_CLK_REF_ROSC_PH_SRC:               u32 = 0x0;
pub const CLOCKS_CLK_REF_CLK_REF_AUX_SRC:           u32 = 0x1;
pub const CLOCKS_CLK_REF_XOSC_SRC:                  u32 = 0x2;
pub const CLOCKS_CLK_REF_LPOSC_SRC:                 u32 = 0x3;

// CLK_SYS_CTRL.AUXSRC [7:5]
pub const CLOCKS_CLK_SYS_PLL_SYS_AUXSOURCE:         u32 = 0x0;
pub const CLOCKS_CLK_SYS_PLL_USB_AUXSOURCE:         u32 = 0x1;
pub const CLOCKS_CLK_SYS_ROSC_AUXSOURCE:            u32 = 0x2;
pub const CLOCKS_CLK_SYS_XOSC_AUXSOURCE:            u32 = 0x3;
pub const CLOCKS_CLK_SYS_GPIN0_AUXSOURCE:           u32 = 0x4;
pub const CLOCKS_CLK_SYS_GPIN1_AUXSOURCE:           u32 = 0x5;

// CLK_SYS_CTRL.SRC [0:0]
pub const CLOCKS_CLK_SYS_CLK_REF_SRC:               u32 = 0x0;
pub const CLOCKS_CLK_SYS_CLK_SYS_AUX_SRC:           u32 = 0x1;

// CLK_PERI_CTRL.AUXSRC [7:5]
pub const CLOCKS_CLK_PERI_CLK_SYS_AUXSOURCE:        u32 = 0x0;
pub const CLOCKS_CLK_PERI_PLL_SYS_AUXSOURCE:        u32 = 0x1;
pub const CLOCKS_CLK_PERI_PLL_USB_AUXSOURCE:        u32 = 0x2;
pub const CLOCKS_CLK_PERI_ROSC_PH_AUXSOURCE:        u32 = 0x3;
pub const CLOCKS_CLK_PERI_XOSC_AUXSOURCE:           u32 = 0x4;
pub const CLOCKS_CLK_PERI_GPIN0_AUXSOURCE:          u32 = 0x5;
pub const CLOCKS_CLK_PERI_GPIN1_AUXSOURCE:          u32 = 0x6;

// CLK_HSTX_CTRL.AUXSRC [7:5]
pub const CLOCKS_CLK_HSTX_CLK_SYS_AUXSOURCE:        u32 = 0x0;
pub const CLOCKS_CLK_HSTX_PLL_SYS_AUXSOURCE:        u32 = 0x1;
pub const CLOCKS_CLK_HSTX_PLL_USB_AUXSOURCE:        u32 = 0x2;
pub const CLOCKS_CLK_HSTX_GPIN0_AUXSOURCE:          u32 = 0x3;
pub const CLOCKS_CLK_HSTX_GPIN1_AUXSOURCE:          u32 = 0x4;

// CLK_USB_CTRL.AUXSRC [7:5]
pub const CLOCKS_CLK_USB_PLL_USB_AUXSOURCE:         u32 = 0x0;
pub const CLOCKS_CLK_USB_PLL_SYS_AUXSOURCE:         u32 = 0x1;
pub const CLOCKS_CLK_USB_ROSC_PH_AUXSOURCE:         u32 = 0x2;
pub const CLOCKS_CLK_USB_XOSC_AUXSOURCE:            u32 = 0x3;
pub const CLOCKS_CLK_USB_GPIN0_AUXSOURCE:           u32 = 0x4;
pub const CLOCKS_CLK_USB_GPIN1_AUXSOURCE:           u32 = 0x5;

// CLK_ADC_CTRL.AUXSRC [7:5]
pub const CLOCKS_CLK_ADC_PLL_USB_AUXSOURCE:         u32 = 0x0;
pub const CLOCKS_CLK_ADC_PLL_SYS_AUXSOURCE:         u32 = 0x1;
pub const CLOCKS_CLK_ADC_ROSC_PH_AUXSOURCE:         u32 = 0x2;
pub const CLOCKS_CLK_ADC_XOSC_AUXSOURCE:            u32 = 0x3;
pub const CLOCKS_CLK_ADC_GPIN0_AUXSOURCE:           u32 = 0x4;
pub const CLOCKS_CLK_ADC_GPIN1_AUXSOURCE:           u32 = 0x5;

// DFTCLK_XOSC_CTRL.SRC [1:0]
pub const CLOCKS_CLK_DFTCLK_XOSC_NULL_SRC:          u32 = 0x0;
pub const CLOCKS_CLK_DFTCLK_XOSC_PLL_USB_PRIMARY_SRC:u32 = 0x1;
pub const CLOCKS_CLK_DFTCLK_XOSC_GPIN0_SRC:         u32 = 0x2;

// DFTCLK_ROSC_CTRL.SRC [1:0]
pub const CLOCKS_CLK_DFTCLK_ROSC_NULL_SRC:          u32 = 0x0;
pub const CLOCKS_CLK_DFTCLK_ROSC_PLL_SYS_PRIMARY_ROSC_SRC:u32 = 0x1;
pub const CLOCKS_CLK_DFTCLK_ROSC_GPIN1_SRC:         u32 = 0x2;

// DFTCLK_LPOSC_CTRL.SRC [1:0]
pub const CLOCKS_CLK_DFTCLK_LPOSC_NULL_SRC:         u32 = 0x0;
pub const CLOCKS_CLK_DFTCLK_LPOSC_PLL_USB_PRIMARY_LPOSC_SRC:u32 = 0x1;
pub const CLOCKS_CLK_DFTCLK_LPOSC_GPIN1_SRC:        u32 = 0x2;

// FC0_SRC: FC_SRC
pub const CLOCKS_FC0_SRC_NULL:                      u32 = 0x0;
pub const CLOCKS_FC0_SRC_PLL_SYS_CLKSRC_PRIMARY:    u32 = 0x1;
pub const CLOCKS_FC0_SRC_PLL_USB_CLKSRC_PRIMARY:    u32 = 0x2;
pub const CLOCKS_FC0_SRC_ROSC_CLKSRC:               u32 = 0x3;
pub const CLOCKS_FC0_SRC_ROSC_CLKSRC_PH:            u32 = 0x4;
pub const CLOCKS_FC0_SRC_XOSC_CLKSRC:               u32 = 0x5;
pub const CLOCKS_FC0_SRC_CLKSRC_GPIN0:              u32 = 0x6;
pub const CLOCKS_FC0_SRC_CLKSRC_GPIN1:              u32 = 0x7;
pub const CLOCKS_FC0_SRC_CLK_REF:                   u32 = 0x8;
pub const CLOCKS_FC0_SRC_CLK_SYS:                   u32 = 0x9;
pub const CLOCKS_FC0_SRC_CLK_PERI:                  u32 = 0xA;
pub const CLOCKS_FC0_SRC_CLK_USB:                   u32 = 0xB;
pub const CLOCKS_FC0_SRC_CLK_ADC:                   u32 = 0xC;
pub const CLOCKS_FC0_SRC_CLK_HSTX:                  u32 = 0xD;
pub const CLOCKS_FC0_SRC_LPOSC_CLKSRC:              u32 = 0xE;
pub const CLOCKS_FC0_SRC_OTP_CLK2FC:                u32 = 0xF;
pub const CLOCKS_FC0_SRC_PLL_USB_CLKSRC_PRIMARY_DFT:u32 = 0x10;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
