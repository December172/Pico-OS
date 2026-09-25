#![allow(dead_code)]
// UART0
pub const UART0_BASE:                               u32 = 0x4003_4000;
pub const UART0_UARTDR:                             u32 = UART0_BASE + 0x0;
pub const UART0_UARTRSR:                            u32 = UART0_BASE + 0x4;
pub const UART0_UARTFR:                             u32 = UART0_BASE + 0x18;
pub const UART0_UARTILPR:                           u32 = UART0_BASE + 0x20;
pub const UART0_UARTIBRD:                           u32 = UART0_BASE + 0x24;
pub const UART0_UARTFBRD:                           u32 = UART0_BASE + 0x28;
pub const UART0_UARTLCR_H:                          u32 = UART0_BASE + 0x2C;
pub const UART0_UARTCR:                             u32 = UART0_BASE + 0x30;
pub const UART0_UARTIFLS:                           u32 = UART0_BASE + 0x34;
pub const UART0_UARTIMSC:                           u32 = UART0_BASE + 0x38;
pub const UART0_UARTRIS:                            u32 = UART0_BASE + 0x3C;
pub const UART0_UARTMIS:                            u32 = UART0_BASE + 0x40;
pub const UART0_UARTICR:                            u32 = UART0_BASE + 0x44;
pub const UART0_UARTDMACR:                          u32 = UART0_BASE + 0x48;
pub const UART0_UARTPERIPHID0:                      u32 = UART0_BASE + 0xFE0;
pub const UART0_UARTPERIPHID1:                      u32 = UART0_BASE + 0xFE4;
pub const UART0_UARTPERIPHID2:                      u32 = UART0_BASE + 0xFE8;
pub const UART0_UARTPERIPHID3:                      u32 = UART0_BASE + 0xFEC;
pub const UART0_UARTPCELLID0:                       u32 = UART0_BASE + 0xFF0;
pub const UART0_UARTPCELLID1:                       u32 = UART0_BASE + 0xFF4;
pub const UART0_UARTPCELLID2:                       u32 = UART0_BASE + 0xFF8;
pub const UART0_UARTPCELLID3:                       u32 = UART0_BASE + 0xFFC;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// UARTDR
pub const UART0_UARTDR_OE_BIT:                      u32 = 11;
pub const UART0_UARTDR_BE_BIT:                      u32 = 10;
pub const UART0_UARTDR_PE_BIT:                      u32 = 9;
pub const UART0_UARTDR_FE_BIT:                      u32 = 8;
pub const UART0_UARTDR_DATA_LOW:                    u32 = 0;
pub const UART0_UARTDR_DATA_HIGH:                   u32 = 7;
// UARTRSR
pub const UART0_UARTRSR_OE_BIT:                     u32 = 3;
pub const UART0_UARTRSR_BE_BIT:                     u32 = 2;
pub const UART0_UARTRSR_PE_BIT:                     u32 = 1;
pub const UART0_UARTRSR_FE_BIT:                     u32 = 0;
// UARTFR
pub const UART0_UARTFR_RI_BIT:                      u32 = 8;
pub const UART0_UARTFR_TXFE_BIT:                    u32 = 7;
pub const UART0_UARTFR_RXFF_BIT:                    u32 = 6;
pub const UART0_UARTFR_TXFF_BIT:                    u32 = 5;
pub const UART0_UARTFR_RXFE_BIT:                    u32 = 4;
pub const UART0_UARTFR_BUSY_BIT:                    u32 = 3;
pub const UART0_UARTFR_DCD_BIT:                     u32 = 2;
pub const UART0_UARTFR_DSR_BIT:                     u32 = 1;
pub const UART0_UARTFR_CTS_BIT:                     u32 = 0;
// UARTILPR
pub const UART0_UARTILPR_ILPDVSR_LOW:               u32 = 0;
pub const UART0_UARTILPR_ILPDVSR_HIGH:              u32 = 7;
// UARTIBRD
pub const UART0_UARTIBRD_BAUD_DIVINT_LOW:           u32 = 0;
pub const UART0_UARTIBRD_BAUD_DIVINT_HIGH:          u32 = 15;
// UARTFBRD
pub const UART0_UARTFBRD_BAUD_DIVFRAC_LOW:          u32 = 0;
pub const UART0_UARTFBRD_BAUD_DIVFRAC_HIGH:         u32 = 5;
// UARTLCR_H
pub const UART0_UARTLCR_H_SPS_BIT:                  u32 = 7;
pub const UART0_UARTLCR_H_WLEN_LOW:                 u32 = 5;
pub const UART0_UARTLCR_H_WLEN_HIGH:                u32 = 6;
pub const UART0_UARTLCR_H_FEN_BIT:                  u32 = 4;
pub const UART0_UARTLCR_H_STP2_BIT:                 u32 = 3;
pub const UART0_UARTLCR_H_EPS_BIT:                  u32 = 2;
pub const UART0_UARTLCR_H_PEN_BIT:                  u32 = 1;
pub const UART0_UARTLCR_H_BRK_BIT:                  u32 = 0;
// UARTCR
pub const UART0_UARTCR_CTSEN_BIT:                   u32 = 15;
pub const UART0_UARTCR_RTSEN_BIT:                   u32 = 14;
pub const UART0_UARTCR_OUT2_BIT:                    u32 = 13;
pub const UART0_UARTCR_OUT1_BIT:                    u32 = 12;
pub const UART0_UARTCR_RTS_BIT:                     u32 = 11;
pub const UART0_UARTCR_DTR_BIT:                     u32 = 10;
pub const UART0_UARTCR_RXE_BIT:                     u32 = 9;
pub const UART0_UARTCR_TXE_BIT:                     u32 = 8;
pub const UART0_UARTCR_LBE_BIT:                     u32 = 7;
pub const UART0_UARTCR_SIRLP_BIT:                   u32 = 2;
pub const UART0_UARTCR_SIREN_BIT:                   u32 = 1;
pub const UART0_UARTCR_UARTEN_BIT:                  u32 = 0;
// UARTIFLS
pub const UART0_UARTIFLS_RXIFLSEL_LOW:              u32 = 3;
pub const UART0_UARTIFLS_RXIFLSEL_HIGH:             u32 = 5;
pub const UART0_UARTIFLS_TXIFLSEL_LOW:              u32 = 0;
pub const UART0_UARTIFLS_TXIFLSEL_HIGH:             u32 = 2;
// UARTIMSC
pub const UART0_UARTIMSC_OEIM_BIT:                  u32 = 10;
pub const UART0_UARTIMSC_BEIM_BIT:                  u32 = 9;
pub const UART0_UARTIMSC_PEIM_BIT:                  u32 = 8;
pub const UART0_UARTIMSC_FEIM_BIT:                  u32 = 7;
pub const UART0_UARTIMSC_RTIM_BIT:                  u32 = 6;
pub const UART0_UARTIMSC_TXIM_BIT:                  u32 = 5;
pub const UART0_UARTIMSC_RXIM_BIT:                  u32 = 4;
pub const UART0_UARTIMSC_DSRMIM_BIT:                u32 = 3;
pub const UART0_UARTIMSC_DCDMIM_BIT:                u32 = 2;
pub const UART0_UARTIMSC_CTSMIM_BIT:                u32 = 1;
pub const UART0_UARTIMSC_RIMIM_BIT:                 u32 = 0;
// UARTRIS
pub const UART0_UARTRIS_OERIS_BIT:                  u32 = 10;
pub const UART0_UARTRIS_BERIS_BIT:                  u32 = 9;
pub const UART0_UARTRIS_PERIS_BIT:                  u32 = 8;
pub const UART0_UARTRIS_FERIS_BIT:                  u32 = 7;
pub const UART0_UARTRIS_RTRIS_BIT:                  u32 = 6;
pub const UART0_UARTRIS_TXRIS_BIT:                  u32 = 5;
pub const UART0_UARTRIS_RXRIS_BIT:                  u32 = 4;
pub const UART0_UARTRIS_DSRRMIS_BIT:                u32 = 3;
pub const UART0_UARTRIS_DCDRMIS_BIT:                u32 = 2;
pub const UART0_UARTRIS_CTSRMIS_BIT:                u32 = 1;
pub const UART0_UARTRIS_RIRMIS_BIT:                 u32 = 0;
// UARTMIS
pub const UART0_UARTMIS_OEMIS_BIT:                  u32 = 10;
pub const UART0_UARTMIS_BEMIS_BIT:                  u32 = 9;
pub const UART0_UARTMIS_PEMIS_BIT:                  u32 = 8;
pub const UART0_UARTMIS_FEMIS_BIT:                  u32 = 7;
pub const UART0_UARTMIS_RTMIS_BIT:                  u32 = 6;
pub const UART0_UARTMIS_TXMIS_BIT:                  u32 = 5;
pub const UART0_UARTMIS_RXMIS_BIT:                  u32 = 4;
pub const UART0_UARTMIS_DSRMMIS_BIT:                u32 = 3;
pub const UART0_UARTMIS_DCDMMIS_BIT:                u32 = 2;
pub const UART0_UARTMIS_CTSMMIS_BIT:                u32 = 1;
pub const UART0_UARTMIS_RIMMIS_BIT:                 u32 = 0;
// UARTICR
pub const UART0_UARTICR_OEIC_BIT:                   u32 = 10;
pub const UART0_UARTICR_BEIC_BIT:                   u32 = 9;
pub const UART0_UARTICR_PEIC_BIT:                   u32 = 8;
pub const UART0_UARTICR_FEIC_BIT:                   u32 = 7;
pub const UART0_UARTICR_RTIC_BIT:                   u32 = 6;
pub const UART0_UARTICR_TXIC_BIT:                   u32 = 5;
pub const UART0_UARTICR_RXIC_BIT:                   u32 = 4;
pub const UART0_UARTICR_DSRMIC_BIT:                 u32 = 3;
pub const UART0_UARTICR_DCDMIC_BIT:                 u32 = 2;
pub const UART0_UARTICR_CTSMIC_BIT:                 u32 = 1;
pub const UART0_UARTICR_RIMIC_BIT:                  u32 = 0;
// UARTDMACR
pub const UART0_UARTDMACR_DMAONERR_BIT:             u32 = 2;
pub const UART0_UARTDMACR_TXDMAE_BIT:               u32 = 1;
pub const UART0_UARTDMACR_RXDMAE_BIT:               u32 = 0;
// UARTPERIPHID0
pub const UART0_UARTPERIPHID0_PARTNUMBER0_LOW:      u32 = 0;
pub const UART0_UARTPERIPHID0_PARTNUMBER0_HIGH:     u32 = 7;
// UARTPERIPHID1
pub const UART0_UARTPERIPHID1_DESIGNER0_LOW:        u32 = 4;
pub const UART0_UARTPERIPHID1_DESIGNER0_HIGH:       u32 = 7;
pub const UART0_UARTPERIPHID1_PARTNUMBER1_LOW:      u32 = 0;
pub const UART0_UARTPERIPHID1_PARTNUMBER1_HIGH:     u32 = 3;
// UARTPERIPHID2
pub const UART0_UARTPERIPHID2_REVISION_LOW:         u32 = 4;
pub const UART0_UARTPERIPHID2_REVISION_HIGH:        u32 = 7;
pub const UART0_UARTPERIPHID2_DESIGNER1_LOW:        u32 = 0;
pub const UART0_UARTPERIPHID2_DESIGNER1_HIGH:       u32 = 3;
// UARTPERIPHID3
pub const UART0_UARTPERIPHID3_CONFIGURATION_LOW:    u32 = 0;
pub const UART0_UARTPERIPHID3_CONFIGURATION_HIGH:   u32 = 7;
// UARTPCELLID0
pub const UART0_UARTPCELLID0_LOW:                   u32 = 0;
pub const UART0_UARTPCELLID0_HIGH:                  u32 = 7;
// UARTPCELLID1
pub const UART0_UARTPCELLID1_LOW:                   u32 = 0;
pub const UART0_UARTPCELLID1_HIGH:                  u32 = 7;
// UARTPCELLID2
pub const UART0_UARTPCELLID2_LOW:                   u32 = 0;
pub const UART0_UARTPCELLID2_HIGH:                  u32 = 7;
// UARTPCELLID3
pub const UART0_UARTPCELLID3_LOW:                   u32 = 0;
pub const UART0_UARTPCELLID3_HIGH:                  u32 = 7;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
