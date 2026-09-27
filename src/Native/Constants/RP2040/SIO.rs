#![allow(dead_code)]
// SIO

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

pub const SIO_BASE:                                 u32 = 0xD000_0000;

pub const SIO_CPUID:                                u32 = SIO_BASE + 0x0;
pub const SIO_GPIO_IN:                              u32 = SIO_BASE + 0x4;
pub const SIO_GPIO_HI_IN:                           u32 = SIO_BASE + 0x8;
pub const SIO_GPIO_OUT:                             u32 = SIO_BASE + 0x10;
pub const SIO_GPIO_OUT_SET:                         u32 = SIO_BASE + 0x14;
pub const SIO_GPIO_OUT_CLR:                         u32 = SIO_BASE + 0x18;
pub const SIO_GPIO_OUT_XOR:                         u32 = SIO_BASE + 0x1C;
pub const SIO_GPIO_OE:                              u32 = SIO_BASE + 0x20;
pub const SIO_GPIO_OE_SET:                          u32 = SIO_BASE + 0x24;
pub const SIO_GPIO_OE_CLR:                          u32 = SIO_BASE + 0x28;
pub const SIO_GPIO_OE_XOR:                          u32 = SIO_BASE + 0x2C;
pub const SIO_GPIO_HI_OUT:                          u32 = SIO_BASE + 0x30;
pub const SIO_GPIO_HI_OUT_SET:                      u32 = SIO_BASE + 0x34;
pub const SIO_GPIO_HI_OUT_CLR:                      u32 = SIO_BASE + 0x38;
pub const SIO_GPIO_HI_OUT_XOR:                      u32 = SIO_BASE + 0x3C;
pub const SIO_GPIO_HI_OE:                           u32 = SIO_BASE + 0x40;
pub const SIO_GPIO_HI_OE_SET:                       u32 = SIO_BASE + 0x44;
pub const SIO_GPIO_HI_OE_CLR:                       u32 = SIO_BASE + 0x48;
pub const SIO_GPIO_HI_OE_XOR:                       u32 = SIO_BASE + 0x4C;
pub const SIO_FIFO_ST:                              u32 = SIO_BASE + 0x50;
pub const SIO_FIFO_WR:                              u32 = SIO_BASE + 0x54;
pub const SIO_FIFO_RD:                              u32 = SIO_BASE + 0x58;
pub const SIO_SPINLOCK_ST:                          u32 = SIO_BASE + 0x5C;
pub const SIO_DIV_UDIVIDEND:                        u32 = SIO_BASE + 0x60;
pub const SIO_DIV_UDIVISOR:                         u32 = SIO_BASE + 0x64;
pub const SIO_DIV_SDIVIDEND:                        u32 = SIO_BASE + 0x68;
pub const SIO_DIV_SDIVISOR:                         u32 = SIO_BASE + 0x6C;
pub const SIO_DIV_QUOTIENT:                         u32 = SIO_BASE + 0x70;
pub const SIO_DIV_REMAINDER:                        u32 = SIO_BASE + 0x74;
pub const SIO_DIV_CSR:                              u32 = SIO_BASE + 0x78;
// INTERP0_ACCUM0..INTERP0_ACCUM1
pub fn SIO_INTERP0_ACCUM(n: u32) -> u32 {
    return SIO_BASE + 0x80 + n * 0x4
}

// INTERP1_ACCUM0..INTERP1_ACCUM1
pub fn SIO_INTERP1_ACCUM(n: u32) -> u32 {
    return SIO_BASE + 0xC0 + n * 0x4
}

// INTERP0_BASE0..INTERP0_BASE2
pub fn SIO_INTERP0_BASE(n: u32) -> u32 {
    return SIO_BASE + 0x88 + n * 0x4
}

// INTERP1_BASE0..INTERP1_BASE2
pub fn SIO_INTERP1_BASE(n: u32) -> u32 {
    return SIO_BASE + 0xC8 + n * 0x4
}

// INTERP0_POP_LANE0..INTERP0_POP_LANE1
pub fn SIO_INTERP0_POP_LANE(n: u32) -> u32 {
    return SIO_BASE + 0x94 + n * 0x4
}

// INTERP1_POP_LANE0..INTERP1_POP_LANE1
pub fn SIO_INTERP1_POP_LANE(n: u32) -> u32 {
    return SIO_BASE + 0xD4 + n * 0x4
}

// INTERP0_POP_FULL..INTERP1_POP_FULL
pub fn SIO_INTERP_POP_FULL(n: u32) -> u32 {
    return SIO_BASE + 0x9C + n * 0x40
}

// INTERP0_PEEK_LANE0..INTERP0_PEEK_LANE1
pub fn SIO_INTERP0_PEEK_LANE(n: u32) -> u32 {
    return SIO_BASE + 0xA0 + n * 0x4
}

// INTERP1_PEEK_LANE0..INTERP1_PEEK_LANE1
pub fn SIO_INTERP1_PEEK_LANE(n: u32) -> u32 {
    return SIO_BASE + 0xE0 + n * 0x4
}

// INTERP0_PEEK_FULL..INTERP1_PEEK_FULL
pub fn SIO_INTERP_PEEK_FULL(n: u32) -> u32 {
    return SIO_BASE + 0xA8 + n * 0x40
}

// INTERP0_CTRL_LANE0..INTERP0_CTRL_LANE1
pub fn SIO_INTERP0_CTRL_LANE(n: u32) -> u32 {
    return SIO_BASE + 0xAC + n * 0x4
}

// INTERP1_CTRL_LANE0..INTERP1_CTRL_LANE1
pub fn SIO_INTERP1_CTRL_LANE(n: u32) -> u32 {
    return SIO_BASE + 0xEC + n * 0x4
}

// INTERP0_ACCUM0_ADD..INTERP0_ACCUM1_ADD
pub fn SIO_INTERP0_ACCUM_ADD(n: u32) -> u32 {
    return SIO_BASE + 0xB4 + n * 0x4
}

// INTERP1_ACCUM0_ADD..INTERP1_ACCUM1_ADD
pub fn SIO_INTERP1_ACCUM_ADD(n: u32) -> u32 {
    return SIO_BASE + 0xF4 + n * 0x4
}

// INTERP0_BASE_1AND0..INTERP1_BASE_1AND0
pub fn SIO_INTERP_BASE_1AND0(n: u32) -> u32 {
    return SIO_BASE + 0xBC + n * 0x40
}

// SPINLOCK0..SPINLOCK31
pub fn SIO_SPINLOCK(n: u32) -> u32 {
    return SIO_BASE + 0x100 + n * 0x4
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// GPIO_IN
pub const SIO_GPIO_IN_LOW:                          u32 = 0;
pub const SIO_GPIO_IN_HIGH:                         u32 = 29;

// GPIO_HI_IN
pub const SIO_GPIO_HI_IN_LOW:                       u32 = 0;
pub const SIO_GPIO_HI_IN_HIGH:                      u32 = 5;

// GPIO_OUT
pub const SIO_GPIO_OUT_LOW:                         u32 = 0;
pub const SIO_GPIO_OUT_HIGH:                        u32 = 29;

// GPIO_OUT_SET
pub const SIO_GPIO_OUT_SET_LOW:                     u32 = 0;
pub const SIO_GPIO_OUT_SET_HIGH:                    u32 = 29;

// GPIO_OUT_CLR
pub const SIO_GPIO_OUT_CLR_LOW:                     u32 = 0;
pub const SIO_GPIO_OUT_CLR_HIGH:                    u32 = 29;

// GPIO_OUT_XOR
pub const SIO_GPIO_OUT_XOR_LOW:                     u32 = 0;
pub const SIO_GPIO_OUT_XOR_HIGH:                    u32 = 29;

// GPIO_OE
pub const SIO_GPIO_OE_LOW:                          u32 = 0;
pub const SIO_GPIO_OE_HIGH:                         u32 = 29;

// GPIO_OE_SET
pub const SIO_GPIO_OE_SET_LOW:                      u32 = 0;
pub const SIO_GPIO_OE_SET_HIGH:                     u32 = 29;

// GPIO_OE_CLR
pub const SIO_GPIO_OE_CLR_LOW:                      u32 = 0;
pub const SIO_GPIO_OE_CLR_HIGH:                     u32 = 29;

// GPIO_OE_XOR
pub const SIO_GPIO_OE_XOR_LOW:                      u32 = 0;
pub const SIO_GPIO_OE_XOR_HIGH:                     u32 = 29;

// GPIO_HI_OUT
pub const SIO_GPIO_HI_OUT_LOW:                      u32 = 0;
pub const SIO_GPIO_HI_OUT_HIGH:                     u32 = 5;

// GPIO_HI_OUT_SET
pub const SIO_GPIO_HI_OUT_SET_LOW:                  u32 = 0;
pub const SIO_GPIO_HI_OUT_SET_HIGH:                 u32 = 5;

// GPIO_HI_OUT_CLR
pub const SIO_GPIO_HI_OUT_CLR_LOW:                  u32 = 0;
pub const SIO_GPIO_HI_OUT_CLR_HIGH:                 u32 = 5;

// GPIO_HI_OUT_XOR
pub const SIO_GPIO_HI_OUT_XOR_LOW:                  u32 = 0;
pub const SIO_GPIO_HI_OUT_XOR_HIGH:                 u32 = 5;

// GPIO_HI_OE
pub const SIO_GPIO_HI_OE_LOW:                       u32 = 0;
pub const SIO_GPIO_HI_OE_HIGH:                      u32 = 5;

// GPIO_HI_OE_SET
pub const SIO_GPIO_HI_OE_SET_LOW:                   u32 = 0;
pub const SIO_GPIO_HI_OE_SET_HIGH:                  u32 = 5;

// GPIO_HI_OE_CLR
pub const SIO_GPIO_HI_OE_CLR_LOW:                   u32 = 0;
pub const SIO_GPIO_HI_OE_CLR_HIGH:                  u32 = 5;

// GPIO_HI_OE_XOR
pub const SIO_GPIO_HI_OE_XOR_LOW:                   u32 = 0;
pub const SIO_GPIO_HI_OE_XOR_HIGH:                  u32 = 5;

// FIFO_ST
pub const SIO_FIFO_ST_ROE_BIT:                      u32 = 3;
pub const SIO_FIFO_ST_WOF_BIT:                      u32 = 2;
pub const SIO_FIFO_ST_RDY_BIT:                      u32 = 1;
pub const SIO_FIFO_ST_VLD_BIT:                      u32 = 0;

// DIV_CSR
pub const SIO_DIV_CSR_DIRTY_BIT:                    u32 = 1;
pub const SIO_DIV_CSR_READY_BIT:                    u32 = 0;

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
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
