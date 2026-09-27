#![allow(dead_code)]
// SPI

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const SPI0_BASE:                                u32 = 0x4008_0000;
pub const SPI1_BASE:                                u32 = 0x4008_8000;

// SSPCR0..SSPCR1
pub fn SPI_SSPCR_OFFSET(n: u32) -> u32 {
    return 0x0 + n * 0x4
}

pub const SPI_SSPDR_OFFSET:                         u32 = 0x8;
pub const SPI_SSPSR_OFFSET:                         u32 = 0xC;
pub const SPI_SSPCPSR_OFFSET:                       u32 = 0x10;
pub const SPI_SSPIMSC_OFFSET:                       u32 = 0x14;
pub const SPI_SSPRIS_OFFSET:                        u32 = 0x18;
pub const SPI_SSPMIS_OFFSET:                        u32 = 0x1C;
pub const SPI_SSPICR_OFFSET:                        u32 = 0x20;
pub const SPI_SSPDMACR_OFFSET:                      u32 = 0x24;
// SSPPERIPHID0..SSPPERIPHID3
pub fn SPI_SSPPERIPHID_OFFSET(n: u32) -> u32 {
    return 0xFE0 + n * 0x4
}

// SSPPCELLID0..SSPPCELLID3
pub fn SPI_SSPPCELLID_OFFSET(n: u32) -> u32 {
    return 0xFF0 + n * 0x4
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// SSPCR0
pub const SPI_SSPCR0_SCR_LOW:                       u32 = 8;
pub const SPI_SSPCR0_SCR_HIGH:                      u32 = 15;
pub const SPI_SSPCR0_SPH_BIT:                       u32 = 7;
pub const SPI_SSPCR0_SPO_BIT:                       u32 = 6;
pub const SPI_SSPCR0_FRF_LOW:                       u32 = 4;
pub const SPI_SSPCR0_FRF_HIGH:                      u32 = 5;
pub const SPI_SSPCR0_DSS_LOW:                       u32 = 0;
pub const SPI_SSPCR0_DSS_HIGH:                      u32 = 3;

// SSPCR1
pub const SPI_SSPCR1_SOD_BIT:                       u32 = 3;
pub const SPI_SSPCR1_MS_BIT:                        u32 = 2;
pub const SPI_SSPCR1_SSE_BIT:                       u32 = 1;
pub const SPI_SSPCR1_LBM_BIT:                       u32 = 0;

// SSPDR
pub const SPI_SSPDR_DATA_LOW:                       u32 = 0;
pub const SPI_SSPDR_DATA_HIGH:                      u32 = 15;

// SSPSR
pub const SPI_SSPSR_BSY_BIT:                        u32 = 4;
pub const SPI_SSPSR_RFF_BIT:                        u32 = 3;
pub const SPI_SSPSR_RNE_BIT:                        u32 = 2;
pub const SPI_SSPSR_TNF_BIT:                        u32 = 1;
pub const SPI_SSPSR_TFE_BIT:                        u32 = 0;

// SSPCPSR
pub const SPI_SSPCPSR_CPSDVSR_LOW:                  u32 = 0;
pub const SPI_SSPCPSR_CPSDVSR_HIGH:                 u32 = 7;

// SSPIMSC
pub const SPI_SSPIMSC_TXIM_BIT:                     u32 = 3;
pub const SPI_SSPIMSC_RXIM_BIT:                     u32 = 2;
pub const SPI_SSPIMSC_RTIM_BIT:                     u32 = 1;
pub const SPI_SSPIMSC_RORIM_BIT:                    u32 = 0;

// SSPRIS
pub const SPI_SSPRIS_TXRIS_BIT:                     u32 = 3;
pub const SPI_SSPRIS_RXRIS_BIT:                     u32 = 2;
pub const SPI_SSPRIS_RTRIS_BIT:                     u32 = 1;
pub const SPI_SSPRIS_RORRIS_BIT:                    u32 = 0;

// SSPMIS
pub const SPI_SSPMIS_TXMIS_BIT:                     u32 = 3;
pub const SPI_SSPMIS_RXMIS_BIT:                     u32 = 2;
pub const SPI_SSPMIS_RTMIS_BIT:                     u32 = 1;
pub const SPI_SSPMIS_RORMIS_BIT:                    u32 = 0;

// SSPICR
pub const SPI_SSPICR_RTIC_BIT:                      u32 = 1;
pub const SPI_SSPICR_RORIC_BIT:                     u32 = 0;

// SSPDMACR
pub const SPI_SSPDMACR_TXDMAE_BIT:                  u32 = 1;
pub const SPI_SSPDMACR_RXDMAE_BIT:                  u32 = 0;

// SSPPERIPHID0
pub const SPI_SSPPERIPHID0_PARTNUMBER0_LOW:         u32 = 0;
pub const SPI_SSPPERIPHID0_PARTNUMBER0_HIGH:        u32 = 7;

// SSPPERIPHID1
pub const SPI_SSPPERIPHID1_DESIGNER0_LOW:           u32 = 4;
pub const SPI_SSPPERIPHID1_DESIGNER0_HIGH:          u32 = 7;
pub const SPI_SSPPERIPHID1_PARTNUMBER1_LOW:         u32 = 0;
pub const SPI_SSPPERIPHID1_PARTNUMBER1_HIGH:        u32 = 3;

// SSPPERIPHID2
pub const SPI_SSPPERIPHID2_REVISION_LOW:            u32 = 4;
pub const SPI_SSPPERIPHID2_REVISION_HIGH:           u32 = 7;
pub const SPI_SSPPERIPHID2_DESIGNER1_LOW:           u32 = 0;
pub const SPI_SSPPERIPHID2_DESIGNER1_HIGH:          u32 = 3;

// SSPPERIPHID3
pub const SPI_SSPPERIPHID3_CONFIGURATION_LOW:       u32 = 0;
pub const SPI_SSPPERIPHID3_CONFIGURATION_HIGH:      u32 = 7;

// SSPPCELLID0, SSPPCELLID1, SSPPCELLID2, SSPPCELLID3
pub const SPI_SSPPCELLID_LOW:                       u32 = 0;
pub const SPI_SSPPCELLID_HIGH:                      u32 = 7;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
