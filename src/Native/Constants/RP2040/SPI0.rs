#![allow(dead_code)]
// SPI0
pub const SPI0_BASE:                                u32 = 0x4003_C000;
pub const SPI0_SSPCR0:                              u32 = SPI0_BASE + 0x0;
pub const SPI0_SSPCR1:                              u32 = SPI0_BASE + 0x4;
pub const SPI0_SSPDR:                               u32 = SPI0_BASE + 0x8;
pub const SPI0_SSPSR:                               u32 = SPI0_BASE + 0xC;
pub const SPI0_SSPCPSR:                             u32 = SPI0_BASE + 0x10;
pub const SPI0_SSPIMSC:                             u32 = SPI0_BASE + 0x14;
pub const SPI0_SSPRIS:                              u32 = SPI0_BASE + 0x18;
pub const SPI0_SSPMIS:                              u32 = SPI0_BASE + 0x1C;
pub const SPI0_SSPICR:                              u32 = SPI0_BASE + 0x20;
pub const SPI0_SSPDMACR:                            u32 = SPI0_BASE + 0x24;
pub const SPI0_SSPPERIPHID0:                        u32 = SPI0_BASE + 0xFE0;
pub const SPI0_SSPPERIPHID1:                        u32 = SPI0_BASE + 0xFE4;
pub const SPI0_SSPPERIPHID2:                        u32 = SPI0_BASE + 0xFE8;
pub const SPI0_SSPPERIPHID3:                        u32 = SPI0_BASE + 0xFEC;
pub const SPI0_SSPPCELLID0:                         u32 = SPI0_BASE + 0xFF0;
pub const SPI0_SSPPCELLID1:                         u32 = SPI0_BASE + 0xFF4;
pub const SPI0_SSPPCELLID2:                         u32 = SPI0_BASE + 0xFF8;
pub const SPI0_SSPPCELLID3:                         u32 = SPI0_BASE + 0xFFC;
