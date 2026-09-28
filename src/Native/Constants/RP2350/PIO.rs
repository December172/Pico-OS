#![allow(dead_code)]
// PIO

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const PIO0_BASE:                                u32 = 0x5020_0000;
pub const PIO1_BASE:                                u32 = 0x5030_0000;
pub const PIO2_BASE:                                u32 = 0x5040_0000;

pub const PIO0_CTRL:                                u32 = PIO0_BASE + 0x0;
pub const PIO0_FSTAT:                               u32 = PIO0_BASE + 0x4;
pub const PIO0_FDEBUG:                              u32 = PIO0_BASE + 0x8;
pub const PIO0_FLEVEL:                              u32 = PIO0_BASE + 0xC;
// TXF0..TXF3
pub fn PIO0_TXF(n: u32) -> u32 {
    return PIO0_BASE + 0x10 + n * 0x4
}

// RXF0..RXF3
pub fn PIO0_RXF(n: u32) -> u32 {
    return PIO0_BASE + 0x20 + n * 0x4
}

pub const PIO0_IRQ:                                 u32 = PIO0_BASE + 0x30;
pub const PIO0_IRQ_FORCE:                           u32 = PIO0_BASE + 0x34;
pub const PIO0_INPUT_SYNC_BYPASS:                   u32 = PIO0_BASE + 0x38;
pub const PIO0_DBG_PADOUT:                          u32 = PIO0_BASE + 0x3C;
pub const PIO0_DBG_PADOE:                           u32 = PIO0_BASE + 0x40;
pub const PIO0_DBG_CFGINFO:                         u32 = PIO0_BASE + 0x44;
// INSTR_MEM0..INSTR_MEM31
pub fn PIO0_INSTR_MEM(n: u32) -> u32 {
    return PIO0_BASE + 0x48 + n * 0x4
}

// SM0_CLKDIV..SM3_CLKDIV
pub fn PIO0_SM_CLKDIV(n: u32) -> u32 {
    return PIO0_BASE + 0xC8 + n * 0x18
}

// SM0_EXECCTRL..SM3_EXECCTRL
pub fn PIO0_SM_EXECCTRL(n: u32) -> u32 {
    return PIO0_BASE + 0xCC + n * 0x18
}

// SM0_SHIFTCTRL..SM3_SHIFTCTRL
pub fn PIO0_SM_SHIFTCTRL(n: u32) -> u32 {
    return PIO0_BASE + 0xD0 + n * 0x18
}

// SM0_ADDR..SM3_ADDR
pub fn PIO0_SM_ADDR(n: u32) -> u32 {
    return PIO0_BASE + 0xD4 + n * 0x18
}

// SM0_INSTR..SM3_INSTR
pub fn PIO0_SM_INSTR(n: u32) -> u32 {
    return PIO0_BASE + 0xD8 + n * 0x18
}

// SM0_PINCTRL..SM3_PINCTRL
pub fn PIO0_SM_PINCTRL(n: u32) -> u32 {
    return PIO0_BASE + 0xDC + n * 0x18
}

// RXF0_PUTGET0..RXF0_PUTGET3
pub fn PIO0_RXF0_PUTGET(n: u32) -> u32 {
    return PIO0_BASE + 0x128 + n * 0x4
}

// RXF1_PUTGET0..RXF1_PUTGET3
pub fn PIO0_RXF1_PUTGET(n: u32) -> u32 {
    return PIO0_BASE + 0x138 + n * 0x4
}

// RXF2_PUTGET0..RXF2_PUTGET3
pub fn PIO0_RXF2_PUTGET(n: u32) -> u32 {
    return PIO0_BASE + 0x148 + n * 0x4
}

// RXF3_PUTGET0..RXF3_PUTGET3
pub fn PIO0_RXF3_PUTGET(n: u32) -> u32 {
    return PIO0_BASE + 0x158 + n * 0x4
}

pub const PIO0_GPIOBASE:                            u32 = PIO0_BASE + 0x168;
pub const PIO0_INTR:                                u32 = PIO0_BASE + 0x16C;
// IRQ0_INTE..IRQ1_INTE
pub fn PIO0_IRQ_INTE(n: u32) -> u32 {
    return PIO0_BASE + 0x170 + n * 0xC
}

// IRQ0_INTF..IRQ1_INTF
pub fn PIO0_IRQ_INTF(n: u32) -> u32 {
    return PIO0_BASE + 0x174 + n * 0xC
}

// IRQ0_INTS..IRQ1_INTS
pub fn PIO0_IRQ_INTS(n: u32) -> u32 {
    return PIO0_BASE + 0x178 + n * 0xC
}

pub const PIO1_CTRL:                                u32 = PIO1_BASE + 0x0;
pub const PIO1_FSTAT:                               u32 = PIO1_BASE + 0x4;
pub const PIO1_FDEBUG:                              u32 = PIO1_BASE + 0x8;
pub const PIO1_FLEVEL:                              u32 = PIO1_BASE + 0xC;
// TXF0..TXF3
pub fn PIO1_TXF(n: u32) -> u32 {
    return PIO1_BASE + 0x10 + n * 0x4
}

// RXF0..RXF3
pub fn PIO1_RXF(n: u32) -> u32 {
    return PIO1_BASE + 0x20 + n * 0x4
}

pub const PIO1_IRQ:                                 u32 = PIO1_BASE + 0x30;
pub const PIO1_IRQ_FORCE:                           u32 = PIO1_BASE + 0x34;
pub const PIO1_INPUT_SYNC_BYPASS:                   u32 = PIO1_BASE + 0x38;
pub const PIO1_DBG_PADOUT:                          u32 = PIO1_BASE + 0x3C;
pub const PIO1_DBG_PADOE:                           u32 = PIO1_BASE + 0x40;
pub const PIO1_DBG_CFGINFO:                         u32 = PIO1_BASE + 0x44;
// INSTR_MEM0..INSTR_MEM31
pub fn PIO1_INSTR_MEM(n: u32) -> u32 {
    return PIO1_BASE + 0x48 + n * 0x4
}

// SM0_CLKDIV..SM3_CLKDIV
pub fn PIO1_SM_CLKDIV(n: u32) -> u32 {
    return PIO1_BASE + 0xC8 + n * 0x18
}

// SM0_EXECCTRL..SM3_EXECCTRL
pub fn PIO1_SM_EXECCTRL(n: u32) -> u32 {
    return PIO1_BASE + 0xCC + n * 0x18
}

// SM0_SHIFTCTRL..SM3_SHIFTCTRL
pub fn PIO1_SM_SHIFTCTRL(n: u32) -> u32 {
    return PIO1_BASE + 0xD0 + n * 0x18
}

// SM0_ADDR..SM3_ADDR
pub fn PIO1_SM_ADDR(n: u32) -> u32 {
    return PIO1_BASE + 0xD4 + n * 0x18
}

// SM0_INSTR..SM3_INSTR
pub fn PIO1_SM_INSTR(n: u32) -> u32 {
    return PIO1_BASE + 0xD8 + n * 0x18
}

// SM0_PINCTRL..SM3_PINCTRL
pub fn PIO1_SM_PINCTRL(n: u32) -> u32 {
    return PIO1_BASE + 0xDC + n * 0x18
}

// RXF0_PUTGET0..RXF0_PUTGET3
pub fn PIO1_RXF0_PUTGET(n: u32) -> u32 {
    return PIO1_BASE + 0x128 + n * 0x4
}

// RXF1_PUTGET0..RXF1_PUTGET3
pub fn PIO1_RXF1_PUTGET(n: u32) -> u32 {
    return PIO1_BASE + 0x138 + n * 0x4
}

// RXF2_PUTGET0..RXF2_PUTGET3
pub fn PIO1_RXF2_PUTGET(n: u32) -> u32 {
    return PIO1_BASE + 0x148 + n * 0x4
}

// RXF3_PUTGET0..RXF3_PUTGET3
pub fn PIO1_RXF3_PUTGET(n: u32) -> u32 {
    return PIO1_BASE + 0x158 + n * 0x4
}

pub const PIO1_GPIOBASE:                            u32 = PIO1_BASE + 0x168;
pub const PIO1_INTR:                                u32 = PIO1_BASE + 0x16C;
// IRQ0_INTE..IRQ1_INTE
pub fn PIO1_IRQ_INTE(n: u32) -> u32 {
    return PIO1_BASE + 0x170 + n * 0xC
}

// IRQ0_INTF..IRQ1_INTF
pub fn PIO1_IRQ_INTF(n: u32) -> u32 {
    return PIO1_BASE + 0x174 + n * 0xC
}

// IRQ0_INTS..IRQ1_INTS
pub fn PIO1_IRQ_INTS(n: u32) -> u32 {
    return PIO1_BASE + 0x178 + n * 0xC
}

pub const PIO2_CTRL:                                u32 = PIO2_BASE + 0x0;
pub const PIO2_FSTAT:                               u32 = PIO2_BASE + 0x4;
pub const PIO2_FDEBUG:                              u32 = PIO2_BASE + 0x8;
pub const PIO2_FLEVEL:                              u32 = PIO2_BASE + 0xC;
// TXF0..TXF3
pub fn PIO2_TXF(n: u32) -> u32 {
    return PIO2_BASE + 0x10 + n * 0x4
}

// RXF0..RXF3
pub fn PIO2_RXF(n: u32) -> u32 {
    return PIO2_BASE + 0x20 + n * 0x4
}

pub const PIO2_IRQ:                                 u32 = PIO2_BASE + 0x30;
pub const PIO2_IRQ_FORCE:                           u32 = PIO2_BASE + 0x34;
pub const PIO2_INPUT_SYNC_BYPASS:                   u32 = PIO2_BASE + 0x38;
pub const PIO2_DBG_PADOUT:                          u32 = PIO2_BASE + 0x3C;
pub const PIO2_DBG_PADOE:                           u32 = PIO2_BASE + 0x40;
pub const PIO2_DBG_CFGINFO:                         u32 = PIO2_BASE + 0x44;
// INSTR_MEM0..INSTR_MEM31
pub fn PIO2_INSTR_MEM(n: u32) -> u32 {
    return PIO2_BASE + 0x48 + n * 0x4
}

// SM0_CLKDIV..SM3_CLKDIV
pub fn PIO2_SM_CLKDIV(n: u32) -> u32 {
    return PIO2_BASE + 0xC8 + n * 0x18
}

// SM0_EXECCTRL..SM3_EXECCTRL
pub fn PIO2_SM_EXECCTRL(n: u32) -> u32 {
    return PIO2_BASE + 0xCC + n * 0x18
}

// SM0_SHIFTCTRL..SM3_SHIFTCTRL
pub fn PIO2_SM_SHIFTCTRL(n: u32) -> u32 {
    return PIO2_BASE + 0xD0 + n * 0x18
}

// SM0_ADDR..SM3_ADDR
pub fn PIO2_SM_ADDR(n: u32) -> u32 {
    return PIO2_BASE + 0xD4 + n * 0x18
}

// SM0_INSTR..SM3_INSTR
pub fn PIO2_SM_INSTR(n: u32) -> u32 {
    return PIO2_BASE + 0xD8 + n * 0x18
}

// SM0_PINCTRL..SM3_PINCTRL
pub fn PIO2_SM_PINCTRL(n: u32) -> u32 {
    return PIO2_BASE + 0xDC + n * 0x18
}

// RXF0_PUTGET0..RXF0_PUTGET3
pub fn PIO2_RXF0_PUTGET(n: u32) -> u32 {
    return PIO2_BASE + 0x128 + n * 0x4
}

// RXF1_PUTGET0..RXF1_PUTGET3
pub fn PIO2_RXF1_PUTGET(n: u32) -> u32 {
    return PIO2_BASE + 0x138 + n * 0x4
}

// RXF2_PUTGET0..RXF2_PUTGET3
pub fn PIO2_RXF2_PUTGET(n: u32) -> u32 {
    return PIO2_BASE + 0x148 + n * 0x4
}

// RXF3_PUTGET0..RXF3_PUTGET3
pub fn PIO2_RXF3_PUTGET(n: u32) -> u32 {
    return PIO2_BASE + 0x158 + n * 0x4
}

pub const PIO2_GPIOBASE:                            u32 = PIO2_BASE + 0x168;
pub const PIO2_INTR:                                u32 = PIO2_BASE + 0x16C;
// IRQ0_INTE..IRQ1_INTE
pub fn PIO2_IRQ_INTE(n: u32) -> u32 {
    return PIO2_BASE + 0x170 + n * 0xC
}

// IRQ0_INTF..IRQ1_INTF
pub fn PIO2_IRQ_INTF(n: u32) -> u32 {
    return PIO2_BASE + 0x174 + n * 0xC
}

// IRQ0_INTS..IRQ1_INTS
pub fn PIO2_IRQ_INTS(n: u32) -> u32 {
    return PIO2_BASE + 0x178 + n * 0xC
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CTRL
pub const PIO_CTRL_NEXTPREV_CLKDIV_RESTART_BIT:     u32 = 26;
pub const PIO_CTRL_NEXTPREV_SM_DISABLE_BIT:         u32 = 25;
pub const PIO_CTRL_NEXTPREV_SM_ENABLE_BIT:          u32 = 24;
pub const PIO_CTRL_NEXT_PIO_MASK_LOW:               u32 = 20;
pub const PIO_CTRL_NEXT_PIO_MASK_HIGH:              u32 = 23;
pub const PIO_CTRL_PREV_PIO_MASK_LOW:               u32 = 16;
pub const PIO_CTRL_PREV_PIO_MASK_HIGH:              u32 = 19;
pub const PIO_CTRL_CLKDIV_RESTART_LOW:              u32 = 8;
pub const PIO_CTRL_CLKDIV_RESTART_HIGH:             u32 = 11;
pub const PIO_CTRL_SM_RESTART_LOW:                  u32 = 4;
pub const PIO_CTRL_SM_RESTART_HIGH:                 u32 = 7;
pub const PIO_CTRL_SM_ENABLE_LOW:                   u32 = 0;
pub const PIO_CTRL_SM_ENABLE_HIGH:                  u32 = 3;

// FSTAT
pub const PIO_FSTAT_TXEMPTY_LOW:                    u32 = 24;
pub const PIO_FSTAT_TXEMPTY_HIGH:                   u32 = 27;
pub const PIO_FSTAT_TXFULL_LOW:                     u32 = 16;
pub const PIO_FSTAT_TXFULL_HIGH:                    u32 = 19;
pub const PIO_FSTAT_RXEMPTY_LOW:                    u32 = 8;
pub const PIO_FSTAT_RXEMPTY_HIGH:                   u32 = 11;
pub const PIO_FSTAT_RXFULL_LOW:                     u32 = 0;
pub const PIO_FSTAT_RXFULL_HIGH:                    u32 = 3;

// FDEBUG
pub const PIO_FDEBUG_TXSTALL_LOW:                   u32 = 24;
pub const PIO_FDEBUG_TXSTALL_HIGH:                  u32 = 27;
pub const PIO_FDEBUG_TXOVER_LOW:                    u32 = 16;
pub const PIO_FDEBUG_TXOVER_HIGH:                   u32 = 19;
pub const PIO_FDEBUG_RXUNDER_LOW:                   u32 = 8;
pub const PIO_FDEBUG_RXUNDER_HIGH:                  u32 = 11;
pub const PIO_FDEBUG_RXSTALL_LOW:                   u32 = 0;
pub const PIO_FDEBUG_RXSTALL_HIGH:                  u32 = 3;

// FLEVEL
pub const PIO_FLEVEL_RX3_LOW:                       u32 = 28;
pub const PIO_FLEVEL_RX3_HIGH:                      u32 = 31;
pub const PIO_FLEVEL_RX2_LOW:                       u32 = 20;
pub const PIO_FLEVEL_RX2_HIGH:                      u32 = 23;
pub const PIO_FLEVEL_RX1_LOW:                       u32 = 12;
pub const PIO_FLEVEL_RX1_HIGH:                      u32 = 15;
pub const PIO_FLEVEL_RX0_LOW:                       u32 = 4;
pub const PIO_FLEVEL_RX0_HIGH:                      u32 = 7;
pub const PIO_FLEVEL_TX3_LOW:                       u32 = 24;
pub const PIO_FLEVEL_TX3_HIGH:                      u32 = 27;
pub const PIO_FLEVEL_TX2_LOW:                       u32 = 16;
pub const PIO_FLEVEL_TX2_HIGH:                      u32 = 19;
pub const PIO_FLEVEL_TX1_LOW:                       u32 = 8;
pub const PIO_FLEVEL_TX1_HIGH:                      u32 = 11;
pub const PIO_FLEVEL_TX0_LOW:                       u32 = 0;
pub const PIO_FLEVEL_TX0_HIGH:                      u32 = 3;

// IRQ
pub const PIO_IRQ_LOW:                              u32 = 0;
pub const PIO_IRQ_HIGH:                             u32 = 7;

// IRQ_FORCE
pub const PIO_IRQ_FORCE_LOW:                        u32 = 0;
pub const PIO_IRQ_FORCE_HIGH:                       u32 = 7;

// DBG_CFGINFO
pub const PIO_DBG_CFGINFO_VERSION_LOW:              u32 = 28;
pub const PIO_DBG_CFGINFO_VERSION_HIGH:             u32 = 31;
pub const PIO_DBG_CFGINFO_IMEM_SIZE_LOW:            u32 = 16;
pub const PIO_DBG_CFGINFO_IMEM_SIZE_HIGH:           u32 = 21;
pub const PIO_DBG_CFGINFO_SM_COUNT_LOW:             u32 = 8;
pub const PIO_DBG_CFGINFO_SM_COUNT_HIGH:            u32 = 11;
pub const PIO_DBG_CFGINFO_FIFO_DEPTH_LOW:           u32 = 0;
pub const PIO_DBG_CFGINFO_FIFO_DEPTH_HIGH:          u32 = 5;

// INSTR_MEM0..INSTR_MEM31
pub const PIO_INSTR_MEM_LOW:                        u32 = 0;
pub const PIO_INSTR_MEM_HIGH:                       u32 = 15;

// SM0_CLKDIV, SM1_CLKDIV, SM2_CLKDIV, SM3_CLKDIV
pub const PIO_SM_CLKDIV_INT_LOW:                    u32 = 16;
pub const PIO_SM_CLKDIV_INT_HIGH:                   u32 = 31;
pub const PIO_SM_CLKDIV_FRAC_LOW:                   u32 = 8;
pub const PIO_SM_CLKDIV_FRAC_HIGH:                  u32 = 15;

// SM0_EXECCTRL, SM1_EXECCTRL, SM2_EXECCTRL, SM3_EXECCTRL
pub const PIO_SM_EXECCTRL_EXEC_STALLED_BIT:         u32 = 31;
pub const PIO_SM_EXECCTRL_SIDE_EN_BIT:              u32 = 30;
pub const PIO_SM_EXECCTRL_SIDE_PINDIR_BIT:          u32 = 29;
pub const PIO_SM_EXECCTRL_JMP_PIN_LOW:              u32 = 24;
pub const PIO_SM_EXECCTRL_JMP_PIN_HIGH:             u32 = 28;
pub const PIO_SM_EXECCTRL_OUT_EN_SEL_LOW:           u32 = 19;
pub const PIO_SM_EXECCTRL_OUT_EN_SEL_HIGH:          u32 = 23;
pub const PIO_SM_EXECCTRL_INLINE_OUT_EN_BIT:        u32 = 18;
pub const PIO_SM_EXECCTRL_OUT_STICKY_BIT:           u32 = 17;
pub const PIO_SM_EXECCTRL_WRAP_TOP_LOW:             u32 = 12;
pub const PIO_SM_EXECCTRL_WRAP_TOP_HIGH:            u32 = 16;
pub const PIO_SM_EXECCTRL_WRAP_BOTTOM_LOW:          u32 = 7;
pub const PIO_SM_EXECCTRL_WRAP_BOTTOM_HIGH:         u32 = 11;
pub const PIO_SM_EXECCTRL_STATUS_SEL_LOW:           u32 = 5;
pub const PIO_SM_EXECCTRL_STATUS_SEL_HIGH:          u32 = 6;
pub const PIO_SM_EXECCTRL_STATUS_N_LOW:             u32 = 0;
pub const PIO_SM_EXECCTRL_STATUS_N_HIGH:            u32 = 4;

// SM0_SHIFTCTRL, SM1_SHIFTCTRL, SM2_SHIFTCTRL, SM3_SHIFTCTRL
pub const PIO_SM_SHIFTCTRL_FJOIN_RX_BIT:            u32 = 31;
pub const PIO_SM_SHIFTCTRL_FJOIN_TX_BIT:            u32 = 30;
pub const PIO_SM_SHIFTCTRL_PULL_THRESH_LOW:         u32 = 25;
pub const PIO_SM_SHIFTCTRL_PULL_THRESH_HIGH:        u32 = 29;
pub const PIO_SM_SHIFTCTRL_PUSH_THRESH_LOW:         u32 = 20;
pub const PIO_SM_SHIFTCTRL_PUSH_THRESH_HIGH:        u32 = 24;
pub const PIO_SM_SHIFTCTRL_OUT_SHIFTDIR_BIT:        u32 = 19;
pub const PIO_SM_SHIFTCTRL_IN_SHIFTDIR_BIT:         u32 = 18;
pub const PIO_SM_SHIFTCTRL_AUTOPULL_BIT:            u32 = 17;
pub const PIO_SM_SHIFTCTRL_AUTOPUSH_BIT:            u32 = 16;
pub const PIO_SM_SHIFTCTRL_FJOIN_RX_PUT_BIT:        u32 = 15;
pub const PIO_SM_SHIFTCTRL_FJOIN_RX_GET_BIT:        u32 = 14;
pub const PIO_SM_SHIFTCTRL_IN_COUNT_LOW:            u32 = 0;
pub const PIO_SM_SHIFTCTRL_IN_COUNT_HIGH:           u32 = 4;

// SM0_ADDR, SM1_ADDR, SM2_ADDR, SM3_ADDR
pub const PIO_SM_ADDR_LOW:                          u32 = 0;
pub const PIO_SM_ADDR_HIGH:                         u32 = 4;

// SM0_INSTR, SM1_INSTR, SM2_INSTR, SM3_INSTR
pub const PIO_SM_INSTR_LOW:                         u32 = 0;
pub const PIO_SM_INSTR_HIGH:                        u32 = 15;

// SM0_PINCTRL, SM1_PINCTRL, SM2_PINCTRL, SM3_PINCTRL
pub const PIO_SM_PINCTRL_SIDESET_COUNT_LOW:         u32 = 29;
pub const PIO_SM_PINCTRL_SIDESET_COUNT_HIGH:        u32 = 31;
pub const PIO_SM_PINCTRL_SET_COUNT_LOW:             u32 = 26;
pub const PIO_SM_PINCTRL_SET_COUNT_HIGH:            u32 = 28;
pub const PIO_SM_PINCTRL_OUT_COUNT_LOW:             u32 = 20;
pub const PIO_SM_PINCTRL_OUT_COUNT_HIGH:            u32 = 25;
pub const PIO_SM_PINCTRL_IN_BASE_LOW:               u32 = 15;
pub const PIO_SM_PINCTRL_IN_BASE_HIGH:              u32 = 19;
pub const PIO_SM_PINCTRL_SIDESET_BASE_LOW:          u32 = 10;
pub const PIO_SM_PINCTRL_SIDESET_BASE_HIGH:         u32 = 14;
pub const PIO_SM_PINCTRL_SET_BASE_LOW:              u32 = 5;
pub const PIO_SM_PINCTRL_SET_BASE_HIGH:             u32 = 9;
pub const PIO_SM_PINCTRL_OUT_BASE_LOW:              u32 = 0;
pub const PIO_SM_PINCTRL_OUT_BASE_HIGH:             u32 = 4;

// GPIOBASE
pub const PIO_GPIOBASE_BIT:                         u32 = 4;

// INTR
pub const PIO_INTR_SM7_BIT:                         u32 = 15;
pub const PIO_INTR_SM6_BIT:                         u32 = 14;
pub const PIO_INTR_SM5_BIT:                         u32 = 13;
pub const PIO_INTR_SM4_BIT:                         u32 = 12;
pub const PIO_INTR_SM3_BIT:                         u32 = 11;
pub const PIO_INTR_SM2_BIT:                         u32 = 10;
pub const PIO_INTR_SM1_BIT:                         u32 = 9;
pub const PIO_INTR_SM0_BIT:                         u32 = 8;
pub const PIO_INTR_SM3_TXNFULL_BIT:                 u32 = 7;
pub const PIO_INTR_SM2_TXNFULL_BIT:                 u32 = 6;
pub const PIO_INTR_SM1_TXNFULL_BIT:                 u32 = 5;
pub const PIO_INTR_SM0_TXNFULL_BIT:                 u32 = 4;
pub const PIO_INTR_SM3_RXNEMPTY_BIT:                u32 = 3;
pub const PIO_INTR_SM2_RXNEMPTY_BIT:                u32 = 2;
pub const PIO_INTR_SM1_RXNEMPTY_BIT:                u32 = 1;
pub const PIO_INTR_SM0_RXNEMPTY_BIT:                u32 = 0;

// IRQ0_INTE, IRQ1_INTE
pub const PIO_IRQ_INTE_SM7_BIT:                     u32 = 15;
pub const PIO_IRQ_INTE_SM6_BIT:                     u32 = 14;
pub const PIO_IRQ_INTE_SM5_BIT:                     u32 = 13;
pub const PIO_IRQ_INTE_SM4_BIT:                     u32 = 12;
pub const PIO_IRQ_INTE_SM3_BIT:                     u32 = 11;
pub const PIO_IRQ_INTE_SM2_BIT:                     u32 = 10;
pub const PIO_IRQ_INTE_SM1_BIT:                     u32 = 9;
pub const PIO_IRQ_INTE_SM0_BIT:                     u32 = 8;
pub const PIO_IRQ_INTE_SM3_TXNFULL_BIT:             u32 = 7;
pub const PIO_IRQ_INTE_SM2_TXNFULL_BIT:             u32 = 6;
pub const PIO_IRQ_INTE_SM1_TXNFULL_BIT:             u32 = 5;
pub const PIO_IRQ_INTE_SM0_TXNFULL_BIT:             u32 = 4;
pub const PIO_IRQ_INTE_SM3_RXNEMPTY_BIT:            u32 = 3;
pub const PIO_IRQ_INTE_SM2_RXNEMPTY_BIT:            u32 = 2;
pub const PIO_IRQ_INTE_SM1_RXNEMPTY_BIT:            u32 = 1;
pub const PIO_IRQ_INTE_SM0_RXNEMPTY_BIT:            u32 = 0;

// IRQ0_INTF, IRQ1_INTF
pub const PIO_IRQ_INTF_SM7_BIT:                     u32 = 15;
pub const PIO_IRQ_INTF_SM6_BIT:                     u32 = 14;
pub const PIO_IRQ_INTF_SM5_BIT:                     u32 = 13;
pub const PIO_IRQ_INTF_SM4_BIT:                     u32 = 12;
pub const PIO_IRQ_INTF_SM3_BIT:                     u32 = 11;
pub const PIO_IRQ_INTF_SM2_BIT:                     u32 = 10;
pub const PIO_IRQ_INTF_SM1_BIT:                     u32 = 9;
pub const PIO_IRQ_INTF_SM0_BIT:                     u32 = 8;
pub const PIO_IRQ_INTF_SM3_TXNFULL_BIT:             u32 = 7;
pub const PIO_IRQ_INTF_SM2_TXNFULL_BIT:             u32 = 6;
pub const PIO_IRQ_INTF_SM1_TXNFULL_BIT:             u32 = 5;
pub const PIO_IRQ_INTF_SM0_TXNFULL_BIT:             u32 = 4;
pub const PIO_IRQ_INTF_SM3_RXNEMPTY_BIT:            u32 = 3;
pub const PIO_IRQ_INTF_SM2_RXNEMPTY_BIT:            u32 = 2;
pub const PIO_IRQ_INTF_SM1_RXNEMPTY_BIT:            u32 = 1;
pub const PIO_IRQ_INTF_SM0_RXNEMPTY_BIT:            u32 = 0;

// IRQ0_INTS, IRQ1_INTS
pub const PIO_IRQ_INTS_SM7_BIT:                     u32 = 15;
pub const PIO_IRQ_INTS_SM6_BIT:                     u32 = 14;
pub const PIO_IRQ_INTS_SM5_BIT:                     u32 = 13;
pub const PIO_IRQ_INTS_SM4_BIT:                     u32 = 12;
pub const PIO_IRQ_INTS_SM3_BIT:                     u32 = 11;
pub const PIO_IRQ_INTS_SM2_BIT:                     u32 = 10;
pub const PIO_IRQ_INTS_SM1_BIT:                     u32 = 9;
pub const PIO_IRQ_INTS_SM0_BIT:                     u32 = 8;
pub const PIO_IRQ_INTS_SM3_TXNFULL_BIT:             u32 = 7;
pub const PIO_IRQ_INTS_SM2_TXNFULL_BIT:             u32 = 6;
pub const PIO_IRQ_INTS_SM1_TXNFULL_BIT:             u32 = 5;
pub const PIO_IRQ_INTS_SM0_TXNFULL_BIT:             u32 = 4;
pub const PIO_IRQ_INTS_SM3_RXNEMPTY_BIT:            u32 = 3;
pub const PIO_IRQ_INTS_SM2_RXNEMPTY_BIT:            u32 = 2;
pub const PIO_IRQ_INTS_SM1_RXNEMPTY_BIT:            u32 = 1;
pub const PIO_IRQ_INTS_SM0_RXNEMPTY_BIT:            u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// DBG_CFGINFO: VERSION
pub const PIO_DBG_CFGINFO_VERSION_V0:               u32 = 0x0;
pub const PIO_DBG_CFGINFO_VERSION_V1:               u32 = 0x1;

// SM0_EXECCTRL..SM3_EXECCTRL: STATUS_SEL
pub const PIO_SM_EXECCTRL_STATUS_SEL_TXLEVEL:       u32 = 0x0;
pub const PIO_SM_EXECCTRL_STATUS_SEL_RXLEVEL:       u32 = 0x1;
pub const PIO_SM_EXECCTRL_STATUS_SEL_IRQ:           u32 = 0x2;

// SM0_EXECCTRL..SM3_EXECCTRL: STATUS_N
pub const PIO_SM_EXECCTRL_STATUS_N_IRQ:             u32 = 0x0;
pub const PIO_SM_EXECCTRL_STATUS_N_IRQ_PREVPIO:     u32 = 0x8;
pub const PIO_SM_EXECCTRL_STATUS_N_IRQ_NEXTPIO:     u32 = 0x10;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
