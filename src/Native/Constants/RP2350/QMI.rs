#![allow(dead_code)]
// QMI

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const QMI_BASE:                                 u32 = 0x400D_0000;

pub const QMI_DIRECT_CSR:                           u32 = QMI_BASE + 0x0;
pub const QMI_DIRECT_TX:                            u32 = QMI_BASE + 0x4;
pub const QMI_DIRECT_RX:                            u32 = QMI_BASE + 0x8;
// M0_TIMING..M1_TIMING
pub fn QMI_M_TIMING(n: u32) -> u32 {
    return QMI_BASE + 0xC + n * 0x14
}

// M0_RFMT..M1_RFMT
pub fn QMI_M_RFMT(n: u32) -> u32 {
    return QMI_BASE + 0x10 + n * 0x14
}

// M0_RCMD..M1_RCMD
pub fn QMI_M_RCMD(n: u32) -> u32 {
    return QMI_BASE + 0x14 + n * 0x14
}

// M0_WFMT..M1_WFMT
pub fn QMI_M_WFMT(n: u32) -> u32 {
    return QMI_BASE + 0x18 + n * 0x14
}

// M0_WCMD..M1_WCMD
pub fn QMI_M_WCMD(n: u32) -> u32 {
    return QMI_BASE + 0x1C + n * 0x14
}

// ATRANS0..ATRANS7
pub fn QMI_ATRANS(n: u32) -> u32 {
    return QMI_BASE + 0x34 + n * 0x4
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// DIRECT_CSR
pub const QMI_DIRECT_CSR_RXDELAY_LOW:               u32 = 30;
pub const QMI_DIRECT_CSR_RXDELAY_HIGH:              u32 = 31;
pub const QMI_DIRECT_CSR_CLKDIV_LOW:                u32 = 22;
pub const QMI_DIRECT_CSR_CLKDIV_HIGH:               u32 = 29;
pub const QMI_DIRECT_CSR_RXLEVEL_LOW:               u32 = 18;
pub const QMI_DIRECT_CSR_RXLEVEL_HIGH:              u32 = 20;
pub const QMI_DIRECT_CSR_RXFULL_BIT:                u32 = 17;
pub const QMI_DIRECT_CSR_RXEMPTY_BIT:               u32 = 16;
pub const QMI_DIRECT_CSR_TXLEVEL_LOW:               u32 = 12;
pub const QMI_DIRECT_CSR_TXLEVEL_HIGH:              u32 = 14;
pub const QMI_DIRECT_CSR_TXEMPTY_BIT:               u32 = 11;
pub const QMI_DIRECT_CSR_TXFULL_BIT:                u32 = 10;
pub const QMI_DIRECT_CSR_AUTO_CS1N_BIT:             u32 = 7;
pub const QMI_DIRECT_CSR_AUTO_CS0N_BIT:             u32 = 6;
pub const QMI_DIRECT_CSR_ASSERT_CS1N_BIT:           u32 = 3;
pub const QMI_DIRECT_CSR_ASSERT_CS0N_BIT:           u32 = 2;
pub const QMI_DIRECT_CSR_BUSY_BIT:                  u32 = 1;
pub const QMI_DIRECT_CSR_EN_BIT:                    u32 = 0;

// DIRECT_TX
pub const QMI_DIRECT_TX_NOPUSH_BIT:                 u32 = 20;
pub const QMI_DIRECT_TX_OE_BIT:                     u32 = 19;
pub const QMI_DIRECT_TX_DWIDTH_BIT:                 u32 = 18;
pub const QMI_DIRECT_TX_IWIDTH_LOW:                 u32 = 16;
pub const QMI_DIRECT_TX_IWIDTH_HIGH:                u32 = 17;
pub const QMI_DIRECT_TX_DATA_LOW:                   u32 = 0;
pub const QMI_DIRECT_TX_DATA_HIGH:                  u32 = 15;

// DIRECT_RX
pub const QMI_DIRECT_RX_LOW:                        u32 = 0;
pub const QMI_DIRECT_RX_HIGH:                       u32 = 15;

// M0_TIMING, M1_TIMING
pub const QMI_M_TIMING_COOLDOWN_LOW:                u32 = 30;
pub const QMI_M_TIMING_COOLDOWN_HIGH:               u32 = 31;
pub const QMI_M_TIMING_PAGEBREAK_LOW:               u32 = 28;
pub const QMI_M_TIMING_PAGEBREAK_HIGH:              u32 = 29;
pub const QMI_M_TIMING_SELECT_SETUP_BIT:            u32 = 25;
pub const QMI_M_TIMING_SELECT_HOLD_LOW:             u32 = 23;
pub const QMI_M_TIMING_SELECT_HOLD_HIGH:            u32 = 24;
pub const QMI_M_TIMING_MAX_SELECT_LOW:              u32 = 17;
pub const QMI_M_TIMING_MAX_SELECT_HIGH:             u32 = 22;
pub const QMI_M_TIMING_MIN_DESELECT_LOW:            u32 = 12;
pub const QMI_M_TIMING_MIN_DESELECT_HIGH:           u32 = 16;
pub const QMI_M_TIMING_RXDELAY_LOW:                 u32 = 8;
pub const QMI_M_TIMING_RXDELAY_HIGH:                u32 = 10;
pub const QMI_M_TIMING_CLKDIV_LOW:                  u32 = 0;
pub const QMI_M_TIMING_CLKDIV_HIGH:                 u32 = 7;

// M0_RFMT, M0_WFMT, M1_RFMT, M1_WFMT
pub const QMI_M_DTR_BIT:                            u32 = 28;
pub const QMI_M_DUMMY_LEN_LOW:                      u32 = 16;
pub const QMI_M_DUMMY_LEN_HIGH:                     u32 = 18;
pub const QMI_M_SUFFIX_LEN_LOW:                     u32 = 14;
pub const QMI_M_SUFFIX_LEN_HIGH:                    u32 = 15;
pub const QMI_M_PREFIX_LEN_BIT:                     u32 = 12;
pub const QMI_M_DATA_WIDTH_LOW:                     u32 = 8;
pub const QMI_M_DATA_WIDTH_HIGH:                    u32 = 9;
pub const QMI_M_DUMMY_WIDTH_LOW:                    u32 = 6;
pub const QMI_M_DUMMY_WIDTH_HIGH:                   u32 = 7;
pub const QMI_M_SUFFIX_WIDTH_LOW:                   u32 = 4;
pub const QMI_M_SUFFIX_WIDTH_HIGH:                  u32 = 5;
pub const QMI_M_ADDR_WIDTH_LOW:                     u32 = 2;
pub const QMI_M_ADDR_WIDTH_HIGH:                    u32 = 3;
pub const QMI_M_PREFIX_WIDTH_LOW:                   u32 = 0;
pub const QMI_M_PREFIX_WIDTH_HIGH:                  u32 = 1;

// M0_RCMD, M1_RCMD
pub const QMI_M_RCMD_SUFFIX_LOW:                    u32 = 8;
pub const QMI_M_RCMD_SUFFIX_HIGH:                   u32 = 15;
pub const QMI_M_RCMD_PREFIX_LOW:                    u32 = 0;
pub const QMI_M_RCMD_PREFIX_HIGH:                   u32 = 7;

// M0_WCMD, M1_WCMD
pub const QMI_M_WCMD_SUFFIX_LOW:                    u32 = 8;
pub const QMI_M_WCMD_SUFFIX_HIGH:                   u32 = 15;
pub const QMI_M_WCMD_PREFIX_LOW:                    u32 = 0;
pub const QMI_M_WCMD_PREFIX_HIGH:                   u32 = 7;

// ATRANS0, ATRANS1, ATRANS2, ATRANS3, ATRANS4, ATRANS5, ATRANS6, ATRANS7
pub const QMI_ATRANS_SIZE_LOW:                      u32 = 16;
pub const QMI_ATRANS_SIZE_HIGH:                     u32 = 26;
pub const QMI_ATRANS_BASE_LOW:                      u32 = 0;
pub const QMI_ATRANS_BASE_HIGH:                     u32 = 11;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// DIRECT_TX: IWIDTH
pub const QMI_DIRECT_TX_IWIDTH_S:                   u32 = 0x0;
pub const QMI_DIRECT_TX_IWIDTH_D:                   u32 = 0x1;
pub const QMI_DIRECT_TX_IWIDTH_Q:                   u32 = 0x2;

// M0_TIMING..M1_TIMING: PAGEBREAK
pub const QMI_M_TIMING_PAGEBREAK_NONE:              u32 = 0x0;
pub const QMI_M_TIMING_PAGEBREAK_256:               u32 = 0x1;
pub const QMI_M_TIMING_PAGEBREAK_1024:              u32 = 0x2;
pub const QMI_M_TIMING_PAGEBREAK_4096:              u32 = 0x3;

// M0_RFMT..M1_WFMT: DUMMY_LEN
pub const QMI_M_DUMMY_LEN_NONE:                     u32 = 0x0;
pub const QMI_M_DUMMY_LEN_4:                        u32 = 0x1;
pub const QMI_M_DUMMY_LEN_8:                        u32 = 0x2;
pub const QMI_M_DUMMY_LEN_12:                       u32 = 0x3;
pub const QMI_M_DUMMY_LEN_16:                       u32 = 0x4;
pub const QMI_M_DUMMY_LEN_20:                       u32 = 0x5;
pub const QMI_M_DUMMY_LEN_24:                       u32 = 0x6;
pub const QMI_M_DUMMY_LEN_28:                       u32 = 0x7;

// M0_RFMT..M1_WFMT: SUFFIX_LEN
pub const QMI_M_SUFFIX_LEN_NONE:                    u32 = 0x0;
pub const QMI_M_SUFFIX_LEN_8:                       u32 = 0x2;

// M0_RFMT..M1_WFMT: PREFIX_LEN
pub const QMI_M_PREFIX_LEN_NONE:                    u32 = 0x0;
pub const QMI_M_PREFIX_LEN_8:                       u32 = 0x1;

// M0_RFMT..M1_WFMT: DATA_WIDTH
pub const QMI_M_DATA_WIDTH_S:                       u32 = 0x0;
pub const QMI_M_DATA_WIDTH_D:                       u32 = 0x1;
pub const QMI_M_DATA_WIDTH_Q:                       u32 = 0x2;

// M0_RFMT..M1_WFMT: DUMMY_WIDTH
pub const QMI_M_DUMMY_WIDTH_S:                      u32 = 0x0;
pub const QMI_M_DUMMY_WIDTH_D:                      u32 = 0x1;
pub const QMI_M_DUMMY_WIDTH_Q:                      u32 = 0x2;

// M0_RFMT..M1_WFMT: SUFFIX_WIDTH
pub const QMI_M_SUFFIX_WIDTH_S:                     u32 = 0x0;
pub const QMI_M_SUFFIX_WIDTH_D:                     u32 = 0x1;
pub const QMI_M_SUFFIX_WIDTH_Q:                     u32 = 0x2;

// M0_RFMT..M1_WFMT: ADDR_WIDTH
pub const QMI_M_ADDR_WIDTH_S:                       u32 = 0x0;
pub const QMI_M_ADDR_WIDTH_D:                       u32 = 0x1;
pub const QMI_M_ADDR_WIDTH_Q:                       u32 = 0x2;

// M0_RFMT..M1_WFMT: PREFIX_WIDTH
pub const QMI_M_PREFIX_WIDTH_S:                     u32 = 0x0;
pub const QMI_M_PREFIX_WIDTH_D:                     u32 = 0x1;
pub const QMI_M_PREFIX_WIDTH_Q:                     u32 = 0x2;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
