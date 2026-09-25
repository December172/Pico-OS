#![allow(dead_code)]
// I2C0
pub const I2C0_BASE:                                u32 = 0x4004_4000;
pub const I2C0_IC_CON:                              u32 = I2C0_BASE + 0x0;
pub const I2C0_IC_TAR:                              u32 = I2C0_BASE + 0x4;
pub const I2C0_IC_SAR:                              u32 = I2C0_BASE + 0x8;
pub const I2C0_IC_DATA_CMD:                         u32 = I2C0_BASE + 0x10;
pub const I2C0_IC_SS_SCL_HCNT:                      u32 = I2C0_BASE + 0x14;
pub const I2C0_IC_SS_SCL_LCNT:                      u32 = I2C0_BASE + 0x18;
pub const I2C0_IC_FS_SCL_HCNT:                      u32 = I2C0_BASE + 0x1C;
pub const I2C0_IC_FS_SCL_LCNT:                      u32 = I2C0_BASE + 0x20;
pub const I2C0_IC_INTR_STAT:                        u32 = I2C0_BASE + 0x2C;
pub const I2C0_IC_INTR_MASK:                        u32 = I2C0_BASE + 0x30;
pub const I2C0_IC_RAW_INTR_STAT:                    u32 = I2C0_BASE + 0x34;
pub const I2C0_IC_RX_TL:                            u32 = I2C0_BASE + 0x38;
pub const I2C0_IC_TX_TL:                            u32 = I2C0_BASE + 0x3C;
pub const I2C0_IC_CLR_INTR:                         u32 = I2C0_BASE + 0x40;
pub const I2C0_IC_CLR_RX_UNDER:                     u32 = I2C0_BASE + 0x44;
pub const I2C0_IC_CLR_RX_OVER:                      u32 = I2C0_BASE + 0x48;
pub const I2C0_IC_CLR_TX_OVER:                      u32 = I2C0_BASE + 0x4C;
pub const I2C0_IC_CLR_RD_REQ:                       u32 = I2C0_BASE + 0x50;
pub const I2C0_IC_CLR_TX_ABRT:                      u32 = I2C0_BASE + 0x54;
pub const I2C0_IC_CLR_RX_DONE:                      u32 = I2C0_BASE + 0x58;
pub const I2C0_IC_CLR_ACTIVITY:                     u32 = I2C0_BASE + 0x5C;
pub const I2C0_IC_CLR_STOP_DET:                     u32 = I2C0_BASE + 0x60;
pub const I2C0_IC_CLR_START_DET:                    u32 = I2C0_BASE + 0x64;
pub const I2C0_IC_CLR_GEN_CALL:                     u32 = I2C0_BASE + 0x68;
pub const I2C0_IC_ENABLE:                           u32 = I2C0_BASE + 0x6C;
pub const I2C0_IC_STATUS:                           u32 = I2C0_BASE + 0x70;
pub const I2C0_IC_TXFLR:                            u32 = I2C0_BASE + 0x74;
pub const I2C0_IC_RXFLR:                            u32 = I2C0_BASE + 0x78;
pub const I2C0_IC_SDA_HOLD:                         u32 = I2C0_BASE + 0x7C;
pub const I2C0_IC_TX_ABRT_SOURCE:                   u32 = I2C0_BASE + 0x80;
pub const I2C0_IC_SLV_DATA_NACK_ONLY:               u32 = I2C0_BASE + 0x84;
pub const I2C0_IC_DMA_CR:                           u32 = I2C0_BASE + 0x88;
pub const I2C0_IC_DMA_TDLR:                         u32 = I2C0_BASE + 0x8C;
pub const I2C0_IC_DMA_RDLR:                         u32 = I2C0_BASE + 0x90;
pub const I2C0_IC_SDA_SETUP:                        u32 = I2C0_BASE + 0x94;
pub const I2C0_IC_ACK_GENERAL_CALL:                 u32 = I2C0_BASE + 0x98;
pub const I2C0_IC_ENABLE_STATUS:                    u32 = I2C0_BASE + 0x9C;
pub const I2C0_IC_FS_SPKLEN:                        u32 = I2C0_BASE + 0xA0;
pub const I2C0_IC_CLR_RESTART_DET:                  u32 = I2C0_BASE + 0xA8;
pub const I2C0_IC_COMP_PARAM_1:                     u32 = I2C0_BASE + 0xF4;
pub const I2C0_IC_COMP_VERSION:                     u32 = I2C0_BASE + 0xF8;
pub const I2C0_IC_COMP_TYPE:                        u32 = I2C0_BASE + 0xFC;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// IC_CON
pub const I2C0_IC_CON_STOP_DET_IF_MASTER_ACTIVE_BIT:u32 = 10;
pub const I2C0_IC_CON_RX_FIFO_FULL_HLD_CTRL_BIT:    u32 = 9;
pub const I2C0_IC_CON_TX_EMPTY_CTRL_BIT:            u32 = 8;
pub const I2C0_IC_CON_STOP_DET_IFADDRESSED_BIT:     u32 = 7;
pub const I2C0_IC_CON_IC_SLAVE_DISABLE_BIT:         u32 = 6;
pub const I2C0_IC_CON_IC_RESTART_EN_BIT:            u32 = 5;
pub const I2C0_IC_CON_IC_10BITADDR_MASTER_BIT:      u32 = 4;
pub const I2C0_IC_CON_IC_10BITADDR_SLAVE_BIT:       u32 = 3;
pub const I2C0_IC_CON_SPEED_LOW:                    u32 = 1;
pub const I2C0_IC_CON_SPEED_HIGH:                   u32 = 2;
pub const I2C0_IC_CON_MASTER_MODE_BIT:              u32 = 0;
// IC_TAR
pub const I2C0_IC_TAR_SPECIAL_BIT:                  u32 = 11;
pub const I2C0_IC_TAR_GC_OR_START_BIT:              u32 = 10;
pub const I2C0_IC_TAR_LOW:                          u32 = 0;
pub const I2C0_IC_TAR_HIGH:                         u32 = 9;
// IC_SAR
pub const I2C0_IC_SAR_LOW:                          u32 = 0;
pub const I2C0_IC_SAR_HIGH:                         u32 = 9;
// IC_DATA_CMD
pub const I2C0_IC_DATA_CMD_FIRST_DATA_BYTE_BIT:     u32 = 11;
pub const I2C0_IC_DATA_CMD_RESTART_BIT:             u32 = 10;
pub const I2C0_IC_DATA_CMD_STOP_BIT:                u32 = 9;
pub const I2C0_IC_DATA_CMD_CMD_BIT:                 u32 = 8;
pub const I2C0_IC_DATA_CMD_DAT_LOW:                 u32 = 0;
pub const I2C0_IC_DATA_CMD_DAT_HIGH:                u32 = 7;
// IC_SS_SCL_HCNT
pub const I2C0_IC_SS_SCL_HCNT_LOW:                  u32 = 0;
pub const I2C0_IC_SS_SCL_HCNT_HIGH:                 u32 = 15;
// IC_SS_SCL_LCNT
pub const I2C0_IC_SS_SCL_LCNT_LOW:                  u32 = 0;
pub const I2C0_IC_SS_SCL_LCNT_HIGH:                 u32 = 15;
// IC_FS_SCL_HCNT
pub const I2C0_IC_FS_SCL_HCNT_LOW:                  u32 = 0;
pub const I2C0_IC_FS_SCL_HCNT_HIGH:                 u32 = 15;
// IC_FS_SCL_LCNT
pub const I2C0_IC_FS_SCL_LCNT_LOW:                  u32 = 0;
pub const I2C0_IC_FS_SCL_LCNT_HIGH:                 u32 = 15;
// IC_INTR_STAT
pub const I2C0_IC_INTR_STAT_R_RESTART_DET_BIT:      u32 = 12;
pub const I2C0_IC_INTR_STAT_R_GEN_CALL_BIT:         u32 = 11;
pub const I2C0_IC_INTR_STAT_R_START_DET_BIT:        u32 = 10;
pub const I2C0_IC_INTR_STAT_R_STOP_DET_BIT:         u32 = 9;
pub const I2C0_IC_INTR_STAT_R_ACTIVITY_BIT:         u32 = 8;
pub const I2C0_IC_INTR_STAT_R_RX_DONE_BIT:          u32 = 7;
pub const I2C0_IC_INTR_STAT_R_TX_ABRT_BIT:          u32 = 6;
pub const I2C0_IC_INTR_STAT_R_RD_REQ_BIT:           u32 = 5;
pub const I2C0_IC_INTR_STAT_R_TX_EMPTY_BIT:         u32 = 4;
pub const I2C0_IC_INTR_STAT_R_TX_OVER_BIT:          u32 = 3;
pub const I2C0_IC_INTR_STAT_R_RX_FULL_BIT:          u32 = 2;
pub const I2C0_IC_INTR_STAT_R_RX_OVER_BIT:          u32 = 1;
pub const I2C0_IC_INTR_STAT_R_RX_UNDER_BIT:         u32 = 0;
// IC_INTR_MASK
pub const I2C0_IC_INTR_MASK_M_RESTART_DET_BIT:      u32 = 12;
pub const I2C0_IC_INTR_MASK_M_GEN_CALL_BIT:         u32 = 11;
pub const I2C0_IC_INTR_MASK_M_START_DET_BIT:        u32 = 10;
pub const I2C0_IC_INTR_MASK_M_STOP_DET_BIT:         u32 = 9;
pub const I2C0_IC_INTR_MASK_M_ACTIVITY_BIT:         u32 = 8;
pub const I2C0_IC_INTR_MASK_M_RX_DONE_BIT:          u32 = 7;
pub const I2C0_IC_INTR_MASK_M_TX_ABRT_BIT:          u32 = 6;
pub const I2C0_IC_INTR_MASK_M_RD_REQ_BIT:           u32 = 5;
pub const I2C0_IC_INTR_MASK_M_TX_EMPTY_BIT:         u32 = 4;
pub const I2C0_IC_INTR_MASK_M_TX_OVER_BIT:          u32 = 3;
pub const I2C0_IC_INTR_MASK_M_RX_FULL_BIT:          u32 = 2;
pub const I2C0_IC_INTR_MASK_M_RX_OVER_BIT:          u32 = 1;
pub const I2C0_IC_INTR_MASK_M_RX_UNDER_BIT:         u32 = 0;
// IC_RAW_INTR_STAT
pub const I2C0_IC_RAW_INTR_STAT_RESTART_DET_BIT:    u32 = 12;
pub const I2C0_IC_RAW_INTR_STAT_GEN_CALL_BIT:       u32 = 11;
pub const I2C0_IC_RAW_INTR_STAT_START_DET_BIT:      u32 = 10;
pub const I2C0_IC_RAW_INTR_STAT_STOP_DET_BIT:       u32 = 9;
pub const I2C0_IC_RAW_INTR_STAT_ACTIVITY_BIT:       u32 = 8;
pub const I2C0_IC_RAW_INTR_STAT_RX_DONE_BIT:        u32 = 7;
pub const I2C0_IC_RAW_INTR_STAT_TX_ABRT_BIT:        u32 = 6;
pub const I2C0_IC_RAW_INTR_STAT_RD_REQ_BIT:         u32 = 5;
pub const I2C0_IC_RAW_INTR_STAT_TX_EMPTY_BIT:       u32 = 4;
pub const I2C0_IC_RAW_INTR_STAT_TX_OVER_BIT:        u32 = 3;
pub const I2C0_IC_RAW_INTR_STAT_RX_FULL_BIT:        u32 = 2;
pub const I2C0_IC_RAW_INTR_STAT_RX_OVER_BIT:        u32 = 1;
pub const I2C0_IC_RAW_INTR_STAT_RX_UNDER_BIT:       u32 = 0;
// IC_RX_TL
pub const I2C0_IC_RX_TL_RX_TL_LOW:                  u32 = 0;
pub const I2C0_IC_RX_TL_RX_TL_HIGH:                 u32 = 7;
// IC_TX_TL
pub const I2C0_IC_TX_TL_TX_TL_LOW:                  u32 = 0;
pub const I2C0_IC_TX_TL_TX_TL_HIGH:                 u32 = 7;
// IC_CLR_INTR
pub const I2C0_IC_CLR_INTR_CLR_INTR_BIT:            u32 = 0;
// IC_CLR_RX_UNDER
pub const I2C0_IC_CLR_RX_UNDER_CLR_RX_UNDER_BIT:    u32 = 0;
// IC_CLR_RX_OVER
pub const I2C0_IC_CLR_RX_OVER_CLR_RX_OVER_BIT:      u32 = 0;
// IC_CLR_TX_OVER
pub const I2C0_IC_CLR_TX_OVER_CLR_TX_OVER_BIT:      u32 = 0;
// IC_CLR_RD_REQ
pub const I2C0_IC_CLR_RD_REQ_CLR_RD_REQ_BIT:        u32 = 0;
// IC_CLR_TX_ABRT
pub const I2C0_IC_CLR_TX_ABRT_CLR_TX_ABRT_BIT:      u32 = 0;
// IC_CLR_RX_DONE
pub const I2C0_IC_CLR_RX_DONE_CLR_RX_DONE_BIT:      u32 = 0;
// IC_CLR_ACTIVITY
pub const I2C0_IC_CLR_ACTIVITY_CLR_ACTIVITY_BIT:    u32 = 0;
// IC_CLR_STOP_DET
pub const I2C0_IC_CLR_STOP_DET_CLR_STOP_DET_BIT:    u32 = 0;
// IC_CLR_START_DET
pub const I2C0_IC_CLR_START_DET_CLR_START_DET_BIT:  u32 = 0;
// IC_CLR_GEN_CALL
pub const I2C0_IC_CLR_GEN_CALL_CLR_GEN_CALL_BIT:    u32 = 0;
// IC_ENABLE
pub const I2C0_IC_ENABLE_TX_CMD_BLOCK_BIT:          u32 = 2;
pub const I2C0_IC_ENABLE_ABORT_BIT:                 u32 = 1;
pub const I2C0_IC_ENABLE_ENABLE_BIT:                u32 = 0;
// IC_STATUS
pub const I2C0_IC_STATUS_SLV_ACTIVITY_BIT:          u32 = 6;
pub const I2C0_IC_STATUS_MST_ACTIVITY_BIT:          u32 = 5;
pub const I2C0_IC_STATUS_RFF_BIT:                   u32 = 4;
pub const I2C0_IC_STATUS_RFNE_BIT:                  u32 = 3;
pub const I2C0_IC_STATUS_TFE_BIT:                   u32 = 2;
pub const I2C0_IC_STATUS_TFNF_BIT:                  u32 = 1;
pub const I2C0_IC_STATUS_ACTIVITY_BIT:              u32 = 0;
// IC_TXFLR
pub const I2C0_IC_TXFLR_TXFLR_LOW:                  u32 = 0;
pub const I2C0_IC_TXFLR_TXFLR_HIGH:                 u32 = 4;
// IC_RXFLR
pub const I2C0_IC_RXFLR_RXFLR_LOW:                  u32 = 0;
pub const I2C0_IC_RXFLR_RXFLR_HIGH:                 u32 = 4;
// IC_SDA_HOLD
pub const I2C0_IC_SDA_HOLD_IC_SDA_RX_HOLD_LOW:      u32 = 16;
pub const I2C0_IC_SDA_HOLD_IC_SDA_RX_HOLD_HIGH:     u32 = 23;
pub const I2C0_IC_SDA_HOLD_IC_SDA_TX_HOLD_LOW:      u32 = 0;
pub const I2C0_IC_SDA_HOLD_IC_SDA_TX_HOLD_HIGH:     u32 = 15;
// IC_TX_ABRT_SOURCE
pub const I2C0_IC_TX_ABRT_SOURCE_TX_FLUSH_CNT_LOW:  u32 = 23;
pub const I2C0_IC_TX_ABRT_SOURCE_TX_FLUSH_CNT_HIGH: u32 = 31;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_USER_ABRT_BIT:u32 = 16;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_SLVRD_INTX_BIT:u32 = 15;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_SLV_ARBLOST_BIT:u32 = 14;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_SLVFLUSH_TXFIFO_BIT:u32 = 13;
pub const I2C0_IC_TX_ABRT_SOURCE_ARB_LOST_BIT:      u32 = 12;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_MASTER_DIS_BIT:u32 = 11;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_10B_RD_NORSTRT_BIT:u32 = 10;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_SBYTE_NORSTRT_BIT:u32 = 9;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_HS_NORSTRT_BIT:u32 = 8;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_SBYTE_ACKDET_BIT:u32 = 7;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_HS_ACKDET_BIT:u32 = 6;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_GCALL_READ_BIT:u32 = 5;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_GCALL_NOACK_BIT:u32 = 4;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_TXDATA_NOACK_BIT:u32 = 3;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_10ADDR2_NOACK_BIT:u32 = 2;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_10ADDR1_NOACK_BIT:u32 = 1;
pub const I2C0_IC_TX_ABRT_SOURCE_ABRT_7B_ADDR_NOACK_BIT:u32 = 0;
// IC_SLV_DATA_NACK_ONLY
pub const I2C0_IC_SLV_DATA_NACK_ONLY_NACK_BIT:      u32 = 0;
// IC_DMA_CR
pub const I2C0_IC_DMA_CR_TDMAE_BIT:                 u32 = 1;
pub const I2C0_IC_DMA_CR_RDMAE_BIT:                 u32 = 0;
// IC_DMA_TDLR
pub const I2C0_IC_DMA_TDLR_DMATDL_LOW:              u32 = 0;
pub const I2C0_IC_DMA_TDLR_DMATDL_HIGH:             u32 = 3;
// IC_DMA_RDLR
pub const I2C0_IC_DMA_RDLR_DMARDL_LOW:              u32 = 0;
pub const I2C0_IC_DMA_RDLR_DMARDL_HIGH:             u32 = 3;
// IC_SDA_SETUP
pub const I2C0_IC_SDA_SETUP_SDA_SETUP_LOW:          u32 = 0;
pub const I2C0_IC_SDA_SETUP_SDA_SETUP_HIGH:         u32 = 7;
// IC_ACK_GENERAL_CALL
pub const I2C0_IC_ACK_GENERAL_CALL_ACK_GEN_CALL_BIT:u32 = 0;
// IC_ENABLE_STATUS
pub const I2C0_IC_ENABLE_STATUS_SLV_RX_DATA_LOST_BIT:u32 = 2;
pub const I2C0_IC_ENABLE_STATUS_SLV_DISABLED_WHILE_BUSY_BIT:u32 = 1;
pub const I2C0_IC_ENABLE_STATUS_IC_EN_BIT:          u32 = 0;
// IC_FS_SPKLEN
pub const I2C0_IC_FS_SPKLEN_LOW:                    u32 = 0;
pub const I2C0_IC_FS_SPKLEN_HIGH:                   u32 = 7;
// IC_CLR_RESTART_DET
pub const I2C0_IC_CLR_RESTART_DET_CLR_RESTART_DET_BIT:u32 = 0;
// IC_COMP_PARAM_1
pub const I2C0_IC_COMP_PARAM_1_TX_BUFFER_DEPTH_LOW: u32 = 16;
pub const I2C0_IC_COMP_PARAM_1_TX_BUFFER_DEPTH_HIGH:u32 = 23;
pub const I2C0_IC_COMP_PARAM_1_RX_BUFFER_DEPTH_LOW: u32 = 8;
pub const I2C0_IC_COMP_PARAM_1_RX_BUFFER_DEPTH_HIGH:u32 = 15;
pub const I2C0_IC_COMP_PARAM_1_ADD_ENCODED_PARAMS_BIT:u32 = 7;
pub const I2C0_IC_COMP_PARAM_1_HAS_DMA_BIT:         u32 = 6;
pub const I2C0_IC_COMP_PARAM_1_INTR_IO_BIT:         u32 = 5;
pub const I2C0_IC_COMP_PARAM_1_HC_COUNT_VALUES_BIT: u32 = 4;
pub const I2C0_IC_COMP_PARAM_1_MAX_SPEED_MODE_LOW:  u32 = 2;
pub const I2C0_IC_COMP_PARAM_1_MAX_SPEED_MODE_HIGH: u32 = 3;
pub const I2C0_IC_COMP_PARAM_1_APB_DATA_WIDTH_LOW:  u32 = 0;
pub const I2C0_IC_COMP_PARAM_1_APB_DATA_WIDTH_HIGH: u32 = 1;
// IC_COMP_VERSION
pub const I2C0_IC_COMP_VERSION_LOW:                 u32 = 0;
pub const I2C0_IC_COMP_VERSION_HIGH:                u32 = 31;
// IC_COMP_TYPE
pub const I2C0_IC_COMP_TYPE_LOW:                    u32 = 0;
pub const I2C0_IC_COMP_TYPE_HIGH:                   u32 = 31;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
