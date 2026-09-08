#![allow(dead_code)]
// PADS_QSPI
pub const PADS_QSPI_BASE:                           u32 = 0x4002_0000;
pub const PADS_QSPI_VOLTAGE_SELECT:                 u32 = PADS_QSPI_BASE + 0x0;
pub const PADS_QSPI_GPIO_QSPI_SCLK:                 u32 = PADS_QSPI_BASE + 0x4;
pub const PADS_QSPI_GPIO_QSPI_SD0:                  u32 = PADS_QSPI_BASE + 0x8;
pub const PADS_QSPI_GPIO_QSPI_SD1:                  u32 = PADS_QSPI_BASE + 0xC;
pub const PADS_QSPI_GPIO_QSPI_SD2:                  u32 = PADS_QSPI_BASE + 0x10;
pub const PADS_QSPI_GPIO_QSPI_SD3:                  u32 = PADS_QSPI_BASE + 0x14;
pub const PADS_QSPI_GPIO_QSPI_SS:                   u32 = PADS_QSPI_BASE + 0x18;
