#![allow(dead_code)]
// SIO

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const SIO_BASE:                                 u32 = 0xD000_0000;
pub const SIO_NS_BASE:                              u32 = 0xD002_0000;

pub const SIO_CPUID_OFFSET:                         u32 = 0x0;
pub const SIO_GPIO_IN_OFFSET:                       u32 = 0x4;
pub const SIO_GPIO_HI_IN_OFFSET:                    u32 = 0x8;
pub const SIO_GPIO_OUT_OFFSET:                      u32 = 0x10;
pub const SIO_GPIO_HI_OUT_OFFSET:                   u32 = 0x14;
pub const SIO_GPIO_OUT_SET_OFFSET:                  u32 = 0x18;
pub const SIO_GPIO_HI_OUT_SET_OFFSET:               u32 = 0x1C;
pub const SIO_GPIO_OUT_CLR_OFFSET:                  u32 = 0x20;
pub const SIO_GPIO_HI_OUT_CLR_OFFSET:               u32 = 0x24;
pub const SIO_GPIO_OUT_XOR_OFFSET:                  u32 = 0x28;
pub const SIO_GPIO_HI_OUT_XOR_OFFSET:               u32 = 0x2C;
pub const SIO_GPIO_OE_OFFSET:                       u32 = 0x30;
pub const SIO_GPIO_HI_OE_OFFSET:                    u32 = 0x34;
pub const SIO_GPIO_OE_SET_OFFSET:                   u32 = 0x38;
pub const SIO_GPIO_HI_OE_SET_OFFSET:                u32 = 0x3C;
pub const SIO_GPIO_OE_CLR_OFFSET:                   u32 = 0x40;
pub const SIO_GPIO_HI_OE_CLR_OFFSET:                u32 = 0x44;
pub const SIO_GPIO_OE_XOR_OFFSET:                   u32 = 0x48;
pub const SIO_GPIO_HI_OE_XOR_OFFSET:                u32 = 0x4C;
pub const SIO_FIFO_ST_OFFSET:                       u32 = 0x50;
pub const SIO_FIFO_WR_OFFSET:                       u32 = 0x54;
pub const SIO_FIFO_RD_OFFSET:                       u32 = 0x58;
pub const SIO_SPINLOCK_ST_OFFSET:                   u32 = 0x5C;
// INTERP0_ACCUM0..INTERP0_ACCUM1
pub fn SIO_INTERP0_ACCUM_OFFSET(n: u32) -> u32 {
    return 0x80 + n * 0x4
}

// INTERP1_ACCUM0..INTERP1_ACCUM1
pub fn SIO_INTERP1_ACCUM_OFFSET(n: u32) -> u32 {
    return 0xC0 + n * 0x4
}

// INTERP0_BASE0..INTERP0_BASE2
pub fn SIO_INTERP0_BASE_OFFSET(n: u32) -> u32 {
    return 0x88 + n * 0x4
}

// INTERP1_BASE0..INTERP1_BASE2
pub fn SIO_INTERP1_BASE_OFFSET(n: u32) -> u32 {
    return 0xC8 + n * 0x4
}

// INTERP0_POP_LANE0..INTERP0_POP_LANE1
pub fn SIO_INTERP0_POP_LANE_OFFSET(n: u32) -> u32 {
    return 0x94 + n * 0x4
}

// INTERP1_POP_LANE0..INTERP1_POP_LANE1
pub fn SIO_INTERP1_POP_LANE_OFFSET(n: u32) -> u32 {
    return 0xD4 + n * 0x4
}

// INTERP0_POP_FULL..INTERP1_POP_FULL
pub fn SIO_INTERP_POP_FULL_OFFSET(n: u32) -> u32 {
    return 0x9C + n * 0x40
}

// INTERP0_PEEK_LANE0..INTERP0_PEEK_LANE1
pub fn SIO_INTERP0_PEEK_LANE_OFFSET(n: u32) -> u32 {
    return 0xA0 + n * 0x4
}

// INTERP1_PEEK_LANE0..INTERP1_PEEK_LANE1
pub fn SIO_INTERP1_PEEK_LANE_OFFSET(n: u32) -> u32 {
    return 0xE0 + n * 0x4
}

// INTERP0_PEEK_FULL..INTERP1_PEEK_FULL
pub fn SIO_INTERP_PEEK_FULL_OFFSET(n: u32) -> u32 {
    return 0xA8 + n * 0x40
}

// INTERP0_CTRL_LANE0..INTERP0_CTRL_LANE1
pub fn SIO_INTERP0_CTRL_LANE_OFFSET(n: u32) -> u32 {
    return 0xAC + n * 0x4
}

// INTERP1_CTRL_LANE0..INTERP1_CTRL_LANE1
pub fn SIO_INTERP1_CTRL_LANE_OFFSET(n: u32) -> u32 {
    return 0xEC + n * 0x4
}

// INTERP0_ACCUM0_ADD..INTERP0_ACCUM1_ADD
pub fn SIO_INTERP0_ACCUM_ADD_OFFSET(n: u32) -> u32 {
    return 0xB4 + n * 0x4
}

// INTERP1_ACCUM0_ADD..INTERP1_ACCUM1_ADD
pub fn SIO_INTERP1_ACCUM_ADD_OFFSET(n: u32) -> u32 {
    return 0xF4 + n * 0x4
}

// INTERP0_BASE_1AND0..INTERP1_BASE_1AND0
pub fn SIO_INTERP_BASE_1AND0_OFFSET(n: u32) -> u32 {
    return 0xBC + n * 0x40
}

// SPINLOCK0..SPINLOCK31
pub fn SIO_SPINLOCK_OFFSET(n: u32) -> u32 {
    return 0x100 + n * 0x4
}

pub const SIO_DOORBELL_OUT_SET_OFFSET:              u32 = 0x180;
pub const SIO_DOORBELL_OUT_CLR_OFFSET:              u32 = 0x184;
pub const SIO_DOORBELL_IN_SET_OFFSET:               u32 = 0x188;
pub const SIO_DOORBELL_IN_CLR_OFFSET:               u32 = 0x18C;
pub const SIO_PERI_NONSEC_OFFSET:                   u32 = 0x190;
pub const SIO_RISCV_SOFTIRQ_OFFSET:                 u32 = 0x1A0;
pub const SIO_MTIME_CTRL_OFFSET:                    u32 = 0x1A4;
pub const SIO_MTIME_OFFSET:                         u32 = 0x1B0;
pub const SIO_MTIMEH_OFFSET:                        u32 = 0x1B4;
pub const SIO_MTIMECMP_OFFSET:                      u32 = 0x1B8;
pub const SIO_MTIMECMPH_OFFSET:                     u32 = 0x1BC;
pub const SIO_TMDS_CTRL_OFFSET:                     u32 = 0x1C0;
pub const SIO_TMDS_WDATA_OFFSET:                    u32 = 0x1C4;
pub const SIO_TMDS_PEEK_SINGLE_OFFSET:              u32 = 0x1C8;
pub const SIO_TMDS_POP_SINGLE_OFFSET:               u32 = 0x1CC;
// TMDS_PEEK_DOUBLE_L0..TMDS_PEEK_DOUBLE_L2
pub fn SIO_TMDS_PEEK_DOUBLE_L_OFFSET(n: u32) -> u32 {
    return 0x1D0 + n * 0x8
}

// TMDS_POP_DOUBLE_L0..TMDS_POP_DOUBLE_L2
pub fn SIO_TMDS_POP_DOUBLE_L_OFFSET(n: u32) -> u32 {
    return 0x1D4 + n * 0x8
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// GPIO_HI_IN..GPIO_HI_OE_XOR
pub const SIO_GPIO_HI_QSPI_SD_LOW:                  u32 = 28;
pub const SIO_GPIO_HI_QSPI_SD_HIGH:                 u32 = 31;
pub const SIO_GPIO_HI_QSPI_CSN_BIT:                 u32 = 27;
pub const SIO_GPIO_HI_QSPI_SCK_BIT:                 u32 = 26;
pub const SIO_GPIO_HI_USB_DM_BIT:                   u32 = 25;
pub const SIO_GPIO_HI_USB_DP_BIT:                   u32 = 24;
pub const SIO_GPIO_HI_GPIO_LOW:                     u32 = 0;
pub const SIO_GPIO_HI_GPIO_HIGH:                    u32 = 15;

// FIFO_ST
pub const SIO_FIFO_ST_ROE_BIT:                      u32 = 3;
pub const SIO_FIFO_ST_WOF_BIT:                      u32 = 2;
pub const SIO_FIFO_ST_RDY_BIT:                      u32 = 1;
pub const SIO_FIFO_ST_VLD_BIT:                      u32 = 0;

// INTERP0_CTRL_LANE0
pub const SIO_INTERP0_CTRL_LANE0_OVERF_BIT:         u32 = 25;
pub const SIO_INTERP0_CTRL_LANE0_OVERF1_BIT:        u32 = 24;
pub const SIO_INTERP0_CTRL_LANE0_OVERF0_BIT:        u32 = 23;
pub const SIO_INTERP0_CTRL_LANE0_BLEND_BIT:         u32 = 21;
pub const SIO_INTERP0_CTRL_LANE0_FORCE_MSB_LOW:     u32 = 19;
pub const SIO_INTERP0_CTRL_LANE0_FORCE_MSB_HIGH:    u32 = 20;
pub const SIO_INTERP0_CTRL_LANE0_ADD_RAW_BIT:       u32 = 18;
pub const SIO_INTERP0_CTRL_LANE0_CROSS_RESULT_BIT:  u32 = 17;
pub const SIO_INTERP0_CTRL_LANE0_CROSS_INPUT_BIT:   u32 = 16;
pub const SIO_INTERP0_CTRL_LANE0_SIGNED_BIT:        u32 = 15;
pub const SIO_INTERP0_CTRL_LANE0_MASK_MSB_LOW:      u32 = 10;
pub const SIO_INTERP0_CTRL_LANE0_MASK_MSB_HIGH:     u32 = 14;
pub const SIO_INTERP0_CTRL_LANE0_MASK_LSB_LOW:      u32 = 5;
pub const SIO_INTERP0_CTRL_LANE0_MASK_LSB_HIGH:     u32 = 9;
pub const SIO_INTERP0_CTRL_LANE0_SHIFT_LOW:         u32 = 0;
pub const SIO_INTERP0_CTRL_LANE0_SHIFT_HIGH:        u32 = 4;

// INTERP0_CTRL_LANE1, INTERP1_CTRL_LANE1
pub const SIO_INTERP_CTRL_LANE_FORCE_MSB_LOW:       u32 = 19;
pub const SIO_INTERP_CTRL_LANE_FORCE_MSB_HIGH:      u32 = 20;
pub const SIO_INTERP_CTRL_LANE_ADD_RAW_BIT:         u32 = 18;
pub const SIO_INTERP_CTRL_LANE_CROSS_RESULT_BIT:    u32 = 17;
pub const SIO_INTERP_CTRL_LANE_CROSS_INPUT_BIT:     u32 = 16;
pub const SIO_INTERP_CTRL_LANE_SIGNED_BIT:          u32 = 15;
pub const SIO_INTERP_CTRL_LANE_MASK_MSB_LOW:        u32 = 10;
pub const SIO_INTERP_CTRL_LANE_MASK_MSB_HIGH:       u32 = 14;
pub const SIO_INTERP_CTRL_LANE_MASK_LSB_LOW:        u32 = 5;
pub const SIO_INTERP_CTRL_LANE_MASK_LSB_HIGH:       u32 = 9;
pub const SIO_INTERP_CTRL_LANE_SHIFT_LOW:           u32 = 0;
pub const SIO_INTERP_CTRL_LANE_SHIFT_HIGH:          u32 = 4;

// INTERP0_ACCUM0_ADD, INTERP0_ACCUM1_ADD, INTERP1_ACCUM0_ADD, INTERP1_ACCUM1_ADD
pub const SIO_INTERP_ACCUM_ADD_LOW:                 u32 = 0;
pub const SIO_INTERP_ACCUM_ADD_HIGH:                u32 = 23;

// INTERP1_CTRL_LANE0
pub const SIO_INTERP1_CTRL_LANE0_OVERF_BIT:         u32 = 25;
pub const SIO_INTERP1_CTRL_LANE0_OVERF1_BIT:        u32 = 24;
pub const SIO_INTERP1_CTRL_LANE0_OVERF0_BIT:        u32 = 23;
pub const SIO_INTERP1_CTRL_LANE0_CLAMP_BIT:         u32 = 22;
pub const SIO_INTERP1_CTRL_LANE0_FORCE_MSB_LOW:     u32 = 19;
pub const SIO_INTERP1_CTRL_LANE0_FORCE_MSB_HIGH:    u32 = 20;
pub const SIO_INTERP1_CTRL_LANE0_ADD_RAW_BIT:       u32 = 18;
pub const SIO_INTERP1_CTRL_LANE0_CROSS_RESULT_BIT:  u32 = 17;
pub const SIO_INTERP1_CTRL_LANE0_CROSS_INPUT_BIT:   u32 = 16;
pub const SIO_INTERP1_CTRL_LANE0_SIGNED_BIT:        u32 = 15;
pub const SIO_INTERP1_CTRL_LANE0_MASK_MSB_LOW:      u32 = 10;
pub const SIO_INTERP1_CTRL_LANE0_MASK_MSB_HIGH:     u32 = 14;
pub const SIO_INTERP1_CTRL_LANE0_MASK_LSB_LOW:      u32 = 5;
pub const SIO_INTERP1_CTRL_LANE0_MASK_LSB_HIGH:     u32 = 9;
pub const SIO_INTERP1_CTRL_LANE0_SHIFT_LOW:         u32 = 0;
pub const SIO_INTERP1_CTRL_LANE0_SHIFT_HIGH:        u32 = 4;

// DOORBELL_OUT_SET
pub const SIO_DOORBELL_OUT_SET_LOW:                 u32 = 0;
pub const SIO_DOORBELL_OUT_SET_HIGH:                u32 = 7;

// DOORBELL_OUT_CLR
pub const SIO_DOORBELL_OUT_CLR_LOW:                 u32 = 0;
pub const SIO_DOORBELL_OUT_CLR_HIGH:                u32 = 7;

// DOORBELL_IN_SET
pub const SIO_DOORBELL_IN_SET_LOW:                  u32 = 0;
pub const SIO_DOORBELL_IN_SET_HIGH:                 u32 = 7;

// DOORBELL_IN_CLR
pub const SIO_DOORBELL_IN_CLR_LOW:                  u32 = 0;
pub const SIO_DOORBELL_IN_CLR_HIGH:                 u32 = 7;

// PERI_NONSEC
pub const SIO_PERI_NONSEC_TMDS_BIT:                 u32 = 5;
pub const SIO_PERI_NONSEC_INTERP1_BIT:              u32 = 1;
pub const SIO_PERI_NONSEC_INTERP0_BIT:              u32 = 0;

// RISCV_SOFTIRQ
pub const SIO_RISCV_SOFTIRQ_CORE1_CLR_BIT:          u32 = 9;
pub const SIO_RISCV_SOFTIRQ_CORE0_CLR_BIT:          u32 = 8;
pub const SIO_RISCV_SOFTIRQ_CORE1_SET_BIT:          u32 = 1;
pub const SIO_RISCV_SOFTIRQ_CORE0_SET_BIT:          u32 = 0;

// MTIME_CTRL
pub const SIO_MTIME_CTRL_DBGPAUSE_CORE1_BIT:        u32 = 3;
pub const SIO_MTIME_CTRL_DBGPAUSE_CORE0_BIT:        u32 = 2;
pub const SIO_MTIME_CTRL_FULLSPEED_BIT:             u32 = 1;
pub const SIO_MTIME_CTRL_EN_BIT:                    u32 = 0;

// TMDS_CTRL
pub const SIO_TMDS_CTRL_CLEAR_BALANCE_BIT:          u32 = 28;
pub const SIO_TMDS_CTRL_PIX2_NOSHIFT_BIT:           u32 = 27;
pub const SIO_TMDS_CTRL_PIX_SHIFT_LOW:              u32 = 24;
pub const SIO_TMDS_CTRL_PIX_SHIFT_HIGH:             u32 = 26;
pub const SIO_TMDS_CTRL_INTERLEAVE_BIT:             u32 = 23;
pub const SIO_TMDS_CTRL_L2_NBITS_LOW:               u32 = 18;
pub const SIO_TMDS_CTRL_L2_NBITS_HIGH:              u32 = 20;
pub const SIO_TMDS_CTRL_L1_NBITS_LOW:               u32 = 15;
pub const SIO_TMDS_CTRL_L1_NBITS_HIGH:              u32 = 17;
pub const SIO_TMDS_CTRL_L0_NBITS_LOW:               u32 = 12;
pub const SIO_TMDS_CTRL_L0_NBITS_HIGH:              u32 = 14;
pub const SIO_TMDS_CTRL_L2_ROT_LOW:                 u32 = 8;
pub const SIO_TMDS_CTRL_L2_ROT_HIGH:                u32 = 11;
pub const SIO_TMDS_CTRL_L1_ROT_LOW:                 u32 = 4;
pub const SIO_TMDS_CTRL_L1_ROT_HIGH:                u32 = 7;
pub const SIO_TMDS_CTRL_L0_ROT_LOW:                 u32 = 0;
pub const SIO_TMDS_CTRL_L0_ROT_HIGH:                u32 = 3;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// TMDS_CTRL: PIX_SHIFT
pub const SIO_TMDS_CTRL_PIX_SHIFT_0:                u32 = 0x0;
pub const SIO_TMDS_CTRL_PIX_SHIFT_1:                u32 = 0x1;
pub const SIO_TMDS_CTRL_PIX_SHIFT_2:                u32 = 0x2;
pub const SIO_TMDS_CTRL_PIX_SHIFT_4:                u32 = 0x3;
pub const SIO_TMDS_CTRL_PIX_SHIFT_8:                u32 = 0x4;
pub const SIO_TMDS_CTRL_PIX_SHIFT_16:               u32 = 0x5;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
