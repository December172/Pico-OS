#![allow(dead_code)]
// ACCESSCTRL

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const ACCESSCTRL_BASE:                          u32 = 0x4006_0000;

pub const ACCESSCTRL_LOCK:                          u32 = ACCESSCTRL_BASE + 0x0;
pub const ACCESSCTRL_FORCE_CORE_NS:                 u32 = ACCESSCTRL_BASE + 0x4;
pub const ACCESSCTRL_CFGRESET:                      u32 = ACCESSCTRL_BASE + 0x8;
// GPIO_NSMASK0..GPIO_NSMASK1
pub fn ACCESSCTRL_GPIO_NSMASK(pin: u32) -> u32 {
    return ACCESSCTRL_BASE + 0xC + pin * 0x4
}

pub const ACCESSCTRL_ROM:                           u32 = ACCESSCTRL_BASE + 0x14;
pub const ACCESSCTRL_XIP_MAIN:                      u32 = ACCESSCTRL_BASE + 0x18;
// SRAM0..SRAM9
pub fn ACCESSCTRL_SRAM(n: u32) -> u32 {
    return ACCESSCTRL_BASE + 0x1C + n * 0x4
}

pub const ACCESSCTRL_DMA:                           u32 = ACCESSCTRL_BASE + 0x44;
pub const ACCESSCTRL_USBCTRL:                       u32 = ACCESSCTRL_BASE + 0x48;
// PIO0..PIO2
pub fn ACCESSCTRL_PIO(n: u32) -> u32 {
    return ACCESSCTRL_BASE + 0x4C + n * 0x4
}

pub const ACCESSCTRL_CORESIGHT_TRACE:               u32 = ACCESSCTRL_BASE + 0x58;
pub const ACCESSCTRL_CORESIGHT_PERIPH:              u32 = ACCESSCTRL_BASE + 0x5C;
pub const ACCESSCTRL_SYSINFO:                       u32 = ACCESSCTRL_BASE + 0x60;
pub const ACCESSCTRL_RESETS:                        u32 = ACCESSCTRL_BASE + 0x64;
// IO_BANK0..IO_BANK1
pub fn ACCESSCTRL_IO_BANK(n: u32) -> u32 {
    return ACCESSCTRL_BASE + 0x68 + n * 0x4
}

pub const ACCESSCTRL_PADS_BANK0:                    u32 = ACCESSCTRL_BASE + 0x70;
pub const ACCESSCTRL_PADS_QSPI:                     u32 = ACCESSCTRL_BASE + 0x74;
pub const ACCESSCTRL_BUSCTRL:                       u32 = ACCESSCTRL_BASE + 0x78;
pub const ACCESSCTRL_ADC0:                          u32 = ACCESSCTRL_BASE + 0x7C;
pub const ACCESSCTRL_HSTX:                          u32 = ACCESSCTRL_BASE + 0x80;
// I2C0..I2C1
pub fn ACCESSCTRL_I2C(n: u32) -> u32 {
    return ACCESSCTRL_BASE + 0x84 + n * 0x4
}

pub const ACCESSCTRL_PWM:                           u32 = ACCESSCTRL_BASE + 0x8C;
// SPI0..SPI1
pub fn ACCESSCTRL_SPI(n: u32) -> u32 {
    return ACCESSCTRL_BASE + 0x90 + n * 0x4
}

// TIMER0..TIMER1
pub fn ACCESSCTRL_TIMER(n: u32) -> u32 {
    return ACCESSCTRL_BASE + 0x98 + n * 0x4
}

// UART0..UART1
pub fn ACCESSCTRL_UART(n: u32) -> u32 {
    return ACCESSCTRL_BASE + 0xA0 + n * 0x4
}

pub const ACCESSCTRL_OTP:                           u32 = ACCESSCTRL_BASE + 0xA8;
pub const ACCESSCTRL_TBMAN:                         u32 = ACCESSCTRL_BASE + 0xAC;
pub const ACCESSCTRL_POWMAN:                        u32 = ACCESSCTRL_BASE + 0xB0;
pub const ACCESSCTRL_TRNG:                          u32 = ACCESSCTRL_BASE + 0xB4;
pub const ACCESSCTRL_SHA256:                        u32 = ACCESSCTRL_BASE + 0xB8;
pub const ACCESSCTRL_SYSCFG:                        u32 = ACCESSCTRL_BASE + 0xBC;
pub const ACCESSCTRL_CLOCKS:                        u32 = ACCESSCTRL_BASE + 0xC0;
pub const ACCESSCTRL_XOSC:                          u32 = ACCESSCTRL_BASE + 0xC4;
pub const ACCESSCTRL_ROSC:                          u32 = ACCESSCTRL_BASE + 0xC8;
pub const ACCESSCTRL_PLL_SYS:                       u32 = ACCESSCTRL_BASE + 0xCC;
pub const ACCESSCTRL_PLL_USB:                       u32 = ACCESSCTRL_BASE + 0xD0;
pub const ACCESSCTRL_TICKS:                         u32 = ACCESSCTRL_BASE + 0xD4;
pub const ACCESSCTRL_WATCHDOG:                      u32 = ACCESSCTRL_BASE + 0xD8;
pub const ACCESSCTRL_RSM:                           u32 = ACCESSCTRL_BASE + 0xDC;
pub const ACCESSCTRL_XIP_CTRL:                      u32 = ACCESSCTRL_BASE + 0xE0;
pub const ACCESSCTRL_XIP_QMI:                       u32 = ACCESSCTRL_BASE + 0xE4;
pub const ACCESSCTRL_XIP_AUX:                       u32 = ACCESSCTRL_BASE + 0xE8;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// LOCK
pub const ACCESSCTRL_LOCK_DEBUG_BIT:                u32 = 3;
pub const ACCESSCTRL_LOCK_DMA_BIT:                  u32 = 2;
pub const ACCESSCTRL_LOCK_CORE1_BIT:                u32 = 1;
pub const ACCESSCTRL_LOCK_CORE0_BIT:                u32 = 0;

// FORCE_CORE_NS
pub const ACCESSCTRL_FORCE_CORE_NS_CORE1_BIT:       u32 = 1;

// CFGRESET
pub const ACCESSCTRL_CFGRESET_BIT:                  u32 = 0;

// GPIO_NSMASK1
pub const ACCESSCTRL_GPIO_NSMASK1_QSPI_SD_LOW:      u32 = 28;
pub const ACCESSCTRL_GPIO_NSMASK1_QSPI_SD_HIGH:     u32 = 31;
pub const ACCESSCTRL_GPIO_NSMASK1_QSPI_CSN_BIT:     u32 = 27;
pub const ACCESSCTRL_GPIO_NSMASK1_QSPI_SCK_BIT:     u32 = 26;
pub const ACCESSCTRL_GPIO_NSMASK1_USB_DM_BIT:       u32 = 25;
pub const ACCESSCTRL_GPIO_NSMASK1_USB_DP_BIT:       u32 = 24;
pub const ACCESSCTRL_GPIO_NSMASK1_GPIO_LOW:         u32 = 0;
pub const ACCESSCTRL_GPIO_NSMASK1_GPIO_HIGH:        u32 = 15;

// ROM
pub const ACCESSCTRL_ROM_DBG_BIT:                   u32 = 7;
pub const ACCESSCTRL_ROM_DMA_BIT:                   u32 = 6;
pub const ACCESSCTRL_ROM_CORE1_BIT:                 u32 = 5;
pub const ACCESSCTRL_ROM_CORE0_BIT:                 u32 = 4;
pub const ACCESSCTRL_ROM_SP_BIT:                    u32 = 3;
pub const ACCESSCTRL_ROM_SU_BIT:                    u32 = 2;
pub const ACCESSCTRL_ROM_NSP_BIT:                   u32 = 1;
pub const ACCESSCTRL_ROM_NSU_BIT:                   u32 = 0;

// XIP_MAIN
pub const ACCESSCTRL_XIP_MAIN_DBG_BIT:              u32 = 7;
pub const ACCESSCTRL_XIP_MAIN_DMA_BIT:              u32 = 6;
pub const ACCESSCTRL_XIP_MAIN_CORE1_BIT:            u32 = 5;
pub const ACCESSCTRL_XIP_MAIN_CORE0_BIT:            u32 = 4;
pub const ACCESSCTRL_XIP_MAIN_SP_BIT:               u32 = 3;
pub const ACCESSCTRL_XIP_MAIN_SU_BIT:               u32 = 2;
pub const ACCESSCTRL_XIP_MAIN_NSP_BIT:              u32 = 1;
pub const ACCESSCTRL_XIP_MAIN_NSU_BIT:              u32 = 0;

// SRAM0..SRAM9
pub const ACCESSCTRL_SRAM_DBG_BIT:                  u32 = 7;
pub const ACCESSCTRL_SRAM_DMA_BIT:                  u32 = 6;
pub const ACCESSCTRL_SRAM_CORE1_BIT:                u32 = 5;
pub const ACCESSCTRL_SRAM_CORE0_BIT:                u32 = 4;
pub const ACCESSCTRL_SRAM_SP_BIT:                   u32 = 3;
pub const ACCESSCTRL_SRAM_SU_BIT:                   u32 = 2;
pub const ACCESSCTRL_SRAM_NSP_BIT:                  u32 = 1;
pub const ACCESSCTRL_SRAM_NSU_BIT:                  u32 = 0;

// DMA
pub const ACCESSCTRL_DMA_DBG_BIT:                   u32 = 7;
pub const ACCESSCTRL_DMA_BIT:                       u32 = 6;
pub const ACCESSCTRL_DMA_CORE1_BIT:                 u32 = 5;
pub const ACCESSCTRL_DMA_CORE0_BIT:                 u32 = 4;
pub const ACCESSCTRL_DMA_SP_BIT:                    u32 = 3;
pub const ACCESSCTRL_DMA_SU_BIT:                    u32 = 2;
pub const ACCESSCTRL_DMA_NSP_BIT:                   u32 = 1;
pub const ACCESSCTRL_DMA_NSU_BIT:                   u32 = 0;

// USBCTRL
pub const ACCESSCTRL_USBCTRL_DBG_BIT:               u32 = 7;
pub const ACCESSCTRL_USBCTRL_DMA_BIT:               u32 = 6;
pub const ACCESSCTRL_USBCTRL_CORE1_BIT:             u32 = 5;
pub const ACCESSCTRL_USBCTRL_CORE0_BIT:             u32 = 4;
pub const ACCESSCTRL_USBCTRL_SP_BIT:                u32 = 3;
pub const ACCESSCTRL_USBCTRL_SU_BIT:                u32 = 2;
pub const ACCESSCTRL_USBCTRL_NSP_BIT:               u32 = 1;
pub const ACCESSCTRL_USBCTRL_NSU_BIT:               u32 = 0;

// PIO0, PIO1, PIO2
pub const ACCESSCTRL_PIO_DBG_BIT:                   u32 = 7;
pub const ACCESSCTRL_PIO_DMA_BIT:                   u32 = 6;
pub const ACCESSCTRL_PIO_CORE1_BIT:                 u32 = 5;
pub const ACCESSCTRL_PIO_CORE0_BIT:                 u32 = 4;
pub const ACCESSCTRL_PIO_SP_BIT:                    u32 = 3;
pub const ACCESSCTRL_PIO_SU_BIT:                    u32 = 2;
pub const ACCESSCTRL_PIO_NSP_BIT:                   u32 = 1;
pub const ACCESSCTRL_PIO_NSU_BIT:                   u32 = 0;

// CORESIGHT_TRACE
pub const ACCESSCTRL_CORESIGHT_TRACE_DBG_BIT:       u32 = 7;
pub const ACCESSCTRL_CORESIGHT_TRACE_DMA_BIT:       u32 = 6;
pub const ACCESSCTRL_CORESIGHT_TRACE_CORE1_BIT:     u32 = 5;
pub const ACCESSCTRL_CORESIGHT_TRACE_CORE0_BIT:     u32 = 4;
pub const ACCESSCTRL_CORESIGHT_TRACE_SP_BIT:        u32 = 3;
pub const ACCESSCTRL_CORESIGHT_TRACE_SU_BIT:        u32 = 2;
pub const ACCESSCTRL_CORESIGHT_TRACE_NSP_BIT:       u32 = 1;
pub const ACCESSCTRL_CORESIGHT_TRACE_NSU_BIT:       u32 = 0;

// CORESIGHT_PERIPH
pub const ACCESSCTRL_CORESIGHT_PERIPH_DBG_BIT:      u32 = 7;
pub const ACCESSCTRL_CORESIGHT_PERIPH_DMA_BIT:      u32 = 6;
pub const ACCESSCTRL_CORESIGHT_PERIPH_CORE1_BIT:    u32 = 5;
pub const ACCESSCTRL_CORESIGHT_PERIPH_CORE0_BIT:    u32 = 4;
pub const ACCESSCTRL_CORESIGHT_PERIPH_SP_BIT:       u32 = 3;
pub const ACCESSCTRL_CORESIGHT_PERIPH_SU_BIT:       u32 = 2;
pub const ACCESSCTRL_CORESIGHT_PERIPH_NSP_BIT:      u32 = 1;
pub const ACCESSCTRL_CORESIGHT_PERIPH_NSU_BIT:      u32 = 0;

// SYSINFO
pub const ACCESSCTRL_SYSINFO_DBG_BIT:               u32 = 7;
pub const ACCESSCTRL_SYSINFO_DMA_BIT:               u32 = 6;
pub const ACCESSCTRL_SYSINFO_CORE1_BIT:             u32 = 5;
pub const ACCESSCTRL_SYSINFO_CORE0_BIT:             u32 = 4;
pub const ACCESSCTRL_SYSINFO_SP_BIT:                u32 = 3;
pub const ACCESSCTRL_SYSINFO_SU_BIT:                u32 = 2;
pub const ACCESSCTRL_SYSINFO_NSP_BIT:               u32 = 1;
pub const ACCESSCTRL_SYSINFO_NSU_BIT:               u32 = 0;

// RESETS
pub const ACCESSCTRL_RESETS_DBG_BIT:                u32 = 7;
pub const ACCESSCTRL_RESETS_DMA_BIT:                u32 = 6;
pub const ACCESSCTRL_RESETS_CORE1_BIT:              u32 = 5;
pub const ACCESSCTRL_RESETS_CORE0_BIT:              u32 = 4;
pub const ACCESSCTRL_RESETS_SP_BIT:                 u32 = 3;
pub const ACCESSCTRL_RESETS_SU_BIT:                 u32 = 2;
pub const ACCESSCTRL_RESETS_NSP_BIT:                u32 = 1;
pub const ACCESSCTRL_RESETS_NSU_BIT:                u32 = 0;

// IO_BANK0, IO_BANK1
pub const ACCESSCTRL_IO_BANK_DBG_BIT:               u32 = 7;
pub const ACCESSCTRL_IO_BANK_DMA_BIT:               u32 = 6;
pub const ACCESSCTRL_IO_BANK_CORE1_BIT:             u32 = 5;
pub const ACCESSCTRL_IO_BANK_CORE0_BIT:             u32 = 4;
pub const ACCESSCTRL_IO_BANK_SP_BIT:                u32 = 3;
pub const ACCESSCTRL_IO_BANK_SU_BIT:                u32 = 2;
pub const ACCESSCTRL_IO_BANK_NSP_BIT:               u32 = 1;
pub const ACCESSCTRL_IO_BANK_NSU_BIT:               u32 = 0;

// PADS_BANK0
pub const ACCESSCTRL_PADS_BANK0_DBG_BIT:            u32 = 7;
pub const ACCESSCTRL_PADS_BANK0_DMA_BIT:            u32 = 6;
pub const ACCESSCTRL_PADS_BANK0_CORE1_BIT:          u32 = 5;
pub const ACCESSCTRL_PADS_BANK0_CORE0_BIT:          u32 = 4;
pub const ACCESSCTRL_PADS_BANK0_SP_BIT:             u32 = 3;
pub const ACCESSCTRL_PADS_BANK0_SU_BIT:             u32 = 2;
pub const ACCESSCTRL_PADS_BANK0_NSP_BIT:            u32 = 1;
pub const ACCESSCTRL_PADS_BANK0_NSU_BIT:            u32 = 0;

// PADS_QSPI
pub const ACCESSCTRL_PADS_QSPI_DBG_BIT:             u32 = 7;
pub const ACCESSCTRL_PADS_QSPI_DMA_BIT:             u32 = 6;
pub const ACCESSCTRL_PADS_QSPI_CORE1_BIT:           u32 = 5;
pub const ACCESSCTRL_PADS_QSPI_CORE0_BIT:           u32 = 4;
pub const ACCESSCTRL_PADS_QSPI_SP_BIT:              u32 = 3;
pub const ACCESSCTRL_PADS_QSPI_SU_BIT:              u32 = 2;
pub const ACCESSCTRL_PADS_QSPI_NSP_BIT:             u32 = 1;
pub const ACCESSCTRL_PADS_QSPI_NSU_BIT:             u32 = 0;

// BUSCTRL
pub const ACCESSCTRL_BUSCTRL_DBG_BIT:               u32 = 7;
pub const ACCESSCTRL_BUSCTRL_DMA_BIT:               u32 = 6;
pub const ACCESSCTRL_BUSCTRL_CORE1_BIT:             u32 = 5;
pub const ACCESSCTRL_BUSCTRL_CORE0_BIT:             u32 = 4;
pub const ACCESSCTRL_BUSCTRL_SP_BIT:                u32 = 3;
pub const ACCESSCTRL_BUSCTRL_SU_BIT:                u32 = 2;
pub const ACCESSCTRL_BUSCTRL_NSP_BIT:               u32 = 1;
pub const ACCESSCTRL_BUSCTRL_NSU_BIT:               u32 = 0;

// ADC0
pub const ACCESSCTRL_ADC0_DBG_BIT:                  u32 = 7;
pub const ACCESSCTRL_ADC0_DMA_BIT:                  u32 = 6;
pub const ACCESSCTRL_ADC0_CORE1_BIT:                u32 = 5;
pub const ACCESSCTRL_ADC0_CORE0_BIT:                u32 = 4;
pub const ACCESSCTRL_ADC0_SP_BIT:                   u32 = 3;
pub const ACCESSCTRL_ADC0_SU_BIT:                   u32 = 2;
pub const ACCESSCTRL_ADC0_NSP_BIT:                  u32 = 1;
pub const ACCESSCTRL_ADC0_NSU_BIT:                  u32 = 0;

// HSTX
pub const ACCESSCTRL_HSTX_DBG_BIT:                  u32 = 7;
pub const ACCESSCTRL_HSTX_DMA_BIT:                  u32 = 6;
pub const ACCESSCTRL_HSTX_CORE1_BIT:                u32 = 5;
pub const ACCESSCTRL_HSTX_CORE0_BIT:                u32 = 4;
pub const ACCESSCTRL_HSTX_SP_BIT:                   u32 = 3;
pub const ACCESSCTRL_HSTX_SU_BIT:                   u32 = 2;
pub const ACCESSCTRL_HSTX_NSP_BIT:                  u32 = 1;
pub const ACCESSCTRL_HSTX_NSU_BIT:                  u32 = 0;

// I2C0, I2C1
pub const ACCESSCTRL_IC_DBG_BIT:                    u32 = 7;
pub const ACCESSCTRL_IC_DMA_BIT:                    u32 = 6;
pub const ACCESSCTRL_IC_CORE1_BIT:                  u32 = 5;
pub const ACCESSCTRL_IC_CORE0_BIT:                  u32 = 4;
pub const ACCESSCTRL_IC_SP_BIT:                     u32 = 3;
pub const ACCESSCTRL_IC_SU_BIT:                     u32 = 2;
pub const ACCESSCTRL_IC_NSP_BIT:                    u32 = 1;
pub const ACCESSCTRL_IC_NSU_BIT:                    u32 = 0;

// PWM
pub const ACCESSCTRL_PWM_DBG_BIT:                   u32 = 7;
pub const ACCESSCTRL_PWM_DMA_BIT:                   u32 = 6;
pub const ACCESSCTRL_PWM_CORE1_BIT:                 u32 = 5;
pub const ACCESSCTRL_PWM_CORE0_BIT:                 u32 = 4;
pub const ACCESSCTRL_PWM_SP_BIT:                    u32 = 3;
pub const ACCESSCTRL_PWM_SU_BIT:                    u32 = 2;
pub const ACCESSCTRL_PWM_NSP_BIT:                   u32 = 1;
pub const ACCESSCTRL_PWM_NSU_BIT:                   u32 = 0;

// SPI0, SPI1
pub const ACCESSCTRL_SPI_DBG_BIT:                   u32 = 7;
pub const ACCESSCTRL_SPI_DMA_BIT:                   u32 = 6;
pub const ACCESSCTRL_SPI_CORE1_BIT:                 u32 = 5;
pub const ACCESSCTRL_SPI_CORE0_BIT:                 u32 = 4;
pub const ACCESSCTRL_SPI_SP_BIT:                    u32 = 3;
pub const ACCESSCTRL_SPI_SU_BIT:                    u32 = 2;
pub const ACCESSCTRL_SPI_NSP_BIT:                   u32 = 1;
pub const ACCESSCTRL_SPI_NSU_BIT:                   u32 = 0;

// TIMER0, TIMER1
pub const ACCESSCTRL_TIMER_DBG_BIT:                 u32 = 7;
pub const ACCESSCTRL_TIMER_DMA_BIT:                 u32 = 6;
pub const ACCESSCTRL_TIMER_CORE1_BIT:               u32 = 5;
pub const ACCESSCTRL_TIMER_CORE0_BIT:               u32 = 4;
pub const ACCESSCTRL_TIMER_SP_BIT:                  u32 = 3;
pub const ACCESSCTRL_TIMER_SU_BIT:                  u32 = 2;
pub const ACCESSCTRL_TIMER_NSP_BIT:                 u32 = 1;
pub const ACCESSCTRL_TIMER_NSU_BIT:                 u32 = 0;

// UART0, UART1
pub const ACCESSCTRL_UART_DBG_BIT:                  u32 = 7;
pub const ACCESSCTRL_UART_DMA_BIT:                  u32 = 6;
pub const ACCESSCTRL_UART_CORE1_BIT:                u32 = 5;
pub const ACCESSCTRL_UART_CORE0_BIT:                u32 = 4;
pub const ACCESSCTRL_UART_SP_BIT:                   u32 = 3;
pub const ACCESSCTRL_UART_SU_BIT:                   u32 = 2;
pub const ACCESSCTRL_UART_NSP_BIT:                  u32 = 1;
pub const ACCESSCTRL_UART_NSU_BIT:                  u32 = 0;

// OTP
pub const ACCESSCTRL_OTP_DBG_BIT:                   u32 = 7;
pub const ACCESSCTRL_OTP_DMA_BIT:                   u32 = 6;
pub const ACCESSCTRL_OTP_CORE1_BIT:                 u32 = 5;
pub const ACCESSCTRL_OTP_CORE0_BIT:                 u32 = 4;
pub const ACCESSCTRL_OTP_SP_BIT:                    u32 = 3;
pub const ACCESSCTRL_OTP_SU_BIT:                    u32 = 2;
pub const ACCESSCTRL_OTP_NSP_BIT:                   u32 = 1;
pub const ACCESSCTRL_OTP_NSU_BIT:                   u32 = 0;

// TBMAN
pub const ACCESSCTRL_TBMAN_DBG_BIT:                 u32 = 7;
pub const ACCESSCTRL_TBMAN_DMA_BIT:                 u32 = 6;
pub const ACCESSCTRL_TBMAN_CORE1_BIT:               u32 = 5;
pub const ACCESSCTRL_TBMAN_CORE0_BIT:               u32 = 4;
pub const ACCESSCTRL_TBMAN_SP_BIT:                  u32 = 3;
pub const ACCESSCTRL_TBMAN_SU_BIT:                  u32 = 2;
pub const ACCESSCTRL_TBMAN_NSP_BIT:                 u32 = 1;
pub const ACCESSCTRL_TBMAN_NSU_BIT:                 u32 = 0;

// POWMAN
pub const ACCESSCTRL_POWMAN_DBG_BIT:                u32 = 7;
pub const ACCESSCTRL_POWMAN_DMA_BIT:                u32 = 6;
pub const ACCESSCTRL_POWMAN_CORE1_BIT:              u32 = 5;
pub const ACCESSCTRL_POWMAN_CORE0_BIT:              u32 = 4;
pub const ACCESSCTRL_POWMAN_SP_BIT:                 u32 = 3;
pub const ACCESSCTRL_POWMAN_SU_BIT:                 u32 = 2;
pub const ACCESSCTRL_POWMAN_NSP_BIT:                u32 = 1;
pub const ACCESSCTRL_POWMAN_NSU_BIT:                u32 = 0;

// TRNG
pub const ACCESSCTRL_TRNG_DBG_BIT:                  u32 = 7;
pub const ACCESSCTRL_TRNG_DMA_BIT:                  u32 = 6;
pub const ACCESSCTRL_TRNG_CORE1_BIT:                u32 = 5;
pub const ACCESSCTRL_TRNG_CORE0_BIT:                u32 = 4;
pub const ACCESSCTRL_TRNG_SP_BIT:                   u32 = 3;
pub const ACCESSCTRL_TRNG_SU_BIT:                   u32 = 2;
pub const ACCESSCTRL_TRNG_NSP_BIT:                  u32 = 1;
pub const ACCESSCTRL_TRNG_NSU_BIT:                  u32 = 0;

// SHA256
pub const ACCESSCTRL_SHA256_DBG_BIT:                u32 = 7;
pub const ACCESSCTRL_SHA256_DMA_BIT:                u32 = 6;
pub const ACCESSCTRL_SHA256_CORE1_BIT:              u32 = 5;
pub const ACCESSCTRL_SHA256_CORE0_BIT:              u32 = 4;
pub const ACCESSCTRL_SHA256_SP_BIT:                 u32 = 3;
pub const ACCESSCTRL_SHA256_SU_BIT:                 u32 = 2;
pub const ACCESSCTRL_SHA256_NSP_BIT:                u32 = 1;
pub const ACCESSCTRL_SHA256_NSU_BIT:                u32 = 0;

// SYSCFG
pub const ACCESSCTRL_SYSCFG_DBG_BIT:                u32 = 7;
pub const ACCESSCTRL_SYSCFG_DMA_BIT:                u32 = 6;
pub const ACCESSCTRL_SYSCFG_CORE1_BIT:              u32 = 5;
pub const ACCESSCTRL_SYSCFG_CORE0_BIT:              u32 = 4;
pub const ACCESSCTRL_SYSCFG_SP_BIT:                 u32 = 3;
pub const ACCESSCTRL_SYSCFG_SU_BIT:                 u32 = 2;
pub const ACCESSCTRL_SYSCFG_NSP_BIT:                u32 = 1;
pub const ACCESSCTRL_SYSCFG_NSU_BIT:                u32 = 0;

// CLOCKS
pub const ACCESSCTRL_CLOCKS_DBG_BIT:                u32 = 7;
pub const ACCESSCTRL_CLOCKS_DMA_BIT:                u32 = 6;
pub const ACCESSCTRL_CLOCKS_CORE1_BIT:              u32 = 5;
pub const ACCESSCTRL_CLOCKS_CORE0_BIT:              u32 = 4;
pub const ACCESSCTRL_CLOCKS_SP_BIT:                 u32 = 3;
pub const ACCESSCTRL_CLOCKS_SU_BIT:                 u32 = 2;
pub const ACCESSCTRL_CLOCKS_NSP_BIT:                u32 = 1;
pub const ACCESSCTRL_CLOCKS_NSU_BIT:                u32 = 0;

// XOSC
pub const ACCESSCTRL_XOSC_DBG_BIT:                  u32 = 7;
pub const ACCESSCTRL_XOSC_DMA_BIT:                  u32 = 6;
pub const ACCESSCTRL_XOSC_CORE1_BIT:                u32 = 5;
pub const ACCESSCTRL_XOSC_CORE0_BIT:                u32 = 4;
pub const ACCESSCTRL_XOSC_SP_BIT:                   u32 = 3;
pub const ACCESSCTRL_XOSC_SU_BIT:                   u32 = 2;
pub const ACCESSCTRL_XOSC_NSP_BIT:                  u32 = 1;
pub const ACCESSCTRL_XOSC_NSU_BIT:                  u32 = 0;

// ROSC
pub const ACCESSCTRL_ROSC_DBG_BIT:                  u32 = 7;
pub const ACCESSCTRL_ROSC_DMA_BIT:                  u32 = 6;
pub const ACCESSCTRL_ROSC_CORE1_BIT:                u32 = 5;
pub const ACCESSCTRL_ROSC_CORE0_BIT:                u32 = 4;
pub const ACCESSCTRL_ROSC_SP_BIT:                   u32 = 3;
pub const ACCESSCTRL_ROSC_SU_BIT:                   u32 = 2;
pub const ACCESSCTRL_ROSC_NSP_BIT:                  u32 = 1;
pub const ACCESSCTRL_ROSC_NSU_BIT:                  u32 = 0;

// PLL_SYS
pub const ACCESSCTRL_PLL_SYS_DBG_BIT:               u32 = 7;
pub const ACCESSCTRL_PLL_SYS_DMA_BIT:               u32 = 6;
pub const ACCESSCTRL_PLL_SYS_CORE1_BIT:             u32 = 5;
pub const ACCESSCTRL_PLL_SYS_CORE0_BIT:             u32 = 4;
pub const ACCESSCTRL_PLL_SYS_SP_BIT:                u32 = 3;
pub const ACCESSCTRL_PLL_SYS_SU_BIT:                u32 = 2;
pub const ACCESSCTRL_PLL_SYS_NSP_BIT:               u32 = 1;
pub const ACCESSCTRL_PLL_SYS_NSU_BIT:               u32 = 0;

// PLL_USB
pub const ACCESSCTRL_PLL_USB_DBG_BIT:               u32 = 7;
pub const ACCESSCTRL_PLL_USB_DMA_BIT:               u32 = 6;
pub const ACCESSCTRL_PLL_USB_CORE1_BIT:             u32 = 5;
pub const ACCESSCTRL_PLL_USB_CORE0_BIT:             u32 = 4;
pub const ACCESSCTRL_PLL_USB_SP_BIT:                u32 = 3;
pub const ACCESSCTRL_PLL_USB_SU_BIT:                u32 = 2;
pub const ACCESSCTRL_PLL_USB_NSP_BIT:               u32 = 1;
pub const ACCESSCTRL_PLL_USB_NSU_BIT:               u32 = 0;

// TICKS
pub const ACCESSCTRL_TICKS_DBG_BIT:                 u32 = 7;
pub const ACCESSCTRL_TICKS_DMA_BIT:                 u32 = 6;
pub const ACCESSCTRL_TICKS_CORE1_BIT:               u32 = 5;
pub const ACCESSCTRL_TICKS_CORE0_BIT:               u32 = 4;
pub const ACCESSCTRL_TICKS_SP_BIT:                  u32 = 3;
pub const ACCESSCTRL_TICKS_SU_BIT:                  u32 = 2;
pub const ACCESSCTRL_TICKS_NSP_BIT:                 u32 = 1;
pub const ACCESSCTRL_TICKS_NSU_BIT:                 u32 = 0;

// WATCHDOG
pub const ACCESSCTRL_WATCHDOG_DBG_BIT:              u32 = 7;
pub const ACCESSCTRL_WATCHDOG_DMA_BIT:              u32 = 6;
pub const ACCESSCTRL_WATCHDOG_CORE1_BIT:            u32 = 5;
pub const ACCESSCTRL_WATCHDOG_CORE0_BIT:            u32 = 4;
pub const ACCESSCTRL_WATCHDOG_SP_BIT:               u32 = 3;
pub const ACCESSCTRL_WATCHDOG_SU_BIT:               u32 = 2;
pub const ACCESSCTRL_WATCHDOG_NSP_BIT:              u32 = 1;
pub const ACCESSCTRL_WATCHDOG_NSU_BIT:              u32 = 0;

// RSM
pub const ACCESSCTRL_RSM_DBG_BIT:                   u32 = 7;
pub const ACCESSCTRL_RSM_DMA_BIT:                   u32 = 6;
pub const ACCESSCTRL_RSM_CORE1_BIT:                 u32 = 5;
pub const ACCESSCTRL_RSM_CORE0_BIT:                 u32 = 4;
pub const ACCESSCTRL_RSM_SP_BIT:                    u32 = 3;
pub const ACCESSCTRL_RSM_SU_BIT:                    u32 = 2;
pub const ACCESSCTRL_RSM_NSP_BIT:                   u32 = 1;
pub const ACCESSCTRL_RSM_NSU_BIT:                   u32 = 0;

// XIP_CTRL
pub const ACCESSCTRL_XIP_CTRL_DBG_BIT:              u32 = 7;
pub const ACCESSCTRL_XIP_CTRL_DMA_BIT:              u32 = 6;
pub const ACCESSCTRL_XIP_CTRL_CORE1_BIT:            u32 = 5;
pub const ACCESSCTRL_XIP_CTRL_CORE0_BIT:            u32 = 4;
pub const ACCESSCTRL_XIP_CTRL_SP_BIT:               u32 = 3;
pub const ACCESSCTRL_XIP_CTRL_SU_BIT:               u32 = 2;
pub const ACCESSCTRL_XIP_CTRL_NSP_BIT:              u32 = 1;
pub const ACCESSCTRL_XIP_CTRL_NSU_BIT:              u32 = 0;

// XIP_QMI
pub const ACCESSCTRL_XIP_QMI_DBG_BIT:               u32 = 7;
pub const ACCESSCTRL_XIP_QMI_DMA_BIT:               u32 = 6;
pub const ACCESSCTRL_XIP_QMI_CORE1_BIT:             u32 = 5;
pub const ACCESSCTRL_XIP_QMI_CORE0_BIT:             u32 = 4;
pub const ACCESSCTRL_XIP_QMI_SP_BIT:                u32 = 3;
pub const ACCESSCTRL_XIP_QMI_SU_BIT:                u32 = 2;
pub const ACCESSCTRL_XIP_QMI_NSP_BIT:               u32 = 1;
pub const ACCESSCTRL_XIP_QMI_NSU_BIT:               u32 = 0;

// XIP_AUX
pub const ACCESSCTRL_XIP_AUX_DBG_BIT:               u32 = 7;
pub const ACCESSCTRL_XIP_AUX_DMA_BIT:               u32 = 6;
pub const ACCESSCTRL_XIP_AUX_CORE1_BIT:             u32 = 5;
pub const ACCESSCTRL_XIP_AUX_CORE0_BIT:             u32 = 4;
pub const ACCESSCTRL_XIP_AUX_SP_BIT:                u32 = 3;
pub const ACCESSCTRL_XIP_AUX_SU_BIT:                u32 = 2;
pub const ACCESSCTRL_XIP_AUX_NSP_BIT:               u32 = 1;
pub const ACCESSCTRL_XIP_AUX_NSU_BIT:               u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
