#![allow(dead_code)]
// RESETS
pub const RESETS_BASE:                              u32 = 0x4000_C000;
pub const RESETS_RESET:                             u32 = RESETS_BASE + 0x0;
pub const RESETS_WDSEL:                             u32 = RESETS_BASE + 0x4;
pub const RESETS_RESET_DONE:                        u32 = RESETS_BASE + 0x8;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// RESET
pub const RESETS_RESET_USBCTRL_BIT:                 u32 = 24;
pub const RESETS_RESET_UART1_BIT:                   u32 = 23;
pub const RESETS_RESET_UART0_BIT:                   u32 = 22;
pub const RESETS_RESET_TIMER_BIT:                   u32 = 21;
pub const RESETS_RESET_TBMAN_BIT:                   u32 = 20;
pub const RESETS_RESET_SYSINFO_BIT:                 u32 = 19;
pub const RESETS_RESET_SYSCFG_BIT:                  u32 = 18;
pub const RESETS_RESET_SPI1_BIT:                    u32 = 17;
pub const RESETS_RESET_SPI0_BIT:                    u32 = 16;
pub const RESETS_RESET_RTC_BIT:                     u32 = 15;
pub const RESETS_RESET_PWM_BIT:                     u32 = 14;
pub const RESETS_RESET_PLL_USB_BIT:                 u32 = 13;
pub const RESETS_RESET_PLL_SYS_BIT:                 u32 = 12;
pub const RESETS_RESET_PIO1_BIT:                    u32 = 11;
pub const RESETS_RESET_PIO0_BIT:                    u32 = 10;
pub const RESETS_RESET_PADS_QSPI_BIT:               u32 = 9;
pub const RESETS_RESET_PADS_BANK0_BIT:              u32 = 8;
pub const RESETS_RESET_JTAG_BIT:                    u32 = 7;
pub const RESETS_RESET_IO_QSPI_BIT:                 u32 = 6;
pub const RESETS_RESET_IO_BANK0_BIT:                u32 = 5;
pub const RESETS_RESET_I2C1_BIT:                    u32 = 4;
pub const RESETS_RESET_I2C0_BIT:                    u32 = 3;
pub const RESETS_RESET_DMA_BIT:                     u32 = 2;
pub const RESETS_RESET_BUSCTRL_BIT:                 u32 = 1;
pub const RESETS_RESET_ADC_BIT:                     u32 = 0;
// WDSEL
pub const RESETS_WDSEL_USBCTRL_BIT:                 u32 = 24;
pub const RESETS_WDSEL_UART1_BIT:                   u32 = 23;
pub const RESETS_WDSEL_UART0_BIT:                   u32 = 22;
pub const RESETS_WDSEL_TIMER_BIT:                   u32 = 21;
pub const RESETS_WDSEL_TBMAN_BIT:                   u32 = 20;
pub const RESETS_WDSEL_SYSINFO_BIT:                 u32 = 19;
pub const RESETS_WDSEL_SYSCFG_BIT:                  u32 = 18;
pub const RESETS_WDSEL_SPI1_BIT:                    u32 = 17;
pub const RESETS_WDSEL_SPI0_BIT:                    u32 = 16;
pub const RESETS_WDSEL_RTC_BIT:                     u32 = 15;
pub const RESETS_WDSEL_PWM_BIT:                     u32 = 14;
pub const RESETS_WDSEL_PLL_USB_BIT:                 u32 = 13;
pub const RESETS_WDSEL_PLL_SYS_BIT:                 u32 = 12;
pub const RESETS_WDSEL_PIO1_BIT:                    u32 = 11;
pub const RESETS_WDSEL_PIO0_BIT:                    u32 = 10;
pub const RESETS_WDSEL_PADS_QSPI_BIT:               u32 = 9;
pub const RESETS_WDSEL_PADS_BANK0_BIT:              u32 = 8;
pub const RESETS_WDSEL_JTAG_BIT:                    u32 = 7;
pub const RESETS_WDSEL_IO_QSPI_BIT:                 u32 = 6;
pub const RESETS_WDSEL_IO_BANK0_BIT:                u32 = 5;
pub const RESETS_WDSEL_I2C1_BIT:                    u32 = 4;
pub const RESETS_WDSEL_I2C0_BIT:                    u32 = 3;
pub const RESETS_WDSEL_DMA_BIT:                     u32 = 2;
pub const RESETS_WDSEL_BUSCTRL_BIT:                 u32 = 1;
pub const RESETS_WDSEL_ADC_BIT:                     u32 = 0;
// RESET_DONE
pub const RESETS_RESET_DONE_USBCTRL_BIT:            u32 = 24;
pub const RESETS_RESET_DONE_UART1_BIT:              u32 = 23;
pub const RESETS_RESET_DONE_UART0_BIT:              u32 = 22;
pub const RESETS_RESET_DONE_TIMER_BIT:              u32 = 21;
pub const RESETS_RESET_DONE_TBMAN_BIT:              u32 = 20;
pub const RESETS_RESET_DONE_SYSINFO_BIT:            u32 = 19;
pub const RESETS_RESET_DONE_SYSCFG_BIT:             u32 = 18;
pub const RESETS_RESET_DONE_SPI1_BIT:               u32 = 17;
pub const RESETS_RESET_DONE_SPI0_BIT:               u32 = 16;
pub const RESETS_RESET_DONE_RTC_BIT:                u32 = 15;
pub const RESETS_RESET_DONE_PWM_BIT:                u32 = 14;
pub const RESETS_RESET_DONE_PLL_USB_BIT:            u32 = 13;
pub const RESETS_RESET_DONE_PLL_SYS_BIT:            u32 = 12;
pub const RESETS_RESET_DONE_PIO1_BIT:               u32 = 11;
pub const RESETS_RESET_DONE_PIO0_BIT:               u32 = 10;
pub const RESETS_RESET_DONE_PADS_QSPI_BIT:          u32 = 9;
pub const RESETS_RESET_DONE_PADS_BANK0_BIT:         u32 = 8;
pub const RESETS_RESET_DONE_JTAG_BIT:               u32 = 7;
pub const RESETS_RESET_DONE_IO_QSPI_BIT:            u32 = 6;
pub const RESETS_RESET_DONE_IO_BANK0_BIT:           u32 = 5;
pub const RESETS_RESET_DONE_I2C1_BIT:               u32 = 4;
pub const RESETS_RESET_DONE_I2C0_BIT:               u32 = 3;
pub const RESETS_RESET_DONE_DMA_BIT:                u32 = 2;
pub const RESETS_RESET_DONE_BUSCTRL_BIT:            u32 = 1;
pub const RESETS_RESET_DONE_ADC_BIT:                u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
