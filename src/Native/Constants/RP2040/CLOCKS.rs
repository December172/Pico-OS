#![allow(dead_code)]
// CLOCKS
pub const CLOCKS_BASE:                              u32 = 0x4000_8000;
pub const CLOCKS_CLK_GPOUT0_CTRL:                   u32 = CLOCKS_BASE + 0x0;
pub const CLOCKS_CLK_GPOUT0_DIV:                    u32 = CLOCKS_BASE + 0x4;
pub const CLOCKS_CLK_GPOUT0_SELECTED:               u32 = CLOCKS_BASE + 0x8;
pub const CLOCKS_CLK_GPOUT1_CTRL:                   u32 = CLOCKS_BASE + 0xC;
pub const CLOCKS_CLK_GPOUT1_DIV:                    u32 = CLOCKS_BASE + 0x10;
pub const CLOCKS_CLK_GPOUT1_SELECTED:               u32 = CLOCKS_BASE + 0x14;
pub const CLOCKS_CLK_GPOUT2_CTRL:                   u32 = CLOCKS_BASE + 0x18;
pub const CLOCKS_CLK_GPOUT2_DIV:                    u32 = CLOCKS_BASE + 0x1C;
pub const CLOCKS_CLK_GPOUT2_SELECTED:               u32 = CLOCKS_BASE + 0x20;
pub const CLOCKS_CLK_GPOUT3_CTRL:                   u32 = CLOCKS_BASE + 0x24;
pub const CLOCKS_CLK_GPOUT3_DIV:                    u32 = CLOCKS_BASE + 0x28;
pub const CLOCKS_CLK_GPOUT3_SELECTED:               u32 = CLOCKS_BASE + 0x2C;
pub const CLOCKS_CLK_REF_CTRL:                      u32 = CLOCKS_BASE + 0x30;
pub const CLOCKS_CLK_REF_DIV:                       u32 = CLOCKS_BASE + 0x34;
pub const CLOCKS_CLK_REF_SELECTED:                  u32 = CLOCKS_BASE + 0x38;
pub const CLOCKS_CLK_SYS_CTRL:                      u32 = CLOCKS_BASE + 0x3C;
pub const CLOCKS_CLK_SYS_DIV:                       u32 = CLOCKS_BASE + 0x40;
pub const CLOCKS_CLK_SYS_SELECTED:                  u32 = CLOCKS_BASE + 0x44;
pub const CLOCKS_CLK_PERI_CTRL:                     u32 = CLOCKS_BASE + 0x48;
pub const CLOCKS_CLK_PERI_SELECTED:                 u32 = CLOCKS_BASE + 0x50;
pub const CLOCKS_CLK_USB_CTRL:                      u32 = CLOCKS_BASE + 0x54;
pub const CLOCKS_CLK_USB_DIV:                       u32 = CLOCKS_BASE + 0x58;
pub const CLOCKS_CLK_USB_SELECTED:                  u32 = CLOCKS_BASE + 0x5C;
pub const CLOCKS_CLK_ADC_CTRL:                      u32 = CLOCKS_BASE + 0x60;
pub const CLOCKS_CLK_ADC_DIV:                       u32 = CLOCKS_BASE + 0x64;
pub const CLOCKS_CLK_ADC_SELECTED:                  u32 = CLOCKS_BASE + 0x68;
pub const CLOCKS_CLK_RTC_CTRL:                      u32 = CLOCKS_BASE + 0x6C;
pub const CLOCKS_CLK_RTC_DIV:                       u32 = CLOCKS_BASE + 0x70;
pub const CLOCKS_CLK_RTC_SELECTED:                  u32 = CLOCKS_BASE + 0x74;
pub const CLOCKS_CLK_SYS_RESUS_CTRL:                u32 = CLOCKS_BASE + 0x78;
pub const CLOCKS_CLK_SYS_RESUS_STATUS:              u32 = CLOCKS_BASE + 0x7C;
pub const CLOCKS_FC0_REF_KHZ:                       u32 = CLOCKS_BASE + 0x80;
pub const CLOCKS_FC0_MIN_KHZ:                       u32 = CLOCKS_BASE + 0x84;
pub const CLOCKS_FC0_MAX_KHZ:                       u32 = CLOCKS_BASE + 0x88;
pub const CLOCKS_FC0_DELAY:                         u32 = CLOCKS_BASE + 0x8C;
pub const CLOCKS_FC0_INTERVAL:                      u32 = CLOCKS_BASE + 0x90;
pub const CLOCKS_FC0_SRC:                           u32 = CLOCKS_BASE + 0x94;
pub const CLOCKS_FC0_STATUS:                        u32 = CLOCKS_BASE + 0x98;
pub const CLOCKS_FC0_RESULT:                        u32 = CLOCKS_BASE + 0x9C;
pub const CLOCKS_WAKE_EN0:                          u32 = CLOCKS_BASE + 0xA0;
pub const CLOCKS_WAKE_EN1:                          u32 = CLOCKS_BASE + 0xA4;
pub const CLOCKS_SLEEP_EN0:                         u32 = CLOCKS_BASE + 0xA8;
pub const CLOCKS_SLEEP_EN1:                         u32 = CLOCKS_BASE + 0xAC;
pub const CLOCKS_ENABLED0:                          u32 = CLOCKS_BASE + 0xB0;
pub const CLOCKS_ENABLED1:                          u32 = CLOCKS_BASE + 0xB4;
pub const CLOCKS_INTR:                              u32 = CLOCKS_BASE + 0xB8;
pub const CLOCKS_INTE:                              u32 = CLOCKS_BASE + 0xBC;
pub const CLOCKS_INTF:                              u32 = CLOCKS_BASE + 0xC0;
pub const CLOCKS_INTS:                              u32 = CLOCKS_BASE + 0xC4;
/// CLOCKS_CLK_<user>_CTRL: [7:5] sets up the clock source for the user
/// usage: CLOCKS_CLK_<user>_CTRL.write_volatile(CLOCKS_<source>_SOURCE << CLOCKS_SOURCE_SHIFT)
pub const CLOCKS_SOURCE_SHIFT:                      u32 = 5;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CLK_GPOUT0_CTRL
pub const CLOCKS_CLK_GPOUT0_CTRL_NUDGE_BIT:         u32 = 20;
pub const CLOCKS_CLK_GPOUT0_CTRL_PHASE_LOW:         u32 = 16;
pub const CLOCKS_CLK_GPOUT0_CTRL_PHASE_HIGH:        u32 = 17;
pub const CLOCKS_CLK_GPOUT0_CTRL_DC50_BIT:          u32 = 12;
pub const CLOCKS_CLK_GPOUT0_CTRL_ENABLE_BIT:        u32 = 11;
pub const CLOCKS_CLK_GPOUT0_CTRL_KILL_BIT:          u32 = 10;
pub const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_LOW:        u32 = 5;
pub const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_HIGH:       u32 = 8;
// CLK_GPOUT0_DIV
pub const CLOCKS_CLK_GPOUT0_DIV_INT_LOW:            u32 = 8;
pub const CLOCKS_CLK_GPOUT0_DIV_INT_HIGH:           u32 = 31;
pub const CLOCKS_CLK_GPOUT0_DIV_FRAC_LOW:           u32 = 0;
pub const CLOCKS_CLK_GPOUT0_DIV_FRAC_HIGH:          u32 = 7;
// CLK_GPOUT0_SELECTED
pub const CLOCKS_CLK_GPOUT0_SELECTED_LOW:           u32 = 0;
pub const CLOCKS_CLK_GPOUT0_SELECTED_HIGH:          u32 = 31;
// CLK_GPOUT1_CTRL
pub const CLOCKS_CLK_GPOUT1_CTRL_NUDGE_BIT:         u32 = 20;
pub const CLOCKS_CLK_GPOUT1_CTRL_PHASE_LOW:         u32 = 16;
pub const CLOCKS_CLK_GPOUT1_CTRL_PHASE_HIGH:        u32 = 17;
pub const CLOCKS_CLK_GPOUT1_CTRL_DC50_BIT:          u32 = 12;
pub const CLOCKS_CLK_GPOUT1_CTRL_ENABLE_BIT:        u32 = 11;
pub const CLOCKS_CLK_GPOUT1_CTRL_KILL_BIT:          u32 = 10;
pub const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_LOW:        u32 = 5;
pub const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_HIGH:       u32 = 8;
// CLK_GPOUT1_DIV
pub const CLOCKS_CLK_GPOUT1_DIV_INT_LOW:            u32 = 8;
pub const CLOCKS_CLK_GPOUT1_DIV_INT_HIGH:           u32 = 31;
pub const CLOCKS_CLK_GPOUT1_DIV_FRAC_LOW:           u32 = 0;
pub const CLOCKS_CLK_GPOUT1_DIV_FRAC_HIGH:          u32 = 7;
// CLK_GPOUT1_SELECTED
pub const CLOCKS_CLK_GPOUT1_SELECTED_LOW:           u32 = 0;
pub const CLOCKS_CLK_GPOUT1_SELECTED_HIGH:          u32 = 31;
// CLK_GPOUT2_CTRL
pub const CLOCKS_CLK_GPOUT2_CTRL_NUDGE_BIT:         u32 = 20;
pub const CLOCKS_CLK_GPOUT2_CTRL_PHASE_LOW:         u32 = 16;
pub const CLOCKS_CLK_GPOUT2_CTRL_PHASE_HIGH:        u32 = 17;
pub const CLOCKS_CLK_GPOUT2_CTRL_DC50_BIT:          u32 = 12;
pub const CLOCKS_CLK_GPOUT2_CTRL_ENABLE_BIT:        u32 = 11;
pub const CLOCKS_CLK_GPOUT2_CTRL_KILL_BIT:          u32 = 10;
pub const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_LOW:        u32 = 5;
pub const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_HIGH:       u32 = 8;
// CLK_GPOUT2_DIV
pub const CLOCKS_CLK_GPOUT2_DIV_INT_LOW:            u32 = 8;
pub const CLOCKS_CLK_GPOUT2_DIV_INT_HIGH:           u32 = 31;
pub const CLOCKS_CLK_GPOUT2_DIV_FRAC_LOW:           u32 = 0;
pub const CLOCKS_CLK_GPOUT2_DIV_FRAC_HIGH:          u32 = 7;
// CLK_GPOUT2_SELECTED
pub const CLOCKS_CLK_GPOUT2_SELECTED_LOW:           u32 = 0;
pub const CLOCKS_CLK_GPOUT2_SELECTED_HIGH:          u32 = 31;
// CLK_GPOUT3_CTRL
pub const CLOCKS_CLK_GPOUT3_CTRL_NUDGE_BIT:         u32 = 20;
pub const CLOCKS_CLK_GPOUT3_CTRL_PHASE_LOW:         u32 = 16;
pub const CLOCKS_CLK_GPOUT3_CTRL_PHASE_HIGH:        u32 = 17;
pub const CLOCKS_CLK_GPOUT3_CTRL_DC50_BIT:          u32 = 12;
pub const CLOCKS_CLK_GPOUT3_CTRL_ENABLE_BIT:        u32 = 11;
pub const CLOCKS_CLK_GPOUT3_CTRL_KILL_BIT:          u32 = 10;
pub const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_LOW:        u32 = 5;
pub const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_HIGH:       u32 = 8;
// CLK_GPOUT3_DIV
pub const CLOCKS_CLK_GPOUT3_DIV_INT_LOW:            u32 = 8;
pub const CLOCKS_CLK_GPOUT3_DIV_INT_HIGH:           u32 = 31;
pub const CLOCKS_CLK_GPOUT3_DIV_FRAC_LOW:           u32 = 0;
pub const CLOCKS_CLK_GPOUT3_DIV_FRAC_HIGH:          u32 = 7;
// CLK_GPOUT3_SELECTED
pub const CLOCKS_CLK_GPOUT3_SELECTED_LOW:           u32 = 0;
pub const CLOCKS_CLK_GPOUT3_SELECTED_HIGH:          u32 = 31;
// CLK_REF_CTRL
pub const CLOCKS_CLK_REF_CTRL_AUXSRC_LOW:           u32 = 5;
pub const CLOCKS_CLK_REF_CTRL_AUXSRC_HIGH:          u32 = 6;
pub const CLOCKS_CLK_REF_CTRL_SRC_LOW:              u32 = 0;
pub const CLOCKS_CLK_REF_CTRL_SRC_HIGH:             u32 = 1;
// CLK_REF_DIV
pub const CLOCKS_CLK_REF_DIV_INT_LOW:               u32 = 8;
pub const CLOCKS_CLK_REF_DIV_INT_HIGH:              u32 = 9;
// CLK_REF_SELECTED
pub const CLOCKS_CLK_REF_SELECTED_LOW:              u32 = 0;
pub const CLOCKS_CLK_REF_SELECTED_HIGH:             u32 = 31;
// CLK_SYS_CTRL
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_LOW:           u32 = 5;
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_HIGH:          u32 = 7;
pub const CLOCKS_CLK_SYS_CTRL_SRC_BIT:              u32 = 0;
// CLK_SYS_DIV
pub const CLOCKS_CLK_SYS_DIV_INT_LOW:               u32 = 8;
pub const CLOCKS_CLK_SYS_DIV_INT_HIGH:              u32 = 31;
pub const CLOCKS_CLK_SYS_DIV_FRAC_LOW:              u32 = 0;
pub const CLOCKS_CLK_SYS_DIV_FRAC_HIGH:             u32 = 7;
// CLK_SYS_SELECTED
pub const CLOCKS_CLK_SYS_SELECTED_LOW:              u32 = 0;
pub const CLOCKS_CLK_SYS_SELECTED_HIGH:             u32 = 31;
// CLK_PERI_CTRL
pub const CLOCKS_CLK_PERI_CTRL_ENABLE_BIT:          u32 = 11;
pub const CLOCKS_CLK_PERI_CTRL_KILL_BIT:            u32 = 10;
pub const CLOCKS_CLK_PERI_CTRL_AUXSRC_LOW:          u32 = 5;
pub const CLOCKS_CLK_PERI_CTRL_AUXSRC_HIGH:         u32 = 7;
// CLK_PERI_SELECTED
pub const CLOCKS_CLK_PERI_SELECTED_LOW:             u32 = 0;
pub const CLOCKS_CLK_PERI_SELECTED_HIGH:            u32 = 31;
// CLK_USB_CTRL
pub const CLOCKS_CLK_USB_CTRL_NUDGE_BIT:            u32 = 20;
pub const CLOCKS_CLK_USB_CTRL_PHASE_LOW:            u32 = 16;
pub const CLOCKS_CLK_USB_CTRL_PHASE_HIGH:           u32 = 17;
pub const CLOCKS_CLK_USB_CTRL_ENABLE_BIT:           u32 = 11;
pub const CLOCKS_CLK_USB_CTRL_KILL_BIT:             u32 = 10;
pub const CLOCKS_CLK_USB_CTRL_AUXSRC_LOW:           u32 = 5;
pub const CLOCKS_CLK_USB_CTRL_AUXSRC_HIGH:          u32 = 7;
// CLK_USB_DIV
pub const CLOCKS_CLK_USB_DIV_INT_LOW:               u32 = 8;
pub const CLOCKS_CLK_USB_DIV_INT_HIGH:              u32 = 9;
// CLK_USB_SELECTED
pub const CLOCKS_CLK_USB_SELECTED_LOW:              u32 = 0;
pub const CLOCKS_CLK_USB_SELECTED_HIGH:             u32 = 31;
// CLK_ADC_CTRL
pub const CLOCKS_CLK_ADC_CTRL_NUDGE_BIT:            u32 = 20;
pub const CLOCKS_CLK_ADC_CTRL_PHASE_LOW:            u32 = 16;
pub const CLOCKS_CLK_ADC_CTRL_PHASE_HIGH:           u32 = 17;
pub const CLOCKS_CLK_ADC_CTRL_ENABLE_BIT:           u32 = 11;
pub const CLOCKS_CLK_ADC_CTRL_KILL_BIT:             u32 = 10;
pub const CLOCKS_CLK_ADC_CTRL_AUXSRC_LOW:           u32 = 5;
pub const CLOCKS_CLK_ADC_CTRL_AUXSRC_HIGH:          u32 = 7;
// CLK_ADC_DIV
pub const CLOCKS_CLK_ADC_DIV_INT_LOW:               u32 = 8;
pub const CLOCKS_CLK_ADC_DIV_INT_HIGH:              u32 = 9;
// CLK_ADC_SELECTED
pub const CLOCKS_CLK_ADC_SELECTED_LOW:              u32 = 0;
pub const CLOCKS_CLK_ADC_SELECTED_HIGH:             u32 = 31;
// CLK_RTC_CTRL
pub const CLOCKS_CLK_RTC_CTRL_NUDGE_BIT:            u32 = 20;
pub const CLOCKS_CLK_RTC_CTRL_PHASE_LOW:            u32 = 16;
pub const CLOCKS_CLK_RTC_CTRL_PHASE_HIGH:           u32 = 17;
pub const CLOCKS_CLK_RTC_CTRL_ENABLE_BIT:           u32 = 11;
pub const CLOCKS_CLK_RTC_CTRL_KILL_BIT:             u32 = 10;
pub const CLOCKS_CLK_RTC_CTRL_AUXSRC_LOW:           u32 = 5;
pub const CLOCKS_CLK_RTC_CTRL_AUXSRC_HIGH:          u32 = 7;
// CLK_RTC_DIV
pub const CLOCKS_CLK_RTC_DIV_INT_LOW:               u32 = 8;
pub const CLOCKS_CLK_RTC_DIV_INT_HIGH:              u32 = 31;
pub const CLOCKS_CLK_RTC_DIV_FRAC_LOW:              u32 = 0;
pub const CLOCKS_CLK_RTC_DIV_FRAC_HIGH:             u32 = 7;
// CLK_RTC_SELECTED
pub const CLOCKS_CLK_RTC_SELECTED_LOW:              u32 = 0;
pub const CLOCKS_CLK_RTC_SELECTED_HIGH:             u32 = 31;
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
pub const CLOCKS_WAKE_EN0_CLK_SYS_SRAM3_BIT:        u32 = 31;
pub const CLOCKS_WAKE_EN0_CLK_SYS_SRAM2_BIT:        u32 = 30;
pub const CLOCKS_WAKE_EN0_CLK_SYS_SRAM1_BIT:        u32 = 29;
pub const CLOCKS_WAKE_EN0_CLK_SYS_SRAM0_BIT:        u32 = 28;
pub const CLOCKS_WAKE_EN0_CLK_SYS_SPI1_BIT:         u32 = 27;
pub const CLOCKS_WAKE_EN0_CLK_PERI_SPI1_BIT:        u32 = 26;
pub const CLOCKS_WAKE_EN0_CLK_SYS_SPI0_BIT:         u32 = 25;
pub const CLOCKS_WAKE_EN0_CLK_PERI_SPI0_BIT:        u32 = 24;
pub const CLOCKS_WAKE_EN0_CLK_SYS_SIO_BIT:          u32 = 23;
pub const CLOCKS_WAKE_EN0_CLK_SYS_RTC_BIT:          u32 = 22;
pub const CLOCKS_WAKE_EN0_CLK_RTC_RTC_BIT:          u32 = 21;
pub const CLOCKS_WAKE_EN0_CLK_SYS_ROSC_BIT:         u32 = 20;
pub const CLOCKS_WAKE_EN0_CLK_SYS_ROM_BIT:          u32 = 19;
pub const CLOCKS_WAKE_EN0_CLK_SYS_RESETS_BIT:       u32 = 18;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PWM_BIT:          u32 = 17;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PSM_BIT:          u32 = 16;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PLL_USB_BIT:      u32 = 15;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PLL_SYS_BIT:      u32 = 14;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PIO1_BIT:         u32 = 13;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PIO0_BIT:         u32 = 12;
pub const CLOCKS_WAKE_EN0_CLK_SYS_PADS_BIT:         u32 = 11;
pub const CLOCKS_WAKE_EN0_CLK_SYS_VREG_AND_CHIP_RESET_BIT:u32 = 10;
pub const CLOCKS_WAKE_EN0_CLK_SYS_JTAG_BIT:         u32 = 9;
pub const CLOCKS_WAKE_EN0_CLK_SYS_IO_BIT:           u32 = 8;
pub const CLOCKS_WAKE_EN0_CLK_SYS_I2C1_BIT:         u32 = 7;
pub const CLOCKS_WAKE_EN0_CLK_SYS_I2C0_BIT:         u32 = 6;
pub const CLOCKS_WAKE_EN0_CLK_SYS_DMA_BIT:          u32 = 5;
pub const CLOCKS_WAKE_EN0_CLK_SYS_BUSFABRIC_BIT:    u32 = 4;
pub const CLOCKS_WAKE_EN0_CLK_SYS_BUSCTRL_BIT:      u32 = 3;
pub const CLOCKS_WAKE_EN0_CLK_SYS_ADC_BIT:          u32 = 2;
pub const CLOCKS_WAKE_EN0_CLK_ADC_ADC_BIT:          u32 = 1;
pub const CLOCKS_WAKE_EN0_CLK_SYS_CLOCKS_BIT:       u32 = 0;
// WAKE_EN1
pub const CLOCKS_WAKE_EN1_CLK_SYS_XOSC_BIT:         u32 = 14;
pub const CLOCKS_WAKE_EN1_CLK_SYS_XIP_BIT:          u32 = 13;
pub const CLOCKS_WAKE_EN1_CLK_SYS_WATCHDOG_BIT:     u32 = 12;
pub const CLOCKS_WAKE_EN1_CLK_USB_USBCTRL_BIT:      u32 = 11;
pub const CLOCKS_WAKE_EN1_CLK_SYS_USBCTRL_BIT:      u32 = 10;
pub const CLOCKS_WAKE_EN1_CLK_SYS_UART1_BIT:        u32 = 9;
pub const CLOCKS_WAKE_EN1_CLK_PERI_UART1_BIT:       u32 = 8;
pub const CLOCKS_WAKE_EN1_CLK_SYS_UART0_BIT:        u32 = 7;
pub const CLOCKS_WAKE_EN1_CLK_PERI_UART0_BIT:       u32 = 6;
pub const CLOCKS_WAKE_EN1_CLK_SYS_TIMER_BIT:        u32 = 5;
pub const CLOCKS_WAKE_EN1_CLK_SYS_TBMAN_BIT:        u32 = 4;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SYSINFO_BIT:      u32 = 3;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SYSCFG_BIT:       u32 = 2;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM5_BIT:        u32 = 1;
pub const CLOCKS_WAKE_EN1_CLK_SYS_SRAM4_BIT:        u32 = 0;
// SLEEP_EN0
pub const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM3_BIT:       u32 = 31;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM2_BIT:       u32 = 30;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM1_BIT:       u32 = 29;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM0_BIT:       u32 = 28;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_SPI1_BIT:        u32 = 27;
pub const CLOCKS_SLEEP_EN0_CLK_PERI_SPI1_BIT:       u32 = 26;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_SPI0_BIT:        u32 = 25;
pub const CLOCKS_SLEEP_EN0_CLK_PERI_SPI0_BIT:       u32 = 24;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_SIO_BIT:         u32 = 23;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_RTC_BIT:         u32 = 22;
pub const CLOCKS_SLEEP_EN0_CLK_RTC_RTC_BIT:         u32 = 21;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_ROSC_BIT:        u32 = 20;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_ROM_BIT:         u32 = 19;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_RESETS_BIT:      u32 = 18;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PWM_BIT:         u32 = 17;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PSM_BIT:         u32 = 16;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_USB_BIT:     u32 = 15;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_SYS_BIT:     u32 = 14;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PIO1_BIT:        u32 = 13;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PIO0_BIT:        u32 = 12;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_PADS_BIT:        u32 = 11;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_VREG_AND_CHIP_RESET_BIT:u32 = 10;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_JTAG_BIT:        u32 = 9;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_IO_BIT:          u32 = 8;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_I2C1_BIT:        u32 = 7;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_I2C0_BIT:        u32 = 6;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_DMA_BIT:         u32 = 5;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_BUSFABRIC_BIT:   u32 = 4;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_BUSCTRL_BIT:     u32 = 3;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_ADC_BIT:         u32 = 2;
pub const CLOCKS_SLEEP_EN0_CLK_ADC_ADC_BIT:         u32 = 1;
pub const CLOCKS_SLEEP_EN0_CLK_SYS_CLOCKS_BIT:      u32 = 0;
// SLEEP_EN1
pub const CLOCKS_SLEEP_EN1_CLK_SYS_XOSC_BIT:        u32 = 14;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_XIP_BIT:         u32 = 13;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_WATCHDOG_BIT:    u32 = 12;
pub const CLOCKS_SLEEP_EN1_CLK_USB_USBCTRL_BIT:     u32 = 11;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_USBCTRL_BIT:     u32 = 10;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_UART1_BIT:       u32 = 9;
pub const CLOCKS_SLEEP_EN1_CLK_PERI_UART1_BIT:      u32 = 8;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_UART0_BIT:       u32 = 7;
pub const CLOCKS_SLEEP_EN1_CLK_PERI_UART0_BIT:      u32 = 6;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_TIMER_BIT:       u32 = 5;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_TBMAN_BIT:       u32 = 4;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SYSINFO_BIT:     u32 = 3;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SYSCFG_BIT:      u32 = 2;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM5_BIT:       u32 = 1;
pub const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM4_BIT:       u32 = 0;
// ENABLED0
pub const CLOCKS_ENABLED0_CLK_SYS_SRAM3_BIT:        u32 = 31;
pub const CLOCKS_ENABLED0_CLK_SYS_SRAM2_BIT:        u32 = 30;
pub const CLOCKS_ENABLED0_CLK_SYS_SRAM1_BIT:        u32 = 29;
pub const CLOCKS_ENABLED0_CLK_SYS_SRAM0_BIT:        u32 = 28;
pub const CLOCKS_ENABLED0_CLK_SYS_SPI1_BIT:         u32 = 27;
pub const CLOCKS_ENABLED0_CLK_PERI_SPI1_BIT:        u32 = 26;
pub const CLOCKS_ENABLED0_CLK_SYS_SPI0_BIT:         u32 = 25;
pub const CLOCKS_ENABLED0_CLK_PERI_SPI0_BIT:        u32 = 24;
pub const CLOCKS_ENABLED0_CLK_SYS_SIO_BIT:          u32 = 23;
pub const CLOCKS_ENABLED0_CLK_SYS_RTC_BIT:          u32 = 22;
pub const CLOCKS_ENABLED0_CLK_RTC_RTC_BIT:          u32 = 21;
pub const CLOCKS_ENABLED0_CLK_SYS_ROSC_BIT:         u32 = 20;
pub const CLOCKS_ENABLED0_CLK_SYS_ROM_BIT:          u32 = 19;
pub const CLOCKS_ENABLED0_CLK_SYS_RESETS_BIT:       u32 = 18;
pub const CLOCKS_ENABLED0_CLK_SYS_PWM_BIT:          u32 = 17;
pub const CLOCKS_ENABLED0_CLK_SYS_PSM_BIT:          u32 = 16;
pub const CLOCKS_ENABLED0_CLK_SYS_PLL_USB_BIT:      u32 = 15;
pub const CLOCKS_ENABLED0_CLK_SYS_PLL_SYS_BIT:      u32 = 14;
pub const CLOCKS_ENABLED0_CLK_SYS_PIO1_BIT:         u32 = 13;
pub const CLOCKS_ENABLED0_CLK_SYS_PIO0_BIT:         u32 = 12;
pub const CLOCKS_ENABLED0_CLK_SYS_PADS_BIT:         u32 = 11;
pub const CLOCKS_ENABLED0_CLK_SYS_VREG_AND_CHIP_RESET_BIT:u32 = 10;
pub const CLOCKS_ENABLED0_CLK_SYS_JTAG_BIT:         u32 = 9;
pub const CLOCKS_ENABLED0_CLK_SYS_IO_BIT:           u32 = 8;
pub const CLOCKS_ENABLED0_CLK_SYS_I2C1_BIT:         u32 = 7;
pub const CLOCKS_ENABLED0_CLK_SYS_I2C0_BIT:         u32 = 6;
pub const CLOCKS_ENABLED0_CLK_SYS_DMA_BIT:          u32 = 5;
pub const CLOCKS_ENABLED0_CLK_SYS_BUSFABRIC_BIT:    u32 = 4;
pub const CLOCKS_ENABLED0_CLK_SYS_BUSCTRL_BIT:      u32 = 3;
pub const CLOCKS_ENABLED0_CLK_SYS_ADC_BIT:          u32 = 2;
pub const CLOCKS_ENABLED0_CLK_ADC_ADC_BIT:          u32 = 1;
pub const CLOCKS_ENABLED0_CLK_SYS_CLOCKS_BIT:       u32 = 0;
// ENABLED1
pub const CLOCKS_ENABLED1_CLK_SYS_XOSC_BIT:         u32 = 14;
pub const CLOCKS_ENABLED1_CLK_SYS_XIP_BIT:          u32 = 13;
pub const CLOCKS_ENABLED1_CLK_SYS_WATCHDOG_BIT:     u32 = 12;
pub const CLOCKS_ENABLED1_CLK_USB_USBCTRL_BIT:      u32 = 11;
pub const CLOCKS_ENABLED1_CLK_SYS_USBCTRL_BIT:      u32 = 10;
pub const CLOCKS_ENABLED1_CLK_SYS_UART1_BIT:        u32 = 9;
pub const CLOCKS_ENABLED1_CLK_PERI_UART1_BIT:       u32 = 8;
pub const CLOCKS_ENABLED1_CLK_SYS_UART0_BIT:        u32 = 7;
pub const CLOCKS_ENABLED1_CLK_PERI_UART0_BIT:       u32 = 6;
pub const CLOCKS_ENABLED1_CLK_SYS_TIMER_BIT:        u32 = 5;
pub const CLOCKS_ENABLED1_CLK_SYS_TBMAN_BIT:        u32 = 4;
pub const CLOCKS_ENABLED1_CLK_SYS_SYSINFO_BIT:      u32 = 3;
pub const CLOCKS_ENABLED1_CLK_SYS_SYSCFG_BIT:       u32 = 2;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM5_BIT:        u32 = 1;
pub const CLOCKS_ENABLED1_CLK_SYS_SRAM4_BIT:        u32 = 0;
// INTR
pub const CLOCKS_INTR_CLK_SYS_RESUS_BIT:            u32 = 0;
// INTE
pub const CLOCKS_INTE_CLK_SYS_RESUS_BIT:            u32 = 0;
// INTF
pub const CLOCKS_INTF_CLK_SYS_RESUS_BIT:            u32 = 0;
// INTS
pub const CLOCKS_INTS_CLK_SYS_RESUS_BIT:            u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED AUXSRC ENUMERATED VALUES (tools/gen_auxsrc_enums.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CLK_GPOUT0_CTRL, CLK_GPOUT1_CTRL, CLK_GPOUT2_CTRL, CLK_GPOUT3_CTRL.AUXSRC [8:5]
pub const CLOCKS_CLK_GPOUT_PLL_SYS_AUXSOURCE:       u32 = 0x0;
pub const CLOCKS_CLK_GPOUT_GPIN0_AUXSOURCE:         u32 = 0x1;
pub const CLOCKS_CLK_GPOUT_GPIN1_AUXSOURCE:         u32 = 0x2;
pub const CLOCKS_CLK_GPOUT_PLL_USB_AUXSOURCE:       u32 = 0x3;
pub const CLOCKS_CLK_GPOUT_ROSC_AUXSOURCE:          u32 = 0x4;
pub const CLOCKS_CLK_GPOUT_XOSC_AUXSOURCE:          u32 = 0x5;
pub const CLOCKS_CLK_GPOUT_CLK_SYS_AUXSOURCE:       u32 = 0x6;
pub const CLOCKS_CLK_GPOUT_CLK_USB_AUXSOURCE:       u32 = 0x7;
pub const CLOCKS_CLK_GPOUT_CLK_ADC_AUXSOURCE:       u32 = 0x8;
pub const CLOCKS_CLK_GPOUT_CLK_RTC_AUXSOURCE:       u32 = 0x9;
pub const CLOCKS_CLK_GPOUT_CLK_REF_AUXSOURCE:       u32 = 0xA;
pub const CLOCKS_CLK_GPOUT_ROSC_PH_AUXSOURCE:       u32 = 0x4;

// CLK_REF_CTRL.AUXSRC [6:5]
pub const CLOCKS_CLK_REF_PLL_USB_AUXSOURCE:         u32 = 0x0;
pub const CLOCKS_CLK_REF_GPIN0_AUXSOURCE:           u32 = 0x1;
pub const CLOCKS_CLK_REF_GPIN1_AUXSOURCE:           u32 = 0x2;

// CLK_REF_CTRL.SRC [1:0]
pub const CLOCKS_CLK_REF_ROSC_PH_SRC:               u32 = 0x0;
pub const CLOCKS_CLK_REF_CLK_REF_AUX_SRC:           u32 = 0x1;
pub const CLOCKS_CLK_REF_XOSC_SRC:                  u32 = 0x2;

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

// CLK_RTC_CTRL.AUXSRC [7:5]
pub const CLOCKS_CLK_RTC_PLL_USB_AUXSOURCE:         u32 = 0x0;
pub const CLOCKS_CLK_RTC_PLL_SYS_AUXSOURCE:         u32 = 0x1;
pub const CLOCKS_CLK_RTC_ROSC_PH_AUXSOURCE:         u32 = 0x2;
pub const CLOCKS_CLK_RTC_XOSC_AUXSOURCE:            u32 = 0x3;
pub const CLOCKS_CLK_RTC_GPIN0_AUXSOURCE:           u32 = 0x4;
pub const CLOCKS_CLK_RTC_GPIN1_AUXSOURCE:           u32 = 0x5;
// ==== END AUTO-GENERATED AUXSRC ENUMERATED VALUES ====
