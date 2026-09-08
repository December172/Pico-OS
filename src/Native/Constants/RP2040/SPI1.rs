#![allow(dead_code)]
// SPI1
pub const SPI1_BASE:                                u32 = 0x4004_0000;
pub const SPI1_SSPCR0:                              u32 = SPI1_BASE + 0x0;
pub const SPI1_SSPCR1:                              u32 = SPI1_BASE + 0x4;
pub const SPI1_SSPDR:                               u32 = SPI1_BASE + 0x8;
pub const SPI1_SSPSR:                               u32 = SPI1_BASE + 0xC;
pub const SPI1_SSPCPSR:                             u32 = SPI1_BASE + 0x10;
pub const SPI1_SSPIMSC:                             u32 = SPI1_BASE + 0x14;
pub const SPI1_SSPRIS:                              u32 = SPI1_BASE + 0x18;
pub const SPI1_SSPMIS:                              u32 = SPI1_BASE + 0x1C;
pub const SPI1_SSPICR:                              u32 = SPI1_BASE + 0x20;
pub const SPI1_SSPDMACR:                            u32 = SPI1_BASE + 0x24;
pub const SPI1_SSPPERIPHID0:                        u32 = SPI1_BASE + 0xFE0;
pub const SPI1_SSPPERIPHID1:                        u32 = SPI1_BASE + 0xFE4;
pub const SPI1_SSPPERIPHID2:                        u32 = SPI1_BASE + 0xFE8;
pub const SPI1_SSPPERIPHID3:                        u32 = SPI1_BASE + 0xFEC;
pub const SPI1_SSPPCELLID0:                         u32 = SPI1_BASE + 0xFF0;
pub const SPI1_SSPPCELLID1:                         u32 = SPI1_BASE + 0xFF4;
pub const SPI1_SSPPCELLID2:                         u32 = SPI1_BASE + 0xFF8;
pub const SPI1_SSPPCELLID3:                         u32 = SPI1_BASE + 0xFFC;
