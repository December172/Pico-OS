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

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// SSPCR0
pub const SPI1_SSPCR0_SCR_LOW:                      u32 = 8;
pub const SPI1_SSPCR0_SCR_HIGH:                     u32 = 15;
pub const SPI1_SSPCR0_SPH_BIT:                      u32 = 7;
pub const SPI1_SSPCR0_SPO_BIT:                      u32 = 6;
pub const SPI1_SSPCR0_FRF_LOW:                      u32 = 4;
pub const SPI1_SSPCR0_FRF_HIGH:                     u32 = 5;
pub const SPI1_SSPCR0_DSS_LOW:                      u32 = 0;
pub const SPI1_SSPCR0_DSS_HIGH:                     u32 = 3;
// SSPCR1
pub const SPI1_SSPCR1_SOD_BIT:                      u32 = 3;
pub const SPI1_SSPCR1_MS_BIT:                       u32 = 2;
pub const SPI1_SSPCR1_SSE_BIT:                      u32 = 1;
pub const SPI1_SSPCR1_LBM_BIT:                      u32 = 0;
// SSPDR
pub const SPI1_SSPDR_DATA_LOW:                      u32 = 0;
pub const SPI1_SSPDR_DATA_HIGH:                     u32 = 15;
// SSPSR
pub const SPI1_SSPSR_BSY_BIT:                       u32 = 4;
pub const SPI1_SSPSR_RFF_BIT:                       u32 = 3;
pub const SPI1_SSPSR_RNE_BIT:                       u32 = 2;
pub const SPI1_SSPSR_TNF_BIT:                       u32 = 1;
pub const SPI1_SSPSR_TFE_BIT:                       u32 = 0;
// SSPCPSR
pub const SPI1_SSPCPSR_CPSDVSR_LOW:                 u32 = 0;
pub const SPI1_SSPCPSR_CPSDVSR_HIGH:                u32 = 7;
// SSPIMSC
pub const SPI1_SSPIMSC_TXIM_BIT:                    u32 = 3;
pub const SPI1_SSPIMSC_RXIM_BIT:                    u32 = 2;
pub const SPI1_SSPIMSC_RTIM_BIT:                    u32 = 1;
pub const SPI1_SSPIMSC_RORIM_BIT:                   u32 = 0;
// SSPRIS
pub const SPI1_SSPRIS_TXRIS_BIT:                    u32 = 3;
pub const SPI1_SSPRIS_RXRIS_BIT:                    u32 = 2;
pub const SPI1_SSPRIS_RTRIS_BIT:                    u32 = 1;
pub const SPI1_SSPRIS_RORRIS_BIT:                   u32 = 0;
// SSPMIS
pub const SPI1_SSPMIS_TXMIS_BIT:                    u32 = 3;
pub const SPI1_SSPMIS_RXMIS_BIT:                    u32 = 2;
pub const SPI1_SSPMIS_RTMIS_BIT:                    u32 = 1;
pub const SPI1_SSPMIS_RORMIS_BIT:                   u32 = 0;
// SSPICR
pub const SPI1_SSPICR_RTIC_BIT:                     u32 = 1;
pub const SPI1_SSPICR_RORIC_BIT:                    u32 = 0;
// SSPDMACR
pub const SPI1_SSPDMACR_TXDMAE_BIT:                 u32 = 1;
pub const SPI1_SSPDMACR_RXDMAE_BIT:                 u32 = 0;
// SSPPERIPHID0
pub const SPI1_SSPPERIPHID0_PARTNUMBER0_LOW:        u32 = 0;
pub const SPI1_SSPPERIPHID0_PARTNUMBER0_HIGH:       u32 = 7;
// SSPPERIPHID1
pub const SPI1_SSPPERIPHID1_DESIGNER0_LOW:          u32 = 4;
pub const SPI1_SSPPERIPHID1_DESIGNER0_HIGH:         u32 = 7;
pub const SPI1_SSPPERIPHID1_PARTNUMBER1_LOW:        u32 = 0;
pub const SPI1_SSPPERIPHID1_PARTNUMBER1_HIGH:       u32 = 3;
// SSPPERIPHID2
pub const SPI1_SSPPERIPHID2_REVISION_LOW:           u32 = 4;
pub const SPI1_SSPPERIPHID2_REVISION_HIGH:          u32 = 7;
pub const SPI1_SSPPERIPHID2_DESIGNER1_LOW:          u32 = 0;
pub const SPI1_SSPPERIPHID2_DESIGNER1_HIGH:         u32 = 3;
// SSPPERIPHID3
pub const SPI1_SSPPERIPHID3_CONFIGURATION_LOW:      u32 = 0;
pub const SPI1_SSPPERIPHID3_CONFIGURATION_HIGH:     u32 = 7;
// SSPPCELLID0
pub const SPI1_SSPPCELLID0_LOW:                     u32 = 0;
pub const SPI1_SSPPCELLID0_HIGH:                    u32 = 7;
// SSPPCELLID1
pub const SPI1_SSPPCELLID1_LOW:                     u32 = 0;
pub const SPI1_SSPPCELLID1_HIGH:                    u32 = 7;
// SSPPCELLID2
pub const SPI1_SSPPCELLID2_LOW:                     u32 = 0;
pub const SPI1_SSPPCELLID2_HIGH:                    u32 = 7;
// SSPPCELLID3
pub const SPI1_SSPPCELLID3_LOW:                     u32 = 0;
pub const SPI1_SSPPCELLID3_HIGH:                    u32 = 7;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
