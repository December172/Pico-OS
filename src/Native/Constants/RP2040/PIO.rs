#![allow(dead_code)]
// PIO

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

pub const PIO0_BASE:                                u32 = 0x5020_0000;
pub const PIO1_BASE:                                u32 = 0x5030_0000;

pub const PIO_CTRL_OFFSET:                          u32 = 0x0;
pub const PIO_FSTAT_OFFSET:                         u32 = 0x4;
pub const PIO_FDEBUG_OFFSET:                        u32 = 0x8;
pub const PIO_FLEVEL_OFFSET:                        u32 = 0xC;
pub const PIO_TXF0_OFFSET:                          u32 = 0x10;
pub const PIO_TXF1_OFFSET:                          u32 = 0x14;
pub const PIO_TXF2_OFFSET:                          u32 = 0x18;
pub const PIO_TXF3_OFFSET:                          u32 = 0x1C;
pub const PIO_RXF0_OFFSET:                          u32 = 0x20;
pub const PIO_RXF1_OFFSET:                          u32 = 0x24;
pub const PIO_RXF2_OFFSET:                          u32 = 0x28;
pub const PIO_RXF3_OFFSET:                          u32 = 0x2C;
pub const PIO_IRQ_OFFSET:                           u32 = 0x30;
pub const PIO_IRQ_FORCE_OFFSET:                     u32 = 0x34;
pub const PIO_INPUT_SYNC_BYPASS_OFFSET:             u32 = 0x38;
pub const PIO_DBG_PADOUT_OFFSET:                    u32 = 0x3C;
pub const PIO_DBG_PADOE_OFFSET:                     u32 = 0x40;
pub const PIO_DBG_CFGINFO_OFFSET:                   u32 = 0x44;
pub const PIO_INSTR_MEM0_OFFSET:                    u32 = 0x48;
pub const PIO_INSTR_MEM1_OFFSET:                    u32 = 0x4C;
pub const PIO_INSTR_MEM2_OFFSET:                    u32 = 0x50;
pub const PIO_INSTR_MEM3_OFFSET:                    u32 = 0x54;
pub const PIO_INSTR_MEM4_OFFSET:                    u32 = 0x58;
pub const PIO_INSTR_MEM5_OFFSET:                    u32 = 0x5C;
pub const PIO_INSTR_MEM6_OFFSET:                    u32 = 0x60;
pub const PIO_INSTR_MEM7_OFFSET:                    u32 = 0x64;
pub const PIO_INSTR_MEM8_OFFSET:                    u32 = 0x68;
pub const PIO_INSTR_MEM9_OFFSET:                    u32 = 0x6C;
pub const PIO_INSTR_MEM10_OFFSET:                   u32 = 0x70;
pub const PIO_INSTR_MEM11_OFFSET:                   u32 = 0x74;
pub const PIO_INSTR_MEM12_OFFSET:                   u32 = 0x78;
pub const PIO_INSTR_MEM13_OFFSET:                   u32 = 0x7C;
pub const PIO_INSTR_MEM14_OFFSET:                   u32 = 0x80;
pub const PIO_INSTR_MEM15_OFFSET:                   u32 = 0x84;
pub const PIO_INSTR_MEM16_OFFSET:                   u32 = 0x88;
pub const PIO_INSTR_MEM17_OFFSET:                   u32 = 0x8C;
pub const PIO_INSTR_MEM18_OFFSET:                   u32 = 0x90;
pub const PIO_INSTR_MEM19_OFFSET:                   u32 = 0x94;
pub const PIO_INSTR_MEM20_OFFSET:                   u32 = 0x98;
pub const PIO_INSTR_MEM21_OFFSET:                   u32 = 0x9C;
pub const PIO_INSTR_MEM22_OFFSET:                   u32 = 0xA0;
pub const PIO_INSTR_MEM23_OFFSET:                   u32 = 0xA4;
pub const PIO_INSTR_MEM24_OFFSET:                   u32 = 0xA8;
pub const PIO_INSTR_MEM25_OFFSET:                   u32 = 0xAC;
pub const PIO_INSTR_MEM26_OFFSET:                   u32 = 0xB0;
pub const PIO_INSTR_MEM27_OFFSET:                   u32 = 0xB4;
pub const PIO_INSTR_MEM28_OFFSET:                   u32 = 0xB8;
pub const PIO_INSTR_MEM29_OFFSET:                   u32 = 0xBC;
pub const PIO_INSTR_MEM30_OFFSET:                   u32 = 0xC0;
pub const PIO_INSTR_MEM31_OFFSET:                   u32 = 0xC4;
pub const PIO_SM0_CLKDIV_OFFSET:                    u32 = 0xC8;
pub const PIO_SM0_EXECCTRL_OFFSET:                  u32 = 0xCC;
pub const PIO_SM0_SHIFTCTRL_OFFSET:                 u32 = 0xD0;
pub const PIO_SM0_ADDR_OFFSET:                      u32 = 0xD4;
pub const PIO_SM0_INSTR_OFFSET:                     u32 = 0xD8;
pub const PIO_SM0_PINCTRL_OFFSET:                   u32 = 0xDC;
pub const PIO_SM1_CLKDIV_OFFSET:                    u32 = 0xE0;
pub const PIO_SM1_EXECCTRL_OFFSET:                  u32 = 0xE4;
pub const PIO_SM1_SHIFTCTRL_OFFSET:                 u32 = 0xE8;
pub const PIO_SM1_ADDR_OFFSET:                      u32 = 0xEC;
pub const PIO_SM1_INSTR_OFFSET:                     u32 = 0xF0;
pub const PIO_SM1_PINCTRL_OFFSET:                   u32 = 0xF4;
pub const PIO_SM2_CLKDIV_OFFSET:                    u32 = 0xF8;
pub const PIO_SM2_EXECCTRL_OFFSET:                  u32 = 0xFC;
pub const PIO_SM2_SHIFTCTRL_OFFSET:                 u32 = 0x100;
pub const PIO_SM2_ADDR_OFFSET:                      u32 = 0x104;
pub const PIO_SM2_INSTR_OFFSET:                     u32 = 0x108;
pub const PIO_SM2_PINCTRL_OFFSET:                   u32 = 0x10C;
pub const PIO_SM3_CLKDIV_OFFSET:                    u32 = 0x110;
pub const PIO_SM3_EXECCTRL_OFFSET:                  u32 = 0x114;
pub const PIO_SM3_SHIFTCTRL_OFFSET:                 u32 = 0x118;
pub const PIO_SM3_ADDR_OFFSET:                      u32 = 0x11C;
pub const PIO_SM3_INSTR_OFFSET:                     u32 = 0x120;
pub const PIO_SM3_PINCTRL_OFFSET:                   u32 = 0x124;
pub const PIO_INTR_OFFSET:                          u32 = 0x128;
pub const PIO_IRQ0_INTE_OFFSET:                     u32 = 0x12C;
pub const PIO_IRQ0_INTF_OFFSET:                     u32 = 0x130;
pub const PIO_IRQ0_INTS_OFFSET:                     u32 = 0x134;
pub const PIO_IRQ1_INTE_OFFSET:                     u32 = 0x138;
pub const PIO_IRQ1_INTF_OFFSET:                     u32 = 0x13C;
pub const PIO_IRQ1_INTS_OFFSET:                     u32 = 0x140;

// CTRL
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
pub const PIO_FLEVEL_TX3_LOW:                       u32 = 24;
pub const PIO_FLEVEL_TX3_HIGH:                      u32 = 27;
pub const PIO_FLEVEL_RX2_LOW:                       u32 = 20;
pub const PIO_FLEVEL_RX2_HIGH:                      u32 = 23;
pub const PIO_FLEVEL_TX2_LOW:                       u32 = 16;
pub const PIO_FLEVEL_TX2_HIGH:                      u32 = 19;
pub const PIO_FLEVEL_RX1_LOW:                       u32 = 12;
pub const PIO_FLEVEL_RX1_HIGH:                      u32 = 15;
pub const PIO_FLEVEL_TX1_LOW:                       u32 = 8;
pub const PIO_FLEVEL_TX1_HIGH:                      u32 = 11;
pub const PIO_FLEVEL_RX0_LOW:                       u32 = 4;
pub const PIO_FLEVEL_RX0_HIGH:                      u32 = 7;
pub const PIO_FLEVEL_TX0_LOW:                       u32 = 0;
pub const PIO_FLEVEL_TX0_HIGH:                      u32 = 3;
// TXF0
pub const PIO_TXF0_LOW:                             u32 = 0;
pub const PIO_TXF0_HIGH:                            u32 = 31;
// TXF1
pub const PIO_TXF1_LOW:                             u32 = 0;
pub const PIO_TXF1_HIGH:                            u32 = 31;
// TXF2
pub const PIO_TXF2_LOW:                             u32 = 0;
pub const PIO_TXF2_HIGH:                            u32 = 31;
// TXF3
pub const PIO_TXF3_LOW:                             u32 = 0;
pub const PIO_TXF3_HIGH:                            u32 = 31;
// RXF0
pub const PIO_RXF0_LOW:                             u32 = 0;
pub const PIO_RXF0_HIGH:                            u32 = 31;
// RXF1
pub const PIO_RXF1_LOW:                             u32 = 0;
pub const PIO_RXF1_HIGH:                            u32 = 31;
// RXF2
pub const PIO_RXF2_LOW:                             u32 = 0;
pub const PIO_RXF2_HIGH:                            u32 = 31;
// RXF3
pub const PIO_RXF3_LOW:                             u32 = 0;
pub const PIO_RXF3_HIGH:                            u32 = 31;
// IRQ
pub const PIO_IRQ_LOW:                              u32 = 0;
pub const PIO_IRQ_HIGH:                             u32 = 7;
// IRQ_FORCE
pub const PIO_IRQ_FORCE_LOW:                        u32 = 0;
pub const PIO_IRQ_FORCE_HIGH:                       u32 = 7;
// INPUT_SYNC_BYPASS
pub const PIO_INPUT_SYNC_BYPASS_LOW:                u32 = 0;
pub const PIO_INPUT_SYNC_BYPASS_HIGH:               u32 = 31;
// DBG_PADOUT
pub const PIO_DBG_PADOUT_LOW:                       u32 = 0;
pub const PIO_DBG_PADOUT_HIGH:                      u32 = 31;
// DBG_PADOE
pub const PIO_DBG_PADOE_LOW:                        u32 = 0;
pub const PIO_DBG_PADOE_HIGH:                       u32 = 31;
// DBG_CFGINFO
pub const PIO_DBG_CFGINFO_IMEM_SIZE_LOW:            u32 = 16;
pub const PIO_DBG_CFGINFO_IMEM_SIZE_HIGH:           u32 = 21;
pub const PIO_DBG_CFGINFO_SM_COUNT_LOW:             u32 = 8;
pub const PIO_DBG_CFGINFO_SM_COUNT_HIGH:            u32 = 11;
pub const PIO_DBG_CFGINFO_FIFO_DEPTH_LOW:           u32 = 0;
pub const PIO_DBG_CFGINFO_FIFO_DEPTH_HIGH:          u32 = 5;
// INSTR_MEM0
pub const PIO_INSTR_MEM0_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM0_HIGH:                      u32 = 15;
// INSTR_MEM1
pub const PIO_INSTR_MEM1_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM1_HIGH:                      u32 = 15;
// INSTR_MEM2
pub const PIO_INSTR_MEM2_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM2_HIGH:                      u32 = 15;
// INSTR_MEM3
pub const PIO_INSTR_MEM3_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM3_HIGH:                      u32 = 15;
// INSTR_MEM4
pub const PIO_INSTR_MEM4_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM4_HIGH:                      u32 = 15;
// INSTR_MEM5
pub const PIO_INSTR_MEM5_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM5_HIGH:                      u32 = 15;
// INSTR_MEM6
pub const PIO_INSTR_MEM6_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM6_HIGH:                      u32 = 15;
// INSTR_MEM7
pub const PIO_INSTR_MEM7_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM7_HIGH:                      u32 = 15;
// INSTR_MEM8
pub const PIO_INSTR_MEM8_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM8_HIGH:                      u32 = 15;
// INSTR_MEM9
pub const PIO_INSTR_MEM9_LOW:                       u32 = 0;
pub const PIO_INSTR_MEM9_HIGH:                      u32 = 15;
// INSTR_MEM10
pub const PIO_INSTR_MEM10_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM10_HIGH:                     u32 = 15;
// INSTR_MEM11
pub const PIO_INSTR_MEM11_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM11_HIGH:                     u32 = 15;
// INSTR_MEM12
pub const PIO_INSTR_MEM12_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM12_HIGH:                     u32 = 15;
// INSTR_MEM13
pub const PIO_INSTR_MEM13_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM13_HIGH:                     u32 = 15;
// INSTR_MEM14
pub const PIO_INSTR_MEM14_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM14_HIGH:                     u32 = 15;
// INSTR_MEM15
pub const PIO_INSTR_MEM15_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM15_HIGH:                     u32 = 15;
// INSTR_MEM16
pub const PIO_INSTR_MEM16_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM16_HIGH:                     u32 = 15;
// INSTR_MEM17
pub const PIO_INSTR_MEM17_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM17_HIGH:                     u32 = 15;
// INSTR_MEM18
pub const PIO_INSTR_MEM18_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM18_HIGH:                     u32 = 15;
// INSTR_MEM19
pub const PIO_INSTR_MEM19_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM19_HIGH:                     u32 = 15;
// INSTR_MEM20
pub const PIO_INSTR_MEM20_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM20_HIGH:                     u32 = 15;
// INSTR_MEM21
pub const PIO_INSTR_MEM21_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM21_HIGH:                     u32 = 15;
// INSTR_MEM22
pub const PIO_INSTR_MEM22_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM22_HIGH:                     u32 = 15;
// INSTR_MEM23
pub const PIO_INSTR_MEM23_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM23_HIGH:                     u32 = 15;
// INSTR_MEM24
pub const PIO_INSTR_MEM24_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM24_HIGH:                     u32 = 15;
// INSTR_MEM25
pub const PIO_INSTR_MEM25_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM25_HIGH:                     u32 = 15;
// INSTR_MEM26
pub const PIO_INSTR_MEM26_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM26_HIGH:                     u32 = 15;
// INSTR_MEM27
pub const PIO_INSTR_MEM27_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM27_HIGH:                     u32 = 15;
// INSTR_MEM28
pub const PIO_INSTR_MEM28_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM28_HIGH:                     u32 = 15;
// INSTR_MEM29
pub const PIO_INSTR_MEM29_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM29_HIGH:                     u32 = 15;
// INSTR_MEM30
pub const PIO_INSTR_MEM30_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM30_HIGH:                     u32 = 15;
// INSTR_MEM31
pub const PIO_INSTR_MEM31_LOW:                      u32 = 0;
pub const PIO_INSTR_MEM31_HIGH:                     u32 = 15;
// SM0_CLKDIV
pub const PIO_SM0_CLKDIV_INT_LOW:                   u32 = 16;
pub const PIO_SM0_CLKDIV_INT_HIGH:                  u32 = 31;
pub const PIO_SM0_CLKDIV_FRAC_LOW:                  u32 = 8;
pub const PIO_SM0_CLKDIV_FRAC_HIGH:                 u32 = 15;
// SM0_EXECCTRL
pub const PIO_SM0_EXECCTRL_EXEC_STALLED_BIT:        u32 = 31;
pub const PIO_SM0_EXECCTRL_SIDE_EN_BIT:             u32 = 30;
pub const PIO_SM0_EXECCTRL_SIDE_PINDIR_BIT:         u32 = 29;
pub const PIO_SM0_EXECCTRL_JMP_PIN_LOW:             u32 = 24;
pub const PIO_SM0_EXECCTRL_JMP_PIN_HIGH:            u32 = 28;
pub const PIO_SM0_EXECCTRL_OUT_EN_SEL_LOW:          u32 = 19;
pub const PIO_SM0_EXECCTRL_OUT_EN_SEL_HIGH:         u32 = 23;
pub const PIO_SM0_EXECCTRL_INLINE_OUT_EN_BIT:       u32 = 18;
pub const PIO_SM0_EXECCTRL_OUT_STICKY_BIT:          u32 = 17;
pub const PIO_SM0_EXECCTRL_WRAP_TOP_LOW:            u32 = 12;
pub const PIO_SM0_EXECCTRL_WRAP_TOP_HIGH:           u32 = 16;
pub const PIO_SM0_EXECCTRL_WRAP_BOTTOM_LOW:         u32 = 7;
pub const PIO_SM0_EXECCTRL_WRAP_BOTTOM_HIGH:        u32 = 11;
pub const PIO_SM0_EXECCTRL_STATUS_SEL_BIT:          u32 = 4;
pub const PIO_SM0_EXECCTRL_STATUS_N_LOW:            u32 = 0;
pub const PIO_SM0_EXECCTRL_STATUS_N_HIGH:           u32 = 3;
// SM0_SHIFTCTRL
pub const PIO_SM0_SHIFTCTRL_FJOIN_RX_BIT:           u32 = 31;
pub const PIO_SM0_SHIFTCTRL_FJOIN_TX_BIT:           u32 = 30;
pub const PIO_SM0_SHIFTCTRL_PULL_THRESH_LOW:        u32 = 25;
pub const PIO_SM0_SHIFTCTRL_PULL_THRESH_HIGH:       u32 = 29;
pub const PIO_SM0_SHIFTCTRL_PUSH_THRESH_LOW:        u32 = 20;
pub const PIO_SM0_SHIFTCTRL_PUSH_THRESH_HIGH:       u32 = 24;
pub const PIO_SM0_SHIFTCTRL_OUT_SHIFTDIR_BIT:       u32 = 19;
pub const PIO_SM0_SHIFTCTRL_IN_SHIFTDIR_BIT:        u32 = 18;
pub const PIO_SM0_SHIFTCTRL_AUTOPULL_BIT:           u32 = 17;
pub const PIO_SM0_SHIFTCTRL_AUTOPUSH_BIT:           u32 = 16;
// SM0_ADDR
pub const PIO_SM0_ADDR_LOW:                         u32 = 0;
pub const PIO_SM0_ADDR_HIGH:                        u32 = 4;
// SM0_INSTR
pub const PIO_SM0_INSTR_LOW:                        u32 = 0;
pub const PIO_SM0_INSTR_HIGH:                       u32 = 15;
// SM0_PINCTRL
pub const PIO_SM0_PINCTRL_SIDESET_COUNT_LOW:        u32 = 29;
pub const PIO_SM0_PINCTRL_SIDESET_COUNT_HIGH:       u32 = 31;
pub const PIO_SM0_PINCTRL_SET_COUNT_LOW:            u32 = 26;
pub const PIO_SM0_PINCTRL_SET_COUNT_HIGH:           u32 = 28;
pub const PIO_SM0_PINCTRL_OUT_COUNT_LOW:            u32 = 20;
pub const PIO_SM0_PINCTRL_OUT_COUNT_HIGH:           u32 = 25;
pub const PIO_SM0_PINCTRL_IN_BASE_LOW:              u32 = 15;
pub const PIO_SM0_PINCTRL_IN_BASE_HIGH:             u32 = 19;
pub const PIO_SM0_PINCTRL_SIDESET_BASE_LOW:         u32 = 10;
pub const PIO_SM0_PINCTRL_SIDESET_BASE_HIGH:        u32 = 14;
pub const PIO_SM0_PINCTRL_SET_BASE_LOW:             u32 = 5;
pub const PIO_SM0_PINCTRL_SET_BASE_HIGH:            u32 = 9;
pub const PIO_SM0_PINCTRL_OUT_BASE_LOW:             u32 = 0;
pub const PIO_SM0_PINCTRL_OUT_BASE_HIGH:            u32 = 4;
// SM1_CLKDIV
pub const PIO_SM1_CLKDIV_INT_LOW:                   u32 = 16;
pub const PIO_SM1_CLKDIV_INT_HIGH:                  u32 = 31;
pub const PIO_SM1_CLKDIV_FRAC_LOW:                  u32 = 8;
pub const PIO_SM1_CLKDIV_FRAC_HIGH:                 u32 = 15;
// SM1_EXECCTRL
pub const PIO_SM1_EXECCTRL_EXEC_STALLED_BIT:        u32 = 31;
pub const PIO_SM1_EXECCTRL_SIDE_EN_BIT:             u32 = 30;
pub const PIO_SM1_EXECCTRL_SIDE_PINDIR_BIT:         u32 = 29;
pub const PIO_SM1_EXECCTRL_JMP_PIN_LOW:             u32 = 24;
pub const PIO_SM1_EXECCTRL_JMP_PIN_HIGH:            u32 = 28;
pub const PIO_SM1_EXECCTRL_OUT_EN_SEL_LOW:          u32 = 19;
pub const PIO_SM1_EXECCTRL_OUT_EN_SEL_HIGH:         u32 = 23;
pub const PIO_SM1_EXECCTRL_INLINE_OUT_EN_BIT:       u32 = 18;
pub const PIO_SM1_EXECCTRL_OUT_STICKY_BIT:          u32 = 17;
pub const PIO_SM1_EXECCTRL_WRAP_TOP_LOW:            u32 = 12;
pub const PIO_SM1_EXECCTRL_WRAP_TOP_HIGH:           u32 = 16;
pub const PIO_SM1_EXECCTRL_WRAP_BOTTOM_LOW:         u32 = 7;
pub const PIO_SM1_EXECCTRL_WRAP_BOTTOM_HIGH:        u32 = 11;
pub const PIO_SM1_EXECCTRL_STATUS_SEL_BIT:          u32 = 4;
pub const PIO_SM1_EXECCTRL_STATUS_N_LOW:            u32 = 0;
pub const PIO_SM1_EXECCTRL_STATUS_N_HIGH:           u32 = 3;
// SM1_SHIFTCTRL
pub const PIO_SM1_SHIFTCTRL_FJOIN_RX_BIT:           u32 = 31;
pub const PIO_SM1_SHIFTCTRL_FJOIN_TX_BIT:           u32 = 30;
pub const PIO_SM1_SHIFTCTRL_PULL_THRESH_LOW:        u32 = 25;
pub const PIO_SM1_SHIFTCTRL_PULL_THRESH_HIGH:       u32 = 29;
pub const PIO_SM1_SHIFTCTRL_PUSH_THRESH_LOW:        u32 = 20;
pub const PIO_SM1_SHIFTCTRL_PUSH_THRESH_HIGH:       u32 = 24;
pub const PIO_SM1_SHIFTCTRL_OUT_SHIFTDIR_BIT:       u32 = 19;
pub const PIO_SM1_SHIFTCTRL_IN_SHIFTDIR_BIT:        u32 = 18;
pub const PIO_SM1_SHIFTCTRL_AUTOPULL_BIT:           u32 = 17;
pub const PIO_SM1_SHIFTCTRL_AUTOPUSH_BIT:           u32 = 16;
// SM1_ADDR
pub const PIO_SM1_ADDR_LOW:                         u32 = 0;
pub const PIO_SM1_ADDR_HIGH:                        u32 = 4;
// SM1_INSTR
pub const PIO_SM1_INSTR_LOW:                        u32 = 0;
pub const PIO_SM1_INSTR_HIGH:                       u32 = 15;
// SM1_PINCTRL
pub const PIO_SM1_PINCTRL_SIDESET_COUNT_LOW:        u32 = 29;
pub const PIO_SM1_PINCTRL_SIDESET_COUNT_HIGH:       u32 = 31;
pub const PIO_SM1_PINCTRL_SET_COUNT_LOW:            u32 = 26;
pub const PIO_SM1_PINCTRL_SET_COUNT_HIGH:           u32 = 28;
pub const PIO_SM1_PINCTRL_OUT_COUNT_LOW:            u32 = 20;
pub const PIO_SM1_PINCTRL_OUT_COUNT_HIGH:           u32 = 25;
pub const PIO_SM1_PINCTRL_IN_BASE_LOW:              u32 = 15;
pub const PIO_SM1_PINCTRL_IN_BASE_HIGH:             u32 = 19;
pub const PIO_SM1_PINCTRL_SIDESET_BASE_LOW:         u32 = 10;
pub const PIO_SM1_PINCTRL_SIDESET_BASE_HIGH:        u32 = 14;
pub const PIO_SM1_PINCTRL_SET_BASE_LOW:             u32 = 5;
pub const PIO_SM1_PINCTRL_SET_BASE_HIGH:            u32 = 9;
pub const PIO_SM1_PINCTRL_OUT_BASE_LOW:             u32 = 0;
pub const PIO_SM1_PINCTRL_OUT_BASE_HIGH:            u32 = 4;
// SM2_CLKDIV
pub const PIO_SM2_CLKDIV_INT_LOW:                   u32 = 16;
pub const PIO_SM2_CLKDIV_INT_HIGH:                  u32 = 31;
pub const PIO_SM2_CLKDIV_FRAC_LOW:                  u32 = 8;
pub const PIO_SM2_CLKDIV_FRAC_HIGH:                 u32 = 15;
// SM2_EXECCTRL
pub const PIO_SM2_EXECCTRL_EXEC_STALLED_BIT:        u32 = 31;
pub const PIO_SM2_EXECCTRL_SIDE_EN_BIT:             u32 = 30;
pub const PIO_SM2_EXECCTRL_SIDE_PINDIR_BIT:         u32 = 29;
pub const PIO_SM2_EXECCTRL_JMP_PIN_LOW:             u32 = 24;
pub const PIO_SM2_EXECCTRL_JMP_PIN_HIGH:            u32 = 28;
pub const PIO_SM2_EXECCTRL_OUT_EN_SEL_LOW:          u32 = 19;
pub const PIO_SM2_EXECCTRL_OUT_EN_SEL_HIGH:         u32 = 23;
pub const PIO_SM2_EXECCTRL_INLINE_OUT_EN_BIT:       u32 = 18;
pub const PIO_SM2_EXECCTRL_OUT_STICKY_BIT:          u32 = 17;
pub const PIO_SM2_EXECCTRL_WRAP_TOP_LOW:            u32 = 12;
pub const PIO_SM2_EXECCTRL_WRAP_TOP_HIGH:           u32 = 16;
pub const PIO_SM2_EXECCTRL_WRAP_BOTTOM_LOW:         u32 = 7;
pub const PIO_SM2_EXECCTRL_WRAP_BOTTOM_HIGH:        u32 = 11;
pub const PIO_SM2_EXECCTRL_STATUS_SEL_BIT:          u32 = 4;
pub const PIO_SM2_EXECCTRL_STATUS_N_LOW:            u32 = 0;
pub const PIO_SM2_EXECCTRL_STATUS_N_HIGH:           u32 = 3;
// SM2_SHIFTCTRL
pub const PIO_SM2_SHIFTCTRL_FJOIN_RX_BIT:           u32 = 31;
pub const PIO_SM2_SHIFTCTRL_FJOIN_TX_BIT:           u32 = 30;
pub const PIO_SM2_SHIFTCTRL_PULL_THRESH_LOW:        u32 = 25;
pub const PIO_SM2_SHIFTCTRL_PULL_THRESH_HIGH:       u32 = 29;
pub const PIO_SM2_SHIFTCTRL_PUSH_THRESH_LOW:        u32 = 20;
pub const PIO_SM2_SHIFTCTRL_PUSH_THRESH_HIGH:       u32 = 24;
pub const PIO_SM2_SHIFTCTRL_OUT_SHIFTDIR_BIT:       u32 = 19;
pub const PIO_SM2_SHIFTCTRL_IN_SHIFTDIR_BIT:        u32 = 18;
pub const PIO_SM2_SHIFTCTRL_AUTOPULL_BIT:           u32 = 17;
pub const PIO_SM2_SHIFTCTRL_AUTOPUSH_BIT:           u32 = 16;
// SM2_ADDR
pub const PIO_SM2_ADDR_LOW:                         u32 = 0;
pub const PIO_SM2_ADDR_HIGH:                        u32 = 4;
// SM2_INSTR
pub const PIO_SM2_INSTR_LOW:                        u32 = 0;
pub const PIO_SM2_INSTR_HIGH:                       u32 = 15;
// SM2_PINCTRL
pub const PIO_SM2_PINCTRL_SIDESET_COUNT_LOW:        u32 = 29;
pub const PIO_SM2_PINCTRL_SIDESET_COUNT_HIGH:       u32 = 31;
pub const PIO_SM2_PINCTRL_SET_COUNT_LOW:            u32 = 26;
pub const PIO_SM2_PINCTRL_SET_COUNT_HIGH:           u32 = 28;
pub const PIO_SM2_PINCTRL_OUT_COUNT_LOW:            u32 = 20;
pub const PIO_SM2_PINCTRL_OUT_COUNT_HIGH:           u32 = 25;
pub const PIO_SM2_PINCTRL_IN_BASE_LOW:              u32 = 15;
pub const PIO_SM2_PINCTRL_IN_BASE_HIGH:             u32 = 19;
pub const PIO_SM2_PINCTRL_SIDESET_BASE_LOW:         u32 = 10;
pub const PIO_SM2_PINCTRL_SIDESET_BASE_HIGH:        u32 = 14;
pub const PIO_SM2_PINCTRL_SET_BASE_LOW:             u32 = 5;
pub const PIO_SM2_PINCTRL_SET_BASE_HIGH:            u32 = 9;
pub const PIO_SM2_PINCTRL_OUT_BASE_LOW:             u32 = 0;
pub const PIO_SM2_PINCTRL_OUT_BASE_HIGH:            u32 = 4;
// SM3_CLKDIV
pub const PIO_SM3_CLKDIV_INT_LOW:                   u32 = 16;
pub const PIO_SM3_CLKDIV_INT_HIGH:                  u32 = 31;
pub const PIO_SM3_CLKDIV_FRAC_LOW:                  u32 = 8;
pub const PIO_SM3_CLKDIV_FRAC_HIGH:                 u32 = 15;
// SM3_EXECCTRL
pub const PIO_SM3_EXECCTRL_EXEC_STALLED_BIT:        u32 = 31;
pub const PIO_SM3_EXECCTRL_SIDE_EN_BIT:             u32 = 30;
pub const PIO_SM3_EXECCTRL_SIDE_PINDIR_BIT:         u32 = 29;
pub const PIO_SM3_EXECCTRL_JMP_PIN_LOW:             u32 = 24;
pub const PIO_SM3_EXECCTRL_JMP_PIN_HIGH:            u32 = 28;
pub const PIO_SM3_EXECCTRL_OUT_EN_SEL_LOW:          u32 = 19;
pub const PIO_SM3_EXECCTRL_OUT_EN_SEL_HIGH:         u32 = 23;
pub const PIO_SM3_EXECCTRL_INLINE_OUT_EN_BIT:       u32 = 18;
pub const PIO_SM3_EXECCTRL_OUT_STICKY_BIT:          u32 = 17;
pub const PIO_SM3_EXECCTRL_WRAP_TOP_LOW:            u32 = 12;
pub const PIO_SM3_EXECCTRL_WRAP_TOP_HIGH:           u32 = 16;
pub const PIO_SM3_EXECCTRL_WRAP_BOTTOM_LOW:         u32 = 7;
pub const PIO_SM3_EXECCTRL_WRAP_BOTTOM_HIGH:        u32 = 11;
pub const PIO_SM3_EXECCTRL_STATUS_SEL_BIT:          u32 = 4;
pub const PIO_SM3_EXECCTRL_STATUS_N_LOW:            u32 = 0;
pub const PIO_SM3_EXECCTRL_STATUS_N_HIGH:           u32 = 3;
// SM3_SHIFTCTRL
pub const PIO_SM3_SHIFTCTRL_FJOIN_RX_BIT:           u32 = 31;
pub const PIO_SM3_SHIFTCTRL_FJOIN_TX_BIT:           u32 = 30;
pub const PIO_SM3_SHIFTCTRL_PULL_THRESH_LOW:        u32 = 25;
pub const PIO_SM3_SHIFTCTRL_PULL_THRESH_HIGH:       u32 = 29;
pub const PIO_SM3_SHIFTCTRL_PUSH_THRESH_LOW:        u32 = 20;
pub const PIO_SM3_SHIFTCTRL_PUSH_THRESH_HIGH:       u32 = 24;
pub const PIO_SM3_SHIFTCTRL_OUT_SHIFTDIR_BIT:       u32 = 19;
pub const PIO_SM3_SHIFTCTRL_IN_SHIFTDIR_BIT:        u32 = 18;
pub const PIO_SM3_SHIFTCTRL_AUTOPULL_BIT:           u32 = 17;
pub const PIO_SM3_SHIFTCTRL_AUTOPUSH_BIT:           u32 = 16;
// SM3_ADDR
pub const PIO_SM3_ADDR_LOW:                         u32 = 0;
pub const PIO_SM3_ADDR_HIGH:                        u32 = 4;
// SM3_INSTR
pub const PIO_SM3_INSTR_LOW:                        u32 = 0;
pub const PIO_SM3_INSTR_HIGH:                       u32 = 15;
// SM3_PINCTRL
pub const PIO_SM3_PINCTRL_SIDESET_COUNT_LOW:        u32 = 29;
pub const PIO_SM3_PINCTRL_SIDESET_COUNT_HIGH:       u32 = 31;
pub const PIO_SM3_PINCTRL_SET_COUNT_LOW:            u32 = 26;
pub const PIO_SM3_PINCTRL_SET_COUNT_HIGH:           u32 = 28;
pub const PIO_SM3_PINCTRL_OUT_COUNT_LOW:            u32 = 20;
pub const PIO_SM3_PINCTRL_OUT_COUNT_HIGH:           u32 = 25;
pub const PIO_SM3_PINCTRL_IN_BASE_LOW:              u32 = 15;
pub const PIO_SM3_PINCTRL_IN_BASE_HIGH:             u32 = 19;
pub const PIO_SM3_PINCTRL_SIDESET_BASE_LOW:         u32 = 10;
pub const PIO_SM3_PINCTRL_SIDESET_BASE_HIGH:        u32 = 14;
pub const PIO_SM3_PINCTRL_SET_BASE_LOW:             u32 = 5;
pub const PIO_SM3_PINCTRL_SET_BASE_HIGH:            u32 = 9;
pub const PIO_SM3_PINCTRL_OUT_BASE_LOW:             u32 = 0;
pub const PIO_SM3_PINCTRL_OUT_BASE_HIGH:            u32 = 4;
// INTR
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
// IRQ0_INTE
pub const PIO_IRQ0_INTE_SM3_BIT:                    u32 = 11;
pub const PIO_IRQ0_INTE_SM2_BIT:                    u32 = 10;
pub const PIO_IRQ0_INTE_SM1_BIT:                    u32 = 9;
pub const PIO_IRQ0_INTE_SM0_BIT:                    u32 = 8;
pub const PIO_IRQ0_INTE_SM3_TXNFULL_BIT:            u32 = 7;
pub const PIO_IRQ0_INTE_SM2_TXNFULL_BIT:            u32 = 6;
pub const PIO_IRQ0_INTE_SM1_TXNFULL_BIT:            u32 = 5;
pub const PIO_IRQ0_INTE_SM0_TXNFULL_BIT:            u32 = 4;
pub const PIO_IRQ0_INTE_SM3_RXNEMPTY_BIT:           u32 = 3;
pub const PIO_IRQ0_INTE_SM2_RXNEMPTY_BIT:           u32 = 2;
pub const PIO_IRQ0_INTE_SM1_RXNEMPTY_BIT:           u32 = 1;
pub const PIO_IRQ0_INTE_SM0_RXNEMPTY_BIT:           u32 = 0;
// IRQ0_INTF
pub const PIO_IRQ0_INTF_SM3_BIT:                    u32 = 11;
pub const PIO_IRQ0_INTF_SM2_BIT:                    u32 = 10;
pub const PIO_IRQ0_INTF_SM1_BIT:                    u32 = 9;
pub const PIO_IRQ0_INTF_SM0_BIT:                    u32 = 8;
pub const PIO_IRQ0_INTF_SM3_TXNFULL_BIT:            u32 = 7;
pub const PIO_IRQ0_INTF_SM2_TXNFULL_BIT:            u32 = 6;
pub const PIO_IRQ0_INTF_SM1_TXNFULL_BIT:            u32 = 5;
pub const PIO_IRQ0_INTF_SM0_TXNFULL_BIT:            u32 = 4;
pub const PIO_IRQ0_INTF_SM3_RXNEMPTY_BIT:           u32 = 3;
pub const PIO_IRQ0_INTF_SM2_RXNEMPTY_BIT:           u32 = 2;
pub const PIO_IRQ0_INTF_SM1_RXNEMPTY_BIT:           u32 = 1;
pub const PIO_IRQ0_INTF_SM0_RXNEMPTY_BIT:           u32 = 0;
// IRQ0_INTS
pub const PIO_IRQ0_INTS_SM3_BIT:                    u32 = 11;
pub const PIO_IRQ0_INTS_SM2_BIT:                    u32 = 10;
pub const PIO_IRQ0_INTS_SM1_BIT:                    u32 = 9;
pub const PIO_IRQ0_INTS_SM0_BIT:                    u32 = 8;
pub const PIO_IRQ0_INTS_SM3_TXNFULL_BIT:            u32 = 7;
pub const PIO_IRQ0_INTS_SM2_TXNFULL_BIT:            u32 = 6;
pub const PIO_IRQ0_INTS_SM1_TXNFULL_BIT:            u32 = 5;
pub const PIO_IRQ0_INTS_SM0_TXNFULL_BIT:            u32 = 4;
pub const PIO_IRQ0_INTS_SM3_RXNEMPTY_BIT:           u32 = 3;
pub const PIO_IRQ0_INTS_SM2_RXNEMPTY_BIT:           u32 = 2;
pub const PIO_IRQ0_INTS_SM1_RXNEMPTY_BIT:           u32 = 1;
pub const PIO_IRQ0_INTS_SM0_RXNEMPTY_BIT:           u32 = 0;
// IRQ1_INTE
pub const PIO_IRQ1_INTE_SM3_BIT:                    u32 = 11;
pub const PIO_IRQ1_INTE_SM2_BIT:                    u32 = 10;
pub const PIO_IRQ1_INTE_SM1_BIT:                    u32 = 9;
pub const PIO_IRQ1_INTE_SM0_BIT:                    u32 = 8;
pub const PIO_IRQ1_INTE_SM3_TXNFULL_BIT:            u32 = 7;
pub const PIO_IRQ1_INTE_SM2_TXNFULL_BIT:            u32 = 6;
pub const PIO_IRQ1_INTE_SM1_TXNFULL_BIT:            u32 = 5;
pub const PIO_IRQ1_INTE_SM0_TXNFULL_BIT:            u32 = 4;
pub const PIO_IRQ1_INTE_SM3_RXNEMPTY_BIT:           u32 = 3;
pub const PIO_IRQ1_INTE_SM2_RXNEMPTY_BIT:           u32 = 2;
pub const PIO_IRQ1_INTE_SM1_RXNEMPTY_BIT:           u32 = 1;
pub const PIO_IRQ1_INTE_SM0_RXNEMPTY_BIT:           u32 = 0;
// IRQ1_INTF
pub const PIO_IRQ1_INTF_SM3_BIT:                    u32 = 11;
pub const PIO_IRQ1_INTF_SM2_BIT:                    u32 = 10;
pub const PIO_IRQ1_INTF_SM1_BIT:                    u32 = 9;
pub const PIO_IRQ1_INTF_SM0_BIT:                    u32 = 8;
pub const PIO_IRQ1_INTF_SM3_TXNFULL_BIT:            u32 = 7;
pub const PIO_IRQ1_INTF_SM2_TXNFULL_BIT:            u32 = 6;
pub const PIO_IRQ1_INTF_SM1_TXNFULL_BIT:            u32 = 5;
pub const PIO_IRQ1_INTF_SM0_TXNFULL_BIT:            u32 = 4;
pub const PIO_IRQ1_INTF_SM3_RXNEMPTY_BIT:           u32 = 3;
pub const PIO_IRQ1_INTF_SM2_RXNEMPTY_BIT:           u32 = 2;
pub const PIO_IRQ1_INTF_SM1_RXNEMPTY_BIT:           u32 = 1;
pub const PIO_IRQ1_INTF_SM0_RXNEMPTY_BIT:           u32 = 0;
// IRQ1_INTS
pub const PIO_IRQ1_INTS_SM3_BIT:                    u32 = 11;
pub const PIO_IRQ1_INTS_SM2_BIT:                    u32 = 10;
pub const PIO_IRQ1_INTS_SM1_BIT:                    u32 = 9;
pub const PIO_IRQ1_INTS_SM0_BIT:                    u32 = 8;
pub const PIO_IRQ1_INTS_SM3_TXNFULL_BIT:            u32 = 7;
pub const PIO_IRQ1_INTS_SM2_TXNFULL_BIT:            u32 = 6;
pub const PIO_IRQ1_INTS_SM1_TXNFULL_BIT:            u32 = 5;
pub const PIO_IRQ1_INTS_SM0_TXNFULL_BIT:            u32 = 4;
pub const PIO_IRQ1_INTS_SM3_RXNEMPTY_BIT:           u32 = 3;
pub const PIO_IRQ1_INTS_SM2_RXNEMPTY_BIT:           u32 = 2;
pub const PIO_IRQ1_INTS_SM1_RXNEMPTY_BIT:           u32 = 1;
pub const PIO_IRQ1_INTS_SM0_RXNEMPTY_BIT:           u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// SM0_EXECCTRL..SM3_EXECCTRL: STATUS_SEL
pub const PIO_SM_EXECCTRL_STATUS_SEL_TXLEVEL:       u32 = 0x0;
pub const PIO_SM_EXECCTRL_STATUS_SEL_RXLEVEL:       u32 = 0x1;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
