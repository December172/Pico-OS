#![allow(dead_code)]
// SSI
pub const SSI_BASE:                                 u32 = 0x1800_0000;
pub const SSI_CTRLR0:                               u32 = SSI_BASE + 0x0;
pub const SSI_CTRLR1:                               u32 = SSI_BASE + 0x4;
pub const SSI_SSIENR:                               u32 = SSI_BASE + 0x8;
pub const SSI_MWCR:                                 u32 = SSI_BASE + 0xC;
pub const SSI_SER:                                  u32 = SSI_BASE + 0x10;
pub const SSI_BAUDR:                                u32 = SSI_BASE + 0x14;
pub const SSI_TXFTLR:                               u32 = SSI_BASE + 0x18;
pub const SSI_RXFTLR:                               u32 = SSI_BASE + 0x1C;
pub const SSI_TXFLR:                                u32 = SSI_BASE + 0x20;
pub const SSI_RXFLR:                                u32 = SSI_BASE + 0x24;
pub const SSI_SR:                                   u32 = SSI_BASE + 0x28;
pub const SSI_IMR:                                  u32 = SSI_BASE + 0x2C;
pub const SSI_ISR:                                  u32 = SSI_BASE + 0x30;
pub const SSI_RISR:                                 u32 = SSI_BASE + 0x34;
pub const SSI_TXOICR:                               u32 = SSI_BASE + 0x38;
pub const SSI_RXOICR:                               u32 = SSI_BASE + 0x3C;
pub const SSI_RXUICR:                               u32 = SSI_BASE + 0x40;
pub const SSI_MSTICR:                               u32 = SSI_BASE + 0x44;
pub const SSI_ICR:                                  u32 = SSI_BASE + 0x48;
pub const SSI_DMACR:                                u32 = SSI_BASE + 0x4C;
pub const SSI_DMATDLR:                              u32 = SSI_BASE + 0x50;
pub const SSI_DMARDLR:                              u32 = SSI_BASE + 0x54;
pub const SSI_IDR:                                  u32 = SSI_BASE + 0x58;
pub const SSI_SSI_VERSION_ID:                       u32 = SSI_BASE + 0x5C;
pub const SSI_DR0:                                  u32 = SSI_BASE + 0x60;
pub const SSI_RX_SAMPLE_DLY:                        u32 = SSI_BASE + 0xF0;
pub const SSI_SPI_CTRLR0:                           u32 = SSI_BASE + 0xF4;
pub const SSI_TXD_DRIVE_EDGE:                       u32 = SSI_BASE + 0xF8;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CTRLR0
pub const SSI_CTRLR0_SSTE_BIT:                      u32 = 24;
pub const SSI_CTRLR0_SPI_FRF_LOW:                   u32 = 21;
pub const SSI_CTRLR0_SPI_FRF_HIGH:                  u32 = 22;
pub const SSI_CTRLR0_DFS_32_LOW:                    u32 = 16;
pub const SSI_CTRLR0_DFS_32_HIGH:                   u32 = 20;
pub const SSI_CTRLR0_CFS_LOW:                       u32 = 12;
pub const SSI_CTRLR0_CFS_HIGH:                      u32 = 15;
pub const SSI_CTRLR0_SRL_BIT:                       u32 = 11;
pub const SSI_CTRLR0_SLV_OE_BIT:                    u32 = 10;
pub const SSI_CTRLR0_TMOD_LOW:                      u32 = 8;
pub const SSI_CTRLR0_TMOD_HIGH:                     u32 = 9;
pub const SSI_CTRLR0_SCPOL_BIT:                     u32 = 7;
pub const SSI_CTRLR0_SCPH_BIT:                      u32 = 6;
pub const SSI_CTRLR0_FRF_LOW:                       u32 = 4;
pub const SSI_CTRLR0_FRF_HIGH:                      u32 = 5;
pub const SSI_CTRLR0_DFS_LOW:                       u32 = 0;
pub const SSI_CTRLR0_DFS_HIGH:                      u32 = 3;
// CTRLR1
pub const SSI_CTRLR1_NDF_LOW:                       u32 = 0;
pub const SSI_CTRLR1_NDF_HIGH:                      u32 = 15;
// SSIENR
pub const SSI_SSIENR_SSI_EN_BIT:                    u32 = 0;
// MWCR
pub const SSI_MWCR_MHS_BIT:                         u32 = 2;
pub const SSI_MWCR_MDD_BIT:                         u32 = 1;
pub const SSI_MWCR_MWMOD_BIT:                       u32 = 0;
// SER
pub const SSI_SER_BIT:                              u32 = 0;
// BAUDR
pub const SSI_BAUDR_SCKDV_LOW:                      u32 = 0;
pub const SSI_BAUDR_SCKDV_HIGH:                     u32 = 15;
// TXFTLR
pub const SSI_TXFTLR_TFT_LOW:                       u32 = 0;
pub const SSI_TXFTLR_TFT_HIGH:                      u32 = 7;
// RXFTLR
pub const SSI_RXFTLR_RFT_LOW:                       u32 = 0;
pub const SSI_RXFTLR_RFT_HIGH:                      u32 = 7;
// TXFLR
pub const SSI_TXFLR_TFTFL_LOW:                      u32 = 0;
pub const SSI_TXFLR_TFTFL_HIGH:                     u32 = 7;
// RXFLR
pub const SSI_RXFLR_RXTFL_LOW:                      u32 = 0;
pub const SSI_RXFLR_RXTFL_HIGH:                     u32 = 7;
// SR
pub const SSI_SR_DCOL_BIT:                          u32 = 6;
pub const SSI_SR_TXE_BIT:                           u32 = 5;
pub const SSI_SR_RFF_BIT:                           u32 = 4;
pub const SSI_SR_RFNE_BIT:                          u32 = 3;
pub const SSI_SR_TFE_BIT:                           u32 = 2;
pub const SSI_SR_TFNF_BIT:                          u32 = 1;
pub const SSI_SR_BUSY_BIT:                          u32 = 0;
// IMR
pub const SSI_IMR_MSTIM_BIT:                        u32 = 5;
pub const SSI_IMR_RXFIM_BIT:                        u32 = 4;
pub const SSI_IMR_RXOIM_BIT:                        u32 = 3;
pub const SSI_IMR_RXUIM_BIT:                        u32 = 2;
pub const SSI_IMR_TXOIM_BIT:                        u32 = 1;
pub const SSI_IMR_TXEIM_BIT:                        u32 = 0;
// ISR
pub const SSI_ISR_MSTIS_BIT:                        u32 = 5;
pub const SSI_ISR_RXFIS_BIT:                        u32 = 4;
pub const SSI_ISR_RXOIS_BIT:                        u32 = 3;
pub const SSI_ISR_RXUIS_BIT:                        u32 = 2;
pub const SSI_ISR_TXOIS_BIT:                        u32 = 1;
pub const SSI_ISR_TXEIS_BIT:                        u32 = 0;
// RISR
pub const SSI_RISR_MSTIR_BIT:                       u32 = 5;
pub const SSI_RISR_RXFIR_BIT:                       u32 = 4;
pub const SSI_RISR_RXOIR_BIT:                       u32 = 3;
pub const SSI_RISR_RXUIR_BIT:                       u32 = 2;
pub const SSI_RISR_TXOIR_BIT:                       u32 = 1;
pub const SSI_RISR_TXEIR_BIT:                       u32 = 0;
// TXOICR
pub const SSI_TXOICR_BIT:                           u32 = 0;
// RXOICR
pub const SSI_RXOICR_BIT:                           u32 = 0;
// RXUICR
pub const SSI_RXUICR_BIT:                           u32 = 0;
// MSTICR
pub const SSI_MSTICR_BIT:                           u32 = 0;
// ICR
pub const SSI_ICR_BIT:                              u32 = 0;
// DMACR
pub const SSI_DMACR_TDMAE_BIT:                      u32 = 1;
pub const SSI_DMACR_RDMAE_BIT:                      u32 = 0;
// DMATDLR
pub const SSI_DMATDLR_DMATDL_LOW:                   u32 = 0;
pub const SSI_DMATDLR_DMATDL_HIGH:                  u32 = 7;
// DMARDLR
pub const SSI_DMARDLR_DMARDL_LOW:                   u32 = 0;
pub const SSI_DMARDLR_DMARDL_HIGH:                  u32 = 7;
// IDR
pub const SSI_IDR_IDCODE_LOW:                       u32 = 0;
pub const SSI_IDR_IDCODE_HIGH:                      u32 = 31;
// SSI_VERSION_ID
pub const SSI_SSI_VERSION_ID_SSI_COMP_VERSION_LOW:  u32 = 0;
pub const SSI_SSI_VERSION_ID_SSI_COMP_VERSION_HIGH: u32 = 31;
// DR0
pub const SSI_DR0_DR_LOW:                           u32 = 0;
pub const SSI_DR0_DR_HIGH:                          u32 = 31;
// RX_SAMPLE_DLY
pub const SSI_RX_SAMPLE_DLY_RSD_LOW:                u32 = 0;
pub const SSI_RX_SAMPLE_DLY_RSD_HIGH:               u32 = 7;
// SPI_CTRLR0
pub const SSI_SPI_CTRLR0_XIP_CMD_LOW:               u32 = 24;
pub const SSI_SPI_CTRLR0_XIP_CMD_HIGH:              u32 = 31;
pub const SSI_SPI_CTRLR0_SPI_RXDS_EN_BIT:           u32 = 18;
pub const SSI_SPI_CTRLR0_INST_DDR_EN_BIT:           u32 = 17;
pub const SSI_SPI_CTRLR0_SPI_DDR_EN_BIT:            u32 = 16;
pub const SSI_SPI_CTRLR0_WAIT_CYCLES_LOW:           u32 = 11;
pub const SSI_SPI_CTRLR0_WAIT_CYCLES_HIGH:          u32 = 15;
pub const SSI_SPI_CTRLR0_INST_L_LOW:                u32 = 8;
pub const SSI_SPI_CTRLR0_INST_L_HIGH:               u32 = 9;
pub const SSI_SPI_CTRLR0_ADDR_L_LOW:                u32 = 2;
pub const SSI_SPI_CTRLR0_ADDR_L_HIGH:               u32 = 5;
pub const SSI_SPI_CTRLR0_TRANS_TYPE_LOW:            u32 = 0;
pub const SSI_SPI_CTRLR0_TRANS_TYPE_HIGH:           u32 = 1;
// TXD_DRIVE_EDGE
pub const SSI_TXD_DRIVE_EDGE_TDE_LOW:               u32 = 0;
pub const SSI_TXD_DRIVE_EDGE_TDE_HIGH:              u32 = 7;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
