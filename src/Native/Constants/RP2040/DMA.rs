#![allow(dead_code)]
// DMA
pub const DMA_BASE:                                 u32 = 0x5000_0000;
pub fn DMA_CH_READ_ADDR(ch: u32) -> u32 {
    return DMA_BASE + 0x0 + ch * 0x40
}
pub fn DMA_CH_WRITE_ADDR(ch: u32) -> u32 { 
    return DMA_BASE + 0x4 + ch * 0x40 
}
pub fn DMA_CH_TRANS_COUNT(ch: u32) -> u32 { 
    return DMA_BASE + 0x8 + ch * 0x40
}
pub fn DMA_CH_CTRL_TRIG(ch: u32) -> u32 {
    return DMA_BASE + 0xc + ch * 0x40
}
pub fn DMA_CH_AL1_CTRL(ch: u32) -> u32 { 
    return DMA_BASE + 0x10 + ch * 0x40
}
pub fn DMA_CH_AL1_READ_ADDR(ch: u32) -> u32 { 
    return DMA_BASE + 0x14 + ch * 0x40
}
pub fn DMA_CH_AL1_WRITE_ADDR(ch: u32) -> u32 { 
    return DMA_BASE + 0x18 + ch * 0x40
}
pub fn DMA_CH_AL1_TRANS_COUNT_TRIG(ch: u32) -> u32 { 
    return DMA_BASE + 0x1c + ch * 0x40 
}
pub fn DMA_CH_AL2_CTRL(ch: u32) -> u32 {
    return DMA_BASE + 0x20 + ch * 0x40 
}
pub fn DMA_CH_AL2_TRANS_COUNT(ch: u32) -> u32 {
    return DMA_BASE + 0x24 + ch * 0x40 
}
pub fn DMA_CH_AL2_READ_ADDR(ch: u32) -> u32 { 
    return DMA_BASE + 0x28 + ch * 0x40 
}
pub fn DMA_CH_AL2_WRITE_ADDR_TRIG(ch: u32) -> u32 { 
    return DMA_BASE + 0x2c + ch * 0x40 
}
pub fn DMA_CH_AL3_CTRL(ch: u32) -> u32 { 
    return DMA_BASE + 0x30 + ch * 0x40 
}
pub fn DMA_CH_AL3_WRITE_ADDR(ch: u32) -> u32 { 
    return DMA_BASE + 0x34 + ch * 0x40 
}
pub fn DMA_CH_AL3_TRANS_COUNT(ch: u32) -> u32 { 
    return DMA_BASE + 0x38 + ch * 0x40 
}
pub fn DMA_CH_AL3_READ_ADDR_TRIG(ch: u32) -> u32 { 
    return DMA_BASE + 0x3c + ch * 0x40 
}
pub const DMA_INTR:                                 u32 = DMA_BASE + 0x400;
pub const DMA_INTE0:                                u32 = DMA_BASE + 0x404;
pub const DMA_INTF0:                                u32 = DMA_BASE + 0x408;
pub const DMA_INTS0:                                u32 = DMA_BASE + 0x40C;
pub const DMA_INTE1:                                u32 = DMA_BASE + 0x414;
pub const DMA_INTF1:                                u32 = DMA_BASE + 0x418;
pub const DMA_INTS1:                                u32 = DMA_BASE + 0x41C;
pub const DMA_TIMER0:                               u32 = DMA_BASE + 0x420;
pub const DMA_TIMER1:                               u32 = DMA_BASE + 0x424;
pub const DMA_TIMER2:                               u32 = DMA_BASE + 0x428;
pub const DMA_TIMER3:                               u32 = DMA_BASE + 0x42C;
pub const DMA_MULTI_CHAN_TRIGGER:                   u32 = DMA_BASE + 0x430;
pub const DMA_SNIFF_CTRL:                           u32 = DMA_BASE + 0x434;
pub const DMA_SNIFF_DATA:                           u32 = DMA_BASE + 0x438;
pub const DMA_FIFO_LEVELS:                          u32 = DMA_BASE + 0x440;
pub const DMA_CHAN_ABORT:                           u32 = DMA_BASE + 0x444;
pub const DMA_N_CHANNELS:                           u32 = DMA_BASE + 0x448;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CH0_READ_ADDR
pub const DMA_CH0_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH0_READ_ADDR_HIGH:                   u32 = 31;
// CH0_WRITE_ADDR
pub const DMA_CH0_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH0_WRITE_ADDR_HIGH:                  u32 = 31;
// CH0_TRANS_COUNT
pub const DMA_CH0_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH0_TRANS_COUNT_HIGH:                 u32 = 31;
// CH0_CTRL_TRIG
pub const DMA_CH0_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH0_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH0_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH0_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH0_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH0_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH0_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH0_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH0_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH0_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH0_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH0_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH0_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH0_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH0_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH0_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH0_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH0_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH0_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH0_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH0_AL1_CTRL
pub const DMA_CH0_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH0_AL1_CTRL_HIGH:                    u32 = 31;
// CH0_AL1_READ_ADDR
pub const DMA_CH0_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH0_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH0_AL1_WRITE_ADDR
pub const DMA_CH0_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH0_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH0_AL1_TRANS_COUNT_TRIG
pub const DMA_CH0_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH0_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH0_AL2_CTRL
pub const DMA_CH0_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH0_AL2_CTRL_HIGH:                    u32 = 31;
// CH0_AL2_TRANS_COUNT
pub const DMA_CH0_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH0_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH0_AL2_READ_ADDR
pub const DMA_CH0_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH0_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH0_AL2_WRITE_ADDR_TRIG
pub const DMA_CH0_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH0_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH0_AL3_CTRL
pub const DMA_CH0_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH0_AL3_CTRL_HIGH:                    u32 = 31;
// CH0_AL3_WRITE_ADDR
pub const DMA_CH0_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH0_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH0_AL3_TRANS_COUNT
pub const DMA_CH0_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH0_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH0_AL3_READ_ADDR_TRIG
pub const DMA_CH0_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH0_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH1_READ_ADDR
pub const DMA_CH1_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH1_READ_ADDR_HIGH:                   u32 = 31;
// CH1_WRITE_ADDR
pub const DMA_CH1_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH1_WRITE_ADDR_HIGH:                  u32 = 31;
// CH1_TRANS_COUNT
pub const DMA_CH1_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH1_TRANS_COUNT_HIGH:                 u32 = 31;
// CH1_CTRL_TRIG
pub const DMA_CH1_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH1_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH1_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH1_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH1_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH1_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH1_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH1_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH1_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH1_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH1_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH1_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH1_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH1_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH1_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH1_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH1_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH1_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH1_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH1_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH1_AL1_CTRL
pub const DMA_CH1_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH1_AL1_CTRL_HIGH:                    u32 = 31;
// CH1_AL1_READ_ADDR
pub const DMA_CH1_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH1_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH1_AL1_WRITE_ADDR
pub const DMA_CH1_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH1_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH1_AL1_TRANS_COUNT_TRIG
pub const DMA_CH1_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH1_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH1_AL2_CTRL
pub const DMA_CH1_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH1_AL2_CTRL_HIGH:                    u32 = 31;
// CH1_AL2_TRANS_COUNT
pub const DMA_CH1_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH1_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH1_AL2_READ_ADDR
pub const DMA_CH1_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH1_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH1_AL2_WRITE_ADDR_TRIG
pub const DMA_CH1_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH1_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH1_AL3_CTRL
pub const DMA_CH1_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH1_AL3_CTRL_HIGH:                    u32 = 31;
// CH1_AL3_WRITE_ADDR
pub const DMA_CH1_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH1_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH1_AL3_TRANS_COUNT
pub const DMA_CH1_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH1_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH1_AL3_READ_ADDR_TRIG
pub const DMA_CH1_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH1_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH2_READ_ADDR
pub const DMA_CH2_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH2_READ_ADDR_HIGH:                   u32 = 31;
// CH2_WRITE_ADDR
pub const DMA_CH2_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH2_WRITE_ADDR_HIGH:                  u32 = 31;
// CH2_TRANS_COUNT
pub const DMA_CH2_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH2_TRANS_COUNT_HIGH:                 u32 = 31;
// CH2_CTRL_TRIG
pub const DMA_CH2_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH2_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH2_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH2_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH2_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH2_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH2_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH2_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH2_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH2_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH2_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH2_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH2_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH2_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH2_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH2_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH2_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH2_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH2_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH2_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH2_AL1_CTRL
pub const DMA_CH2_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH2_AL1_CTRL_HIGH:                    u32 = 31;
// CH2_AL1_READ_ADDR
pub const DMA_CH2_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH2_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH2_AL1_WRITE_ADDR
pub const DMA_CH2_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH2_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH2_AL1_TRANS_COUNT_TRIG
pub const DMA_CH2_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH2_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH2_AL2_CTRL
pub const DMA_CH2_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH2_AL2_CTRL_HIGH:                    u32 = 31;
// CH2_AL2_TRANS_COUNT
pub const DMA_CH2_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH2_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH2_AL2_READ_ADDR
pub const DMA_CH2_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH2_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH2_AL2_WRITE_ADDR_TRIG
pub const DMA_CH2_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH2_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH2_AL3_CTRL
pub const DMA_CH2_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH2_AL3_CTRL_HIGH:                    u32 = 31;
// CH2_AL3_WRITE_ADDR
pub const DMA_CH2_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH2_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH2_AL3_TRANS_COUNT
pub const DMA_CH2_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH2_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH2_AL3_READ_ADDR_TRIG
pub const DMA_CH2_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH2_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH3_READ_ADDR
pub const DMA_CH3_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH3_READ_ADDR_HIGH:                   u32 = 31;
// CH3_WRITE_ADDR
pub const DMA_CH3_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH3_WRITE_ADDR_HIGH:                  u32 = 31;
// CH3_TRANS_COUNT
pub const DMA_CH3_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH3_TRANS_COUNT_HIGH:                 u32 = 31;
// CH3_CTRL_TRIG
pub const DMA_CH3_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH3_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH3_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH3_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH3_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH3_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH3_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH3_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH3_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH3_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH3_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH3_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH3_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH3_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH3_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH3_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH3_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH3_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH3_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH3_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH3_AL1_CTRL
pub const DMA_CH3_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH3_AL1_CTRL_HIGH:                    u32 = 31;
// CH3_AL1_READ_ADDR
pub const DMA_CH3_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH3_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH3_AL1_WRITE_ADDR
pub const DMA_CH3_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH3_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH3_AL1_TRANS_COUNT_TRIG
pub const DMA_CH3_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH3_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH3_AL2_CTRL
pub const DMA_CH3_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH3_AL2_CTRL_HIGH:                    u32 = 31;
// CH3_AL2_TRANS_COUNT
pub const DMA_CH3_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH3_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH3_AL2_READ_ADDR
pub const DMA_CH3_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH3_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH3_AL2_WRITE_ADDR_TRIG
pub const DMA_CH3_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH3_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH3_AL3_CTRL
pub const DMA_CH3_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH3_AL3_CTRL_HIGH:                    u32 = 31;
// CH3_AL3_WRITE_ADDR
pub const DMA_CH3_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH3_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH3_AL3_TRANS_COUNT
pub const DMA_CH3_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH3_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH3_AL3_READ_ADDR_TRIG
pub const DMA_CH3_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH3_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH4_READ_ADDR
pub const DMA_CH4_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH4_READ_ADDR_HIGH:                   u32 = 31;
// CH4_WRITE_ADDR
pub const DMA_CH4_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH4_WRITE_ADDR_HIGH:                  u32 = 31;
// CH4_TRANS_COUNT
pub const DMA_CH4_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH4_TRANS_COUNT_HIGH:                 u32 = 31;
// CH4_CTRL_TRIG
pub const DMA_CH4_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH4_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH4_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH4_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH4_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH4_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH4_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH4_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH4_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH4_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH4_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH4_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH4_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH4_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH4_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH4_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH4_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH4_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH4_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH4_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH4_AL1_CTRL
pub const DMA_CH4_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH4_AL1_CTRL_HIGH:                    u32 = 31;
// CH4_AL1_READ_ADDR
pub const DMA_CH4_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH4_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH4_AL1_WRITE_ADDR
pub const DMA_CH4_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH4_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH4_AL1_TRANS_COUNT_TRIG
pub const DMA_CH4_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH4_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH4_AL2_CTRL
pub const DMA_CH4_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH4_AL2_CTRL_HIGH:                    u32 = 31;
// CH4_AL2_TRANS_COUNT
pub const DMA_CH4_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH4_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH4_AL2_READ_ADDR
pub const DMA_CH4_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH4_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH4_AL2_WRITE_ADDR_TRIG
pub const DMA_CH4_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH4_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH4_AL3_CTRL
pub const DMA_CH4_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH4_AL3_CTRL_HIGH:                    u32 = 31;
// CH4_AL3_WRITE_ADDR
pub const DMA_CH4_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH4_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH4_AL3_TRANS_COUNT
pub const DMA_CH4_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH4_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH4_AL3_READ_ADDR_TRIG
pub const DMA_CH4_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH4_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH5_READ_ADDR
pub const DMA_CH5_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH5_READ_ADDR_HIGH:                   u32 = 31;
// CH5_WRITE_ADDR
pub const DMA_CH5_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH5_WRITE_ADDR_HIGH:                  u32 = 31;
// CH5_TRANS_COUNT
pub const DMA_CH5_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH5_TRANS_COUNT_HIGH:                 u32 = 31;
// CH5_CTRL_TRIG
pub const DMA_CH5_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH5_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH5_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH5_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH5_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH5_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH5_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH5_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH5_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH5_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH5_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH5_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH5_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH5_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH5_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH5_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH5_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH5_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH5_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH5_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH5_AL1_CTRL
pub const DMA_CH5_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH5_AL1_CTRL_HIGH:                    u32 = 31;
// CH5_AL1_READ_ADDR
pub const DMA_CH5_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH5_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH5_AL1_WRITE_ADDR
pub const DMA_CH5_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH5_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH5_AL1_TRANS_COUNT_TRIG
pub const DMA_CH5_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH5_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH5_AL2_CTRL
pub const DMA_CH5_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH5_AL2_CTRL_HIGH:                    u32 = 31;
// CH5_AL2_TRANS_COUNT
pub const DMA_CH5_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH5_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH5_AL2_READ_ADDR
pub const DMA_CH5_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH5_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH5_AL2_WRITE_ADDR_TRIG
pub const DMA_CH5_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH5_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH5_AL3_CTRL
pub const DMA_CH5_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH5_AL3_CTRL_HIGH:                    u32 = 31;
// CH5_AL3_WRITE_ADDR
pub const DMA_CH5_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH5_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH5_AL3_TRANS_COUNT
pub const DMA_CH5_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH5_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH5_AL3_READ_ADDR_TRIG
pub const DMA_CH5_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH5_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH6_READ_ADDR
pub const DMA_CH6_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH6_READ_ADDR_HIGH:                   u32 = 31;
// CH6_WRITE_ADDR
pub const DMA_CH6_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH6_WRITE_ADDR_HIGH:                  u32 = 31;
// CH6_TRANS_COUNT
pub const DMA_CH6_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH6_TRANS_COUNT_HIGH:                 u32 = 31;
// CH6_CTRL_TRIG
pub const DMA_CH6_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH6_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH6_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH6_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH6_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH6_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH6_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH6_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH6_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH6_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH6_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH6_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH6_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH6_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH6_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH6_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH6_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH6_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH6_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH6_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH6_AL1_CTRL
pub const DMA_CH6_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH6_AL1_CTRL_HIGH:                    u32 = 31;
// CH6_AL1_READ_ADDR
pub const DMA_CH6_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH6_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH6_AL1_WRITE_ADDR
pub const DMA_CH6_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH6_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH6_AL1_TRANS_COUNT_TRIG
pub const DMA_CH6_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH6_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH6_AL2_CTRL
pub const DMA_CH6_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH6_AL2_CTRL_HIGH:                    u32 = 31;
// CH6_AL2_TRANS_COUNT
pub const DMA_CH6_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH6_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH6_AL2_READ_ADDR
pub const DMA_CH6_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH6_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH6_AL2_WRITE_ADDR_TRIG
pub const DMA_CH6_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH6_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH6_AL3_CTRL
pub const DMA_CH6_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH6_AL3_CTRL_HIGH:                    u32 = 31;
// CH6_AL3_WRITE_ADDR
pub const DMA_CH6_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH6_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH6_AL3_TRANS_COUNT
pub const DMA_CH6_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH6_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH6_AL3_READ_ADDR_TRIG
pub const DMA_CH6_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH6_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH7_READ_ADDR
pub const DMA_CH7_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH7_READ_ADDR_HIGH:                   u32 = 31;
// CH7_WRITE_ADDR
pub const DMA_CH7_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH7_WRITE_ADDR_HIGH:                  u32 = 31;
// CH7_TRANS_COUNT
pub const DMA_CH7_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH7_TRANS_COUNT_HIGH:                 u32 = 31;
// CH7_CTRL_TRIG
pub const DMA_CH7_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH7_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH7_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH7_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH7_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH7_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH7_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH7_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH7_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH7_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH7_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH7_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH7_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH7_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH7_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH7_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH7_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH7_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH7_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH7_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH7_AL1_CTRL
pub const DMA_CH7_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH7_AL1_CTRL_HIGH:                    u32 = 31;
// CH7_AL1_READ_ADDR
pub const DMA_CH7_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH7_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH7_AL1_WRITE_ADDR
pub const DMA_CH7_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH7_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH7_AL1_TRANS_COUNT_TRIG
pub const DMA_CH7_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH7_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH7_AL2_CTRL
pub const DMA_CH7_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH7_AL2_CTRL_HIGH:                    u32 = 31;
// CH7_AL2_TRANS_COUNT
pub const DMA_CH7_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH7_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH7_AL2_READ_ADDR
pub const DMA_CH7_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH7_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH7_AL2_WRITE_ADDR_TRIG
pub const DMA_CH7_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH7_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH7_AL3_CTRL
pub const DMA_CH7_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH7_AL3_CTRL_HIGH:                    u32 = 31;
// CH7_AL3_WRITE_ADDR
pub const DMA_CH7_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH7_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH7_AL3_TRANS_COUNT
pub const DMA_CH7_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH7_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH7_AL3_READ_ADDR_TRIG
pub const DMA_CH7_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH7_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH8_READ_ADDR
pub const DMA_CH8_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH8_READ_ADDR_HIGH:                   u32 = 31;
// CH8_WRITE_ADDR
pub const DMA_CH8_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH8_WRITE_ADDR_HIGH:                  u32 = 31;
// CH8_TRANS_COUNT
pub const DMA_CH8_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH8_TRANS_COUNT_HIGH:                 u32 = 31;
// CH8_CTRL_TRIG
pub const DMA_CH8_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH8_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH8_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH8_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH8_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH8_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH8_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH8_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH8_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH8_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH8_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH8_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH8_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH8_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH8_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH8_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH8_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH8_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH8_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH8_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH8_AL1_CTRL
pub const DMA_CH8_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH8_AL1_CTRL_HIGH:                    u32 = 31;
// CH8_AL1_READ_ADDR
pub const DMA_CH8_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH8_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH8_AL1_WRITE_ADDR
pub const DMA_CH8_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH8_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH8_AL1_TRANS_COUNT_TRIG
pub const DMA_CH8_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH8_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH8_AL2_CTRL
pub const DMA_CH8_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH8_AL2_CTRL_HIGH:                    u32 = 31;
// CH8_AL2_TRANS_COUNT
pub const DMA_CH8_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH8_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH8_AL2_READ_ADDR
pub const DMA_CH8_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH8_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH8_AL2_WRITE_ADDR_TRIG
pub const DMA_CH8_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH8_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH8_AL3_CTRL
pub const DMA_CH8_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH8_AL3_CTRL_HIGH:                    u32 = 31;
// CH8_AL3_WRITE_ADDR
pub const DMA_CH8_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH8_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH8_AL3_TRANS_COUNT
pub const DMA_CH8_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH8_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH8_AL3_READ_ADDR_TRIG
pub const DMA_CH8_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH8_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH9_READ_ADDR
pub const DMA_CH9_READ_ADDR_LOW:                    u32 = 0;
pub const DMA_CH9_READ_ADDR_HIGH:                   u32 = 31;
// CH9_WRITE_ADDR
pub const DMA_CH9_WRITE_ADDR_LOW:                   u32 = 0;
pub const DMA_CH9_WRITE_ADDR_HIGH:                  u32 = 31;
// CH9_TRANS_COUNT
pub const DMA_CH9_TRANS_COUNT_LOW:                  u32 = 0;
pub const DMA_CH9_TRANS_COUNT_HIGH:                 u32 = 31;
// CH9_CTRL_TRIG
pub const DMA_CH9_CTRL_TRIG_AHB_ERROR_BIT:          u32 = 31;
pub const DMA_CH9_CTRL_TRIG_READ_ERROR_BIT:         u32 = 30;
pub const DMA_CH9_CTRL_TRIG_WRITE_ERROR_BIT:        u32 = 29;
pub const DMA_CH9_CTRL_TRIG_BUSY_BIT:               u32 = 24;
pub const DMA_CH9_CTRL_TRIG_SNIFF_EN_BIT:           u32 = 23;
pub const DMA_CH9_CTRL_TRIG_BSWAP_BIT:              u32 = 22;
pub const DMA_CH9_CTRL_TRIG_IRQ_QUIET_BIT:          u32 = 21;
pub const DMA_CH9_CTRL_TRIG_TREQ_SEL_LOW:           u32 = 15;
pub const DMA_CH9_CTRL_TRIG_TREQ_SEL_HIGH:          u32 = 20;
pub const DMA_CH9_CTRL_TRIG_CHAIN_TO_LOW:           u32 = 11;
pub const DMA_CH9_CTRL_TRIG_CHAIN_TO_HIGH:          u32 = 14;
pub const DMA_CH9_CTRL_TRIG_RING_SEL_BIT:           u32 = 10;
pub const DMA_CH9_CTRL_TRIG_RING_SIZE_LOW:          u32 = 6;
pub const DMA_CH9_CTRL_TRIG_RING_SIZE_HIGH:         u32 = 9;
pub const DMA_CH9_CTRL_TRIG_INCR_WRITE_BIT:         u32 = 5;
pub const DMA_CH9_CTRL_TRIG_INCR_READ_BIT:          u32 = 4;
pub const DMA_CH9_CTRL_TRIG_DATA_SIZE_LOW:          u32 = 2;
pub const DMA_CH9_CTRL_TRIG_DATA_SIZE_HIGH:         u32 = 3;
pub const DMA_CH9_CTRL_TRIG_HIGH_PRIORITY_BIT:      u32 = 1;
pub const DMA_CH9_CTRL_TRIG_EN_BIT:                 u32 = 0;
// CH9_AL1_CTRL
pub const DMA_CH9_AL1_CTRL_LOW:                     u32 = 0;
pub const DMA_CH9_AL1_CTRL_HIGH:                    u32 = 31;
// CH9_AL1_READ_ADDR
pub const DMA_CH9_AL1_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH9_AL1_READ_ADDR_HIGH:               u32 = 31;
// CH9_AL1_WRITE_ADDR
pub const DMA_CH9_AL1_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH9_AL1_WRITE_ADDR_HIGH:              u32 = 31;
// CH9_AL1_TRANS_COUNT_TRIG
pub const DMA_CH9_AL1_TRANS_COUNT_TRIG_LOW:         u32 = 0;
pub const DMA_CH9_AL1_TRANS_COUNT_TRIG_HIGH:        u32 = 31;
// CH9_AL2_CTRL
pub const DMA_CH9_AL2_CTRL_LOW:                     u32 = 0;
pub const DMA_CH9_AL2_CTRL_HIGH:                    u32 = 31;
// CH9_AL2_TRANS_COUNT
pub const DMA_CH9_AL2_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH9_AL2_TRANS_COUNT_HIGH:             u32 = 31;
// CH9_AL2_READ_ADDR
pub const DMA_CH9_AL2_READ_ADDR_LOW:                u32 = 0;
pub const DMA_CH9_AL2_READ_ADDR_HIGH:               u32 = 31;
// CH9_AL2_WRITE_ADDR_TRIG
pub const DMA_CH9_AL2_WRITE_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH9_AL2_WRITE_ADDR_TRIG_HIGH:         u32 = 31;
// CH9_AL3_CTRL
pub const DMA_CH9_AL3_CTRL_LOW:                     u32 = 0;
pub const DMA_CH9_AL3_CTRL_HIGH:                    u32 = 31;
// CH9_AL3_WRITE_ADDR
pub const DMA_CH9_AL3_WRITE_ADDR_LOW:               u32 = 0;
pub const DMA_CH9_AL3_WRITE_ADDR_HIGH:              u32 = 31;
// CH9_AL3_TRANS_COUNT
pub const DMA_CH9_AL3_TRANS_COUNT_LOW:              u32 = 0;
pub const DMA_CH9_AL3_TRANS_COUNT_HIGH:             u32 = 31;
// CH9_AL3_READ_ADDR_TRIG
pub const DMA_CH9_AL3_READ_ADDR_TRIG_LOW:           u32 = 0;
pub const DMA_CH9_AL3_READ_ADDR_TRIG_HIGH:          u32 = 31;
// CH10_READ_ADDR
pub const DMA_CH10_READ_ADDR_LOW:                   u32 = 0;
pub const DMA_CH10_READ_ADDR_HIGH:                  u32 = 31;
// CH10_WRITE_ADDR
pub const DMA_CH10_WRITE_ADDR_LOW:                  u32 = 0;
pub const DMA_CH10_WRITE_ADDR_HIGH:                 u32 = 31;
// CH10_TRANS_COUNT
pub const DMA_CH10_TRANS_COUNT_LOW:                 u32 = 0;
pub const DMA_CH10_TRANS_COUNT_HIGH:                u32 = 31;
// CH10_CTRL_TRIG
pub const DMA_CH10_CTRL_TRIG_AHB_ERROR_BIT:         u32 = 31;
pub const DMA_CH10_CTRL_TRIG_READ_ERROR_BIT:        u32 = 30;
pub const DMA_CH10_CTRL_TRIG_WRITE_ERROR_BIT:       u32 = 29;
pub const DMA_CH10_CTRL_TRIG_BUSY_BIT:              u32 = 24;
pub const DMA_CH10_CTRL_TRIG_SNIFF_EN_BIT:          u32 = 23;
pub const DMA_CH10_CTRL_TRIG_BSWAP_BIT:             u32 = 22;
pub const DMA_CH10_CTRL_TRIG_IRQ_QUIET_BIT:         u32 = 21;
pub const DMA_CH10_CTRL_TRIG_TREQ_SEL_LOW:          u32 = 15;
pub const DMA_CH10_CTRL_TRIG_TREQ_SEL_HIGH:         u32 = 20;
pub const DMA_CH10_CTRL_TRIG_CHAIN_TO_LOW:          u32 = 11;
pub const DMA_CH10_CTRL_TRIG_CHAIN_TO_HIGH:         u32 = 14;
pub const DMA_CH10_CTRL_TRIG_RING_SEL_BIT:          u32 = 10;
pub const DMA_CH10_CTRL_TRIG_RING_SIZE_LOW:         u32 = 6;
pub const DMA_CH10_CTRL_TRIG_RING_SIZE_HIGH:        u32 = 9;
pub const DMA_CH10_CTRL_TRIG_INCR_WRITE_BIT:        u32 = 5;
pub const DMA_CH10_CTRL_TRIG_INCR_READ_BIT:         u32 = 4;
pub const DMA_CH10_CTRL_TRIG_DATA_SIZE_LOW:         u32 = 2;
pub const DMA_CH10_CTRL_TRIG_DATA_SIZE_HIGH:        u32 = 3;
pub const DMA_CH10_CTRL_TRIG_HIGH_PRIORITY_BIT:     u32 = 1;
pub const DMA_CH10_CTRL_TRIG_EN_BIT:                u32 = 0;
// CH10_AL1_CTRL
pub const DMA_CH10_AL1_CTRL_LOW:                    u32 = 0;
pub const DMA_CH10_AL1_CTRL_HIGH:                   u32 = 31;
// CH10_AL1_READ_ADDR
pub const DMA_CH10_AL1_READ_ADDR_LOW:               u32 = 0;
pub const DMA_CH10_AL1_READ_ADDR_HIGH:              u32 = 31;
// CH10_AL1_WRITE_ADDR
pub const DMA_CH10_AL1_WRITE_ADDR_LOW:              u32 = 0;
pub const DMA_CH10_AL1_WRITE_ADDR_HIGH:             u32 = 31;
// CH10_AL1_TRANS_COUNT_TRIG
pub const DMA_CH10_AL1_TRANS_COUNT_TRIG_LOW:        u32 = 0;
pub const DMA_CH10_AL1_TRANS_COUNT_TRIG_HIGH:       u32 = 31;
// CH10_AL2_CTRL
pub const DMA_CH10_AL2_CTRL_LOW:                    u32 = 0;
pub const DMA_CH10_AL2_CTRL_HIGH:                   u32 = 31;
// CH10_AL2_TRANS_COUNT
pub const DMA_CH10_AL2_TRANS_COUNT_LOW:             u32 = 0;
pub const DMA_CH10_AL2_TRANS_COUNT_HIGH:            u32 = 31;
// CH10_AL2_READ_ADDR
pub const DMA_CH10_AL2_READ_ADDR_LOW:               u32 = 0;
pub const DMA_CH10_AL2_READ_ADDR_HIGH:              u32 = 31;
// CH10_AL2_WRITE_ADDR_TRIG
pub const DMA_CH10_AL2_WRITE_ADDR_TRIG_LOW:         u32 = 0;
pub const DMA_CH10_AL2_WRITE_ADDR_TRIG_HIGH:        u32 = 31;
// CH10_AL3_CTRL
pub const DMA_CH10_AL3_CTRL_LOW:                    u32 = 0;
pub const DMA_CH10_AL3_CTRL_HIGH:                   u32 = 31;
// CH10_AL3_WRITE_ADDR
pub const DMA_CH10_AL3_WRITE_ADDR_LOW:              u32 = 0;
pub const DMA_CH10_AL3_WRITE_ADDR_HIGH:             u32 = 31;
// CH10_AL3_TRANS_COUNT
pub const DMA_CH10_AL3_TRANS_COUNT_LOW:             u32 = 0;
pub const DMA_CH10_AL3_TRANS_COUNT_HIGH:            u32 = 31;
// CH10_AL3_READ_ADDR_TRIG
pub const DMA_CH10_AL3_READ_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH10_AL3_READ_ADDR_TRIG_HIGH:         u32 = 31;
// CH11_READ_ADDR
pub const DMA_CH11_READ_ADDR_LOW:                   u32 = 0;
pub const DMA_CH11_READ_ADDR_HIGH:                  u32 = 31;
// CH11_WRITE_ADDR
pub const DMA_CH11_WRITE_ADDR_LOW:                  u32 = 0;
pub const DMA_CH11_WRITE_ADDR_HIGH:                 u32 = 31;
// CH11_TRANS_COUNT
pub const DMA_CH11_TRANS_COUNT_LOW:                 u32 = 0;
pub const DMA_CH11_TRANS_COUNT_HIGH:                u32 = 31;
// CH11_CTRL_TRIG
pub const DMA_CH11_CTRL_TRIG_AHB_ERROR_BIT:         u32 = 31;
pub const DMA_CH11_CTRL_TRIG_READ_ERROR_BIT:        u32 = 30;
pub const DMA_CH11_CTRL_TRIG_WRITE_ERROR_BIT:       u32 = 29;
pub const DMA_CH11_CTRL_TRIG_BUSY_BIT:              u32 = 24;
pub const DMA_CH11_CTRL_TRIG_SNIFF_EN_BIT:          u32 = 23;
pub const DMA_CH11_CTRL_TRIG_BSWAP_BIT:             u32 = 22;
pub const DMA_CH11_CTRL_TRIG_IRQ_QUIET_BIT:         u32 = 21;
pub const DMA_CH11_CTRL_TRIG_TREQ_SEL_LOW:          u32 = 15;
pub const DMA_CH11_CTRL_TRIG_TREQ_SEL_HIGH:         u32 = 20;
pub const DMA_CH11_CTRL_TRIG_CHAIN_TO_LOW:          u32 = 11;
pub const DMA_CH11_CTRL_TRIG_CHAIN_TO_HIGH:         u32 = 14;
pub const DMA_CH11_CTRL_TRIG_RING_SEL_BIT:          u32 = 10;
pub const DMA_CH11_CTRL_TRIG_RING_SIZE_LOW:         u32 = 6;
pub const DMA_CH11_CTRL_TRIG_RING_SIZE_HIGH:        u32 = 9;
pub const DMA_CH11_CTRL_TRIG_INCR_WRITE_BIT:        u32 = 5;
pub const DMA_CH11_CTRL_TRIG_INCR_READ_BIT:         u32 = 4;
pub const DMA_CH11_CTRL_TRIG_DATA_SIZE_LOW:         u32 = 2;
pub const DMA_CH11_CTRL_TRIG_DATA_SIZE_HIGH:        u32 = 3;
pub const DMA_CH11_CTRL_TRIG_HIGH_PRIORITY_BIT:     u32 = 1;
pub const DMA_CH11_CTRL_TRIG_EN_BIT:                u32 = 0;
// CH11_AL1_CTRL
pub const DMA_CH11_AL1_CTRL_LOW:                    u32 = 0;
pub const DMA_CH11_AL1_CTRL_HIGH:                   u32 = 31;
// CH11_AL1_READ_ADDR
pub const DMA_CH11_AL1_READ_ADDR_LOW:               u32 = 0;
pub const DMA_CH11_AL1_READ_ADDR_HIGH:              u32 = 31;
// CH11_AL1_WRITE_ADDR
pub const DMA_CH11_AL1_WRITE_ADDR_LOW:              u32 = 0;
pub const DMA_CH11_AL1_WRITE_ADDR_HIGH:             u32 = 31;
// CH11_AL1_TRANS_COUNT_TRIG
pub const DMA_CH11_AL1_TRANS_COUNT_TRIG_LOW:        u32 = 0;
pub const DMA_CH11_AL1_TRANS_COUNT_TRIG_HIGH:       u32 = 31;
// CH11_AL2_CTRL
pub const DMA_CH11_AL2_CTRL_LOW:                    u32 = 0;
pub const DMA_CH11_AL2_CTRL_HIGH:                   u32 = 31;
// CH11_AL2_TRANS_COUNT
pub const DMA_CH11_AL2_TRANS_COUNT_LOW:             u32 = 0;
pub const DMA_CH11_AL2_TRANS_COUNT_HIGH:            u32 = 31;
// CH11_AL2_READ_ADDR
pub const DMA_CH11_AL2_READ_ADDR_LOW:               u32 = 0;
pub const DMA_CH11_AL2_READ_ADDR_HIGH:              u32 = 31;
// CH11_AL2_WRITE_ADDR_TRIG
pub const DMA_CH11_AL2_WRITE_ADDR_TRIG_LOW:         u32 = 0;
pub const DMA_CH11_AL2_WRITE_ADDR_TRIG_HIGH:        u32 = 31;
// CH11_AL3_CTRL
pub const DMA_CH11_AL3_CTRL_LOW:                    u32 = 0;
pub const DMA_CH11_AL3_CTRL_HIGH:                   u32 = 31;
// CH11_AL3_WRITE_ADDR
pub const DMA_CH11_AL3_WRITE_ADDR_LOW:              u32 = 0;
pub const DMA_CH11_AL3_WRITE_ADDR_HIGH:             u32 = 31;
// CH11_AL3_TRANS_COUNT
pub const DMA_CH11_AL3_TRANS_COUNT_LOW:             u32 = 0;
pub const DMA_CH11_AL3_TRANS_COUNT_HIGH:            u32 = 31;
// CH11_AL3_READ_ADDR_TRIG
pub const DMA_CH11_AL3_READ_ADDR_TRIG_LOW:          u32 = 0;
pub const DMA_CH11_AL3_READ_ADDR_TRIG_HIGH:         u32 = 31;
// INTR
pub const DMA_INTR_LOW:                             u32 = 0;
pub const DMA_INTR_HIGH:                            u32 = 15;
// INTE0
pub const DMA_INTE0_LOW:                            u32 = 0;
pub const DMA_INTE0_HIGH:                           u32 = 15;
// INTF0
pub const DMA_INTF0_LOW:                            u32 = 0;
pub const DMA_INTF0_HIGH:                           u32 = 15;
// INTS0
pub const DMA_INTS0_LOW:                            u32 = 0;
pub const DMA_INTS0_HIGH:                           u32 = 15;
// INTE1
pub const DMA_INTE1_LOW:                            u32 = 0;
pub const DMA_INTE1_HIGH:                           u32 = 15;
// INTF1
pub const DMA_INTF1_LOW:                            u32 = 0;
pub const DMA_INTF1_HIGH:                           u32 = 15;
// INTS1
pub const DMA_INTS1_LOW:                            u32 = 0;
pub const DMA_INTS1_HIGH:                           u32 = 15;
// TIMER0
pub const DMA_TIMER0_X_LOW:                         u32 = 16;
pub const DMA_TIMER0_X_HIGH:                        u32 = 31;
pub const DMA_TIMER0_Y_LOW:                         u32 = 0;
pub const DMA_TIMER0_Y_HIGH:                        u32 = 15;
// TIMER1
pub const DMA_TIMER1_X_LOW:                         u32 = 16;
pub const DMA_TIMER1_X_HIGH:                        u32 = 31;
pub const DMA_TIMER1_Y_LOW:                         u32 = 0;
pub const DMA_TIMER1_Y_HIGH:                        u32 = 15;
// TIMER2
pub const DMA_TIMER2_X_LOW:                         u32 = 16;
pub const DMA_TIMER2_X_HIGH:                        u32 = 31;
pub const DMA_TIMER2_Y_LOW:                         u32 = 0;
pub const DMA_TIMER2_Y_HIGH:                        u32 = 15;
// TIMER3
pub const DMA_TIMER3_X_LOW:                         u32 = 16;
pub const DMA_TIMER3_X_HIGH:                        u32 = 31;
pub const DMA_TIMER3_Y_LOW:                         u32 = 0;
pub const DMA_TIMER3_Y_HIGH:                        u32 = 15;
// MULTI_CHAN_TRIGGER
pub const DMA_MULTI_CHAN_TRIGGER_LOW:               u32 = 0;
pub const DMA_MULTI_CHAN_TRIGGER_HIGH:              u32 = 15;
// SNIFF_CTRL
pub const DMA_SNIFF_CTRL_OUT_INV_BIT:               u32 = 11;
pub const DMA_SNIFF_CTRL_OUT_REV_BIT:               u32 = 10;
pub const DMA_SNIFF_CTRL_BSWAP_BIT:                 u32 = 9;
pub const DMA_SNIFF_CTRL_CALC_LOW:                  u32 = 5;
pub const DMA_SNIFF_CTRL_CALC_HIGH:                 u32 = 8;
pub const DMA_SNIFF_CTRL_DMACH_LOW:                 u32 = 1;
pub const DMA_SNIFF_CTRL_DMACH_HIGH:                u32 = 4;
pub const DMA_SNIFF_CTRL_EN_BIT:                    u32 = 0;
// SNIFF_DATA
pub const DMA_SNIFF_DATA_LOW:                       u32 = 0;
pub const DMA_SNIFF_DATA_HIGH:                      u32 = 31;
// FIFO_LEVELS
pub const DMA_FIFO_LEVELS_RAF_LVL_LOW:              u32 = 16;
pub const DMA_FIFO_LEVELS_RAF_LVL_HIGH:             u32 = 23;
pub const DMA_FIFO_LEVELS_WAF_LVL_LOW:              u32 = 8;
pub const DMA_FIFO_LEVELS_WAF_LVL_HIGH:             u32 = 15;
pub const DMA_FIFO_LEVELS_TDF_LVL_LOW:              u32 = 0;
pub const DMA_FIFO_LEVELS_TDF_LVL_HIGH:             u32 = 7;
// CHAN_ABORT
pub const DMA_CHAN_ABORT_LOW:                       u32 = 0;
pub const DMA_CHAN_ABORT_HIGH:                      u32 = 15;
// N_CHANNELS
pub const DMA_N_CHANNELS_LOW:                       u32 = 0;
pub const DMA_N_CHANNELS_HIGH:                      u32 = 4;
// CH0_DBG_CTDREQ
pub const DMA_CH0_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH0_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH0_DBG_TCR
pub const DMA_CH0_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH0_DBG_TCR_HIGH:                     u32 = 31;
// CH1_DBG_CTDREQ
pub const DMA_CH1_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH1_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH1_DBG_TCR
pub const DMA_CH1_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH1_DBG_TCR_HIGH:                     u32 = 31;
// CH2_DBG_CTDREQ
pub const DMA_CH2_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH2_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH2_DBG_TCR
pub const DMA_CH2_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH2_DBG_TCR_HIGH:                     u32 = 31;
// CH3_DBG_CTDREQ
pub const DMA_CH3_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH3_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH3_DBG_TCR
pub const DMA_CH3_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH3_DBG_TCR_HIGH:                     u32 = 31;
// CH4_DBG_CTDREQ
pub const DMA_CH4_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH4_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH4_DBG_TCR
pub const DMA_CH4_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH4_DBG_TCR_HIGH:                     u32 = 31;
// CH5_DBG_CTDREQ
pub const DMA_CH5_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH5_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH5_DBG_TCR
pub const DMA_CH5_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH5_DBG_TCR_HIGH:                     u32 = 31;
// CH6_DBG_CTDREQ
pub const DMA_CH6_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH6_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH6_DBG_TCR
pub const DMA_CH6_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH6_DBG_TCR_HIGH:                     u32 = 31;
// CH7_DBG_CTDREQ
pub const DMA_CH7_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH7_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH7_DBG_TCR
pub const DMA_CH7_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH7_DBG_TCR_HIGH:                     u32 = 31;
// CH8_DBG_CTDREQ
pub const DMA_CH8_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH8_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH8_DBG_TCR
pub const DMA_CH8_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH8_DBG_TCR_HIGH:                     u32 = 31;
// CH9_DBG_CTDREQ
pub const DMA_CH9_DBG_CTDREQ_LOW:                   u32 = 0;
pub const DMA_CH9_DBG_CTDREQ_HIGH:                  u32 = 5;
// CH9_DBG_TCR
pub const DMA_CH9_DBG_TCR_LOW:                      u32 = 0;
pub const DMA_CH9_DBG_TCR_HIGH:                     u32 = 31;
// CH10_DBG_CTDREQ
pub const DMA_CH10_DBG_CTDREQ_LOW:                  u32 = 0;
pub const DMA_CH10_DBG_CTDREQ_HIGH:                 u32 = 5;
// CH10_DBG_TCR
pub const DMA_CH10_DBG_TCR_LOW:                     u32 = 0;
pub const DMA_CH10_DBG_TCR_HIGH:                    u32 = 31;
// CH11_DBG_CTDREQ
pub const DMA_CH11_DBG_CTDREQ_LOW:                  u32 = 0;
pub const DMA_CH11_DBG_CTDREQ_HIGH:                 u32 = 5;
// CH11_DBG_TCR
pub const DMA_CH11_DBG_TCR_LOW:                     u32 = 0;
pub const DMA_CH11_DBG_TCR_HIGH:                    u32 = 31;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CH0_CTRL_TRIG..CH11_CTRL_TRIG: TREQ_SEL
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO0_TX0:       u32 = 0x0;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO0_TX1:       u32 = 0x1;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO0_TX2:       u32 = 0x2;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO0_TX3:       u32 = 0x3;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO0_RX0:       u32 = 0x4;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO0_RX1:       u32 = 0x5;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO0_RX2:       u32 = 0x6;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO0_RX3:       u32 = 0x7;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO1_TX0:       u32 = 0x8;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO1_TX1:       u32 = 0x9;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO1_TX2:       u32 = 0xA;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO1_TX3:       u32 = 0xB;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO1_RX0:       u32 = 0xC;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO1_RX1:       u32 = 0xD;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO1_RX2:       u32 = 0xE;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PIO1_RX3:       u32 = 0xF;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_SPI0_TX:        u32 = 0x10;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_SPI0_RX:        u32 = 0x11;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_SPI1_TX:        u32 = 0x12;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_SPI1_RX:        u32 = 0x13;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_UART0_TX:       u32 = 0x14;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_UART0_RX:       u32 = 0x15;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_UART1_TX:       u32 = 0x16;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_UART1_RX:       u32 = 0x17;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PWM_WRAP0:      u32 = 0x18;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PWM_WRAP1:      u32 = 0x19;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PWM_WRAP2:      u32 = 0x1A;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PWM_WRAP3:      u32 = 0x1B;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PWM_WRAP4:      u32 = 0x1C;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PWM_WRAP5:      u32 = 0x1D;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PWM_WRAP6:      u32 = 0x1E;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PWM_WRAP7:      u32 = 0x1F;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_I2C0_TX:        u32 = 0x20;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_I2C0_RX:        u32 = 0x21;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_I2C1_TX:        u32 = 0x22;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_I2C1_RX:        u32 = 0x23;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_ADC:            u32 = 0x24;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_XIP_STREAM:     u32 = 0x25;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_XIP_SSITX:      u32 = 0x26;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_XIP_SSIRX:      u32 = 0x27;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_TIMER0:         u32 = 0x3B;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_TIMER1:         u32 = 0x3C;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_TIMER2:         u32 = 0x3D;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_TIMER3:         u32 = 0x3E;
pub const DMA_CH_CTRL_TRIG_TREQ_SEL_PERMANENT:      u32 = 0x3F;

// CH0_CTRL_TRIG..CH11_CTRL_TRIG: RING_SIZE
pub const DMA_CH_CTRL_TRIG_RING_SIZE_RING_NONE:     u32 = 0x0;

// CH0_CTRL_TRIG..CH11_CTRL_TRIG: DATA_SIZE
pub const DMA_CH_CTRL_TRIG_DATA_SIZE_SIZE_BYTE:     u32 = 0x0;
pub const DMA_CH_CTRL_TRIG_DATA_SIZE_SIZE_HALFWORD: u32 = 0x1;
pub const DMA_CH_CTRL_TRIG_DATA_SIZE_SIZE_WORD:     u32 = 0x2;

// SNIFF_CTRL: CALC
pub const DMA_SNIFF_CTRL_CALC_CRC32:                u32 = 0x0;
pub const DMA_SNIFF_CTRL_CALC_CRC32R:               u32 = 0x1;
pub const DMA_SNIFF_CTRL_CALC_CRC16:                u32 = 0x2;
pub const DMA_SNIFF_CTRL_CALC_CRC16R:               u32 = 0x3;
pub const DMA_SNIFF_CTRL_CALC_EVEN:                 u32 = 0xE;
pub const DMA_SNIFF_CTRL_CALC_SUM:                  u32 = 0xF;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
