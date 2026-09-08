#![allow(dead_code)]
// IO_QSPI
pub const IO_QSPI_BASE:                             u32 = 0x4001_8000;
pub const IO_QSPI_GPIO_QSPI_SCLK_STATUS:            u32 = IO_QSPI_BASE + 0x0;
pub const IO_QSPI_GPIO_QSPI_SCLK_CTRL:              u32 = IO_QSPI_BASE + 0x4;
pub const IO_QSPI_GPIO_QSPI_SS_STATUS:              u32 = IO_QSPI_BASE + 0x8;
pub const IO_QSPI_GPIO_QSPI_SS_CTRL:                u32 = IO_QSPI_BASE + 0xC;
pub const IO_QSPI_GPIO_QSPI_SD0_STATUS:             u32 = IO_QSPI_BASE + 0x10;
pub const IO_QSPI_GPIO_QSPI_SD0_CTRL:               u32 = IO_QSPI_BASE + 0x14;
pub const IO_QSPI_GPIO_QSPI_SD1_STATUS:             u32 = IO_QSPI_BASE + 0x18;
pub const IO_QSPI_GPIO_QSPI_SD1_CTRL:               u32 = IO_QSPI_BASE + 0x1C;
pub const IO_QSPI_GPIO_QSPI_SD2_STATUS:             u32 = IO_QSPI_BASE + 0x20;
pub const IO_QSPI_GPIO_QSPI_SD2_CTRL:               u32 = IO_QSPI_BASE + 0x24;
pub const IO_QSPI_GPIO_QSPI_SD3_STATUS:             u32 = IO_QSPI_BASE + 0x28;
pub const IO_QSPI_GPIO_QSPI_SD3_CTRL:               u32 = IO_QSPI_BASE + 0x2C;
pub const IO_QSPI_INTR:                             u32 = IO_QSPI_BASE + 0x30;
pub const IO_QSPI_PROC0_INTE:                       u32 = IO_QSPI_BASE + 0x34;
pub const IO_QSPI_PROC0_INTF:                       u32 = IO_QSPI_BASE + 0x38;
pub const IO_QSPI_PROC0_INTS:                       u32 = IO_QSPI_BASE + 0x3C;
pub const IO_QSPI_PROC1_INTE:                       u32 = IO_QSPI_BASE + 0x40;
pub const IO_QSPI_PROC1_INTF:                       u32 = IO_QSPI_BASE + 0x44;
pub const IO_QSPI_PROC1_INTS:                       u32 = IO_QSPI_BASE + 0x48;
pub const IO_QSPI_DORMANT_WAKE_INTE:                u32 = IO_QSPI_BASE + 0x4C;
pub const IO_QSPI_DORMANT_WAKE_INTF:                u32 = IO_QSPI_BASE + 0x50;
pub const IO_QSPI_DORMANT_WAKE_INTS:                u32 = IO_QSPI_BASE + 0x54;
