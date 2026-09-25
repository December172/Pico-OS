#![allow(dead_code)]
/// SIO Peripherals & Registers
pub const SIO_BASE:               u32 = 0xD000_0000;

pub const SIO_CPUID:              u32 = SIO_BASE + 0x00;
pub const SIO_GPIO_IN:            u32 = SIO_BASE + 0x04;
pub const SIO_GPIO_HI_IN:         u32 = SIO_BASE + 0x08;
pub const SIO_GPIO_OUT:           u32 = SIO_BASE + 0x10;
pub const SIO_GPIO_OUT_SET:       u32 = SIO_BASE + 0x14;
pub const SIO_GPIO_OUT_CLR:       u32 = SIO_BASE + 0x18;
pub const SIO_GPIO_OUT_XOR:       u32 = SIO_BASE + 0x1C;
pub const SIO_GPIO_OE:            u32 = SIO_BASE + 0x20;
pub const SIO_GPIO_OE_SET:        u32 = SIO_BASE + 0x24;
pub const SIO_GPIO_OE_CLR:        u32 = SIO_BASE + 0x28;
pub const SIO_GPIO_OE_XOR:        u32 = SIO_BASE + 0x2C;
pub const SIO_GPIO_HI_OUT:        u32 = SIO_BASE + 0x30;
pub const SIO_GPIO_HI_OUT_SET:    u32 = SIO_BASE + 0x34;
pub const SIO_GPIO_HI_OUT_CLR:    u32 = SIO_BASE + 0x38;
pub const SIO_GPIO_HI_OUT_XOR:    u32 = SIO_BASE + 0x3C;
pub const SIO_GPIO_HI_OE:         u32 = SIO_BASE + 0x40;
pub const SIO_GPIO_HI_OE_SET:     u32 = SIO_BASE + 0x44;
pub const SIO_GPIO_HI_OE_CLR:     u32 = SIO_BASE + 0x48;
pub const SIO_GPIO_HI_OE_XOR:     u32 = SIO_BASE + 0x4C;

// FIFO
pub const SIO_FIFO_ST:            u32 = SIO_BASE + 0x50;
pub const SIO_FIFO_WR:            u32 = SIO_BASE + 0x54;
pub const SIO_FIFO_RD:            u32 = SIO_BASE + 0x58;

// Spinlock
pub const SIO_SPINLOCK_ST:        u32 = SIO_BASE + 0x5C;

// Divider
pub const SIO_DIV_UDIVIDEND:      u32 = SIO_BASE + 0x60;
pub const SIO_DIV_UDIVISOR:       u32 = SIO_BASE + 0x64;
pub const SIO_DIV_SDIVIDEND:      u32 = SIO_BASE + 0x68;
pub const SIO_DIV_SDIVISOR:       u32 = SIO_BASE + 0x6C;
pub const SIO_DIV_QUOTIENT:       u32 = SIO_BASE + 0x70;
pub const SIO_DIV_REMAINDER:      u32 = SIO_BASE + 0x74;
pub const SIO_DIV_CSR:            u32 = SIO_BASE + 0x78;

// Interpolator 0
pub const SIO_INTERP0_ACCUM0:     u32 = SIO_BASE + 0x80;
pub const SIO_INTERP0_ACCUM1:     u32 = SIO_BASE + 0x84;
pub const SIO_INTERP0_BASE0:      u32 = SIO_BASE + 0x88;
pub const SIO_INTERP0_BASE1:      u32 = SIO_BASE + 0x8C;
pub const SIO_INTERP0_BASE2:      u32 = SIO_BASE + 0x90;
pub const SIO_INTERP0_POP_LINE0:  u32 = SIO_BASE + 0x94;
pub const SIO_INTERP0_POP_LINE1:  u32 = SIO_BASE + 0x98;
pub const SIO_INTERP0_POP_FULL:   u32 = SIO_BASE + 0x9C;
pub const SIO_INTERP0_PEEK_LINE0: u32 = SIO_BASE + 0xA0;
pub const SIO_INTERP0_PEEK_LANE1: u32 = SIO_BASE + 0xA4;
pub const SIO_INTERP0_PEEK_FULL:  u32 = SIO_BASE + 0xA8;
pub const SIO_INTERP0_CTRL_LANE0: u32 = SIO_BASE + 0xAC;
pub const SIO_INTERP0_CTRL_LANE1: u32 = SIO_BASE + 0xB0;
pub const SIO_INTERP0_ACCUM0_ADD: u32 = SIO_BASE + 0xB4;
pub const SIO_INTERP0_ACCUM1_ADD: u32 = SIO_BASE + 0xB8;
pub const SIO_INTERP0_BASE_1AND0: u32 = SIO_BASE + 0xBC;

// Interpolator 1
pub const SIO_INTERP1_ACCUM0:     u32 = SIO_BASE + 0xC0;
pub const SIO_INTERP1_ACCUM1:     u32 = SIO_BASE + 0xC4;
pub const SIO_INTERP1_BASE0:      u32 = SIO_BASE + 0xC8;
pub const SIO_INTERP1_BASE1:      u32 = SIO_BASE + 0xCC;
pub const SIO_INTERP1_BASE2:      u32 = SIO_BASE + 0xD0;
pub const SIO_INTERP1_POP_LINE0:  u32 = SIO_BASE + 0xD4;
pub const SIO_INTERP1_POP_LINE1:  u32 = SIO_BASE + 0xD8;
pub const SIO_INTERP1_POP_FULL:   u32 = SIO_BASE + 0xDC;
pub const SIO_INTERP1_PEEK_LINE0: u32 = SIO_BASE + 0xE0;
pub const SIO_INTERP1_PEEK_LANE1: u32 = SIO_BASE + 0xE4;
pub const SIO_INTERP1_PEEK_FULL:  u32 = SIO_BASE + 0xE8;
pub const SIO_INTERP1_CTRL_LANE0: u32 = SIO_BASE + 0xEC;
pub const SIO_INTERP1_CTRL_LANE1: u32 = SIO_BASE + 0xF0;
pub const SIO_INTERP1_ACCUM0_ADD: u32 = SIO_BASE + 0xF4;
pub const SIO_INTERP1_ACCUM1_ADD: u32 = SIO_BASE + 0xF8;
pub const SIO_INTERP1_BASE_1AND0: u32 = SIO_BASE + 0xFC;

pub fn SIO_SPINLOCK(n: u32) -> u32 {
    if n > 31 {
        panic!("Spinlock number must be between 0 and 31");
    }
    const SIO_SPINLOCK_BASE: u32 = 0x100;
    return SIO_SPINLOCK_BASE + (n * 4) + SIO_BASE;
}

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CPUID
pub const SIO_CPUID_LOW:                            u32 = 0;
pub const SIO_CPUID_HIGH:                           u32 = 31;
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
// FIFO_WR
pub const SIO_FIFO_WR_LOW:                          u32 = 0;
pub const SIO_FIFO_WR_HIGH:                         u32 = 31;
// FIFO_RD
pub const SIO_FIFO_RD_LOW:                          u32 = 0;
pub const SIO_FIFO_RD_HIGH:                         u32 = 31;
// SPINLOCK_ST
pub const SIO_SPINLOCK_ST_LOW:                      u32 = 0;
pub const SIO_SPINLOCK_ST_HIGH:                     u32 = 31;
// DIV_UDIVIDEND
pub const SIO_DIV_UDIVIDEND_LOW:                    u32 = 0;
pub const SIO_DIV_UDIVIDEND_HIGH:                   u32 = 31;
// DIV_UDIVISOR
pub const SIO_DIV_UDIVISOR_LOW:                     u32 = 0;
pub const SIO_DIV_UDIVISOR_HIGH:                    u32 = 31;
// DIV_SDIVIDEND
pub const SIO_DIV_SDIVIDEND_LOW:                    u32 = 0;
pub const SIO_DIV_SDIVIDEND_HIGH:                   u32 = 31;
// DIV_SDIVISOR
pub const SIO_DIV_SDIVISOR_LOW:                     u32 = 0;
pub const SIO_DIV_SDIVISOR_HIGH:                    u32 = 31;
// DIV_QUOTIENT
pub const SIO_DIV_QUOTIENT_LOW:                     u32 = 0;
pub const SIO_DIV_QUOTIENT_HIGH:                    u32 = 31;
// DIV_REMAINDER
pub const SIO_DIV_REMAINDER_LOW:                    u32 = 0;
pub const SIO_DIV_REMAINDER_HIGH:                   u32 = 31;
// DIV_CSR
pub const SIO_DIV_CSR_DIRTY_BIT:                    u32 = 1;
pub const SIO_DIV_CSR_READY_BIT:                    u32 = 0;
// INTERP0_ACCUM0
pub const SIO_INTERP0_ACCUM0_LOW:                   u32 = 0;
pub const SIO_INTERP0_ACCUM0_HIGH:                  u32 = 31;
// INTERP0_ACCUM1
pub const SIO_INTERP0_ACCUM1_LOW:                   u32 = 0;
pub const SIO_INTERP0_ACCUM1_HIGH:                  u32 = 31;
// INTERP0_BASE0
pub const SIO_INTERP0_BASE0_LOW:                    u32 = 0;
pub const SIO_INTERP0_BASE0_HIGH:                   u32 = 31;
// INTERP0_BASE1
pub const SIO_INTERP0_BASE1_LOW:                    u32 = 0;
pub const SIO_INTERP0_BASE1_HIGH:                   u32 = 31;
// INTERP0_BASE2
pub const SIO_INTERP0_BASE2_LOW:                    u32 = 0;
pub const SIO_INTERP0_BASE2_HIGH:                   u32 = 31;
// INTERP0_POP_LANE0
pub const SIO_INTERP0_POP_LANE0_LOW:                u32 = 0;
pub const SIO_INTERP0_POP_LANE0_HIGH:               u32 = 31;
// INTERP0_POP_LANE1
pub const SIO_INTERP0_POP_LANE1_LOW:                u32 = 0;
pub const SIO_INTERP0_POP_LANE1_HIGH:               u32 = 31;
// INTERP0_POP_FULL
pub const SIO_INTERP0_POP_FULL_LOW:                 u32 = 0;
pub const SIO_INTERP0_POP_FULL_HIGH:                u32 = 31;
// INTERP0_PEEK_LANE0
pub const SIO_INTERP0_PEEK_LANE0_LOW:               u32 = 0;
pub const SIO_INTERP0_PEEK_LANE0_HIGH:              u32 = 31;
// INTERP0_PEEK_LANE1
pub const SIO_INTERP0_PEEK_LANE1_LOW:               u32 = 0;
pub const SIO_INTERP0_PEEK_LANE1_HIGH:              u32 = 31;
// INTERP0_PEEK_FULL
pub const SIO_INTERP0_PEEK_FULL_LOW:                u32 = 0;
pub const SIO_INTERP0_PEEK_FULL_HIGH:               u32 = 31;
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
// INTERP0_CTRL_LANE1
pub const SIO_INTERP0_CTRL_LANE1_FORCE_MSB_LOW:     u32 = 19;
pub const SIO_INTERP0_CTRL_LANE1_FORCE_MSB_HIGH:    u32 = 20;
pub const SIO_INTERP0_CTRL_LANE1_ADD_RAW_BIT:       u32 = 18;
pub const SIO_INTERP0_CTRL_LANE1_CROSS_RESULT_BIT:  u32 = 17;
pub const SIO_INTERP0_CTRL_LANE1_CROSS_INPUT_BIT:   u32 = 16;
pub const SIO_INTERP0_CTRL_LANE1_SIGNED_BIT:        u32 = 15;
pub const SIO_INTERP0_CTRL_LANE1_MASK_MSB_LOW:      u32 = 10;
pub const SIO_INTERP0_CTRL_LANE1_MASK_MSB_HIGH:     u32 = 14;
pub const SIO_INTERP0_CTRL_LANE1_MASK_LSB_LOW:      u32 = 5;
pub const SIO_INTERP0_CTRL_LANE1_MASK_LSB_HIGH:     u32 = 9;
pub const SIO_INTERP0_CTRL_LANE1_SHIFT_LOW:         u32 = 0;
pub const SIO_INTERP0_CTRL_LANE1_SHIFT_HIGH:        u32 = 4;
// INTERP0_ACCUM0_ADD
pub const SIO_INTERP0_ACCUM0_ADD_LOW:               u32 = 0;
pub const SIO_INTERP0_ACCUM0_ADD_HIGH:              u32 = 23;
// INTERP0_ACCUM1_ADD
pub const SIO_INTERP0_ACCUM1_ADD_LOW:               u32 = 0;
pub const SIO_INTERP0_ACCUM1_ADD_HIGH:              u32 = 23;
// INTERP0_BASE_1AND0
pub const SIO_INTERP0_BASE_1AND0_LOW:               u32 = 0;
pub const SIO_INTERP0_BASE_1AND0_HIGH:              u32 = 31;
// INTERP1_ACCUM0
pub const SIO_INTERP1_ACCUM0_LOW:                   u32 = 0;
pub const SIO_INTERP1_ACCUM0_HIGH:                  u32 = 31;
// INTERP1_ACCUM1
pub const SIO_INTERP1_ACCUM1_LOW:                   u32 = 0;
pub const SIO_INTERP1_ACCUM1_HIGH:                  u32 = 31;
// INTERP1_BASE0
pub const SIO_INTERP1_BASE0_LOW:                    u32 = 0;
pub const SIO_INTERP1_BASE0_HIGH:                   u32 = 31;
// INTERP1_BASE1
pub const SIO_INTERP1_BASE1_LOW:                    u32 = 0;
pub const SIO_INTERP1_BASE1_HIGH:                   u32 = 31;
// INTERP1_BASE2
pub const SIO_INTERP1_BASE2_LOW:                    u32 = 0;
pub const SIO_INTERP1_BASE2_HIGH:                   u32 = 31;
// INTERP1_POP_LANE0
pub const SIO_INTERP1_POP_LANE0_LOW:                u32 = 0;
pub const SIO_INTERP1_POP_LANE0_HIGH:               u32 = 31;
// INTERP1_POP_LANE1
pub const SIO_INTERP1_POP_LANE1_LOW:                u32 = 0;
pub const SIO_INTERP1_POP_LANE1_HIGH:               u32 = 31;
// INTERP1_POP_FULL
pub const SIO_INTERP1_POP_FULL_LOW:                 u32 = 0;
pub const SIO_INTERP1_POP_FULL_HIGH:                u32 = 31;
// INTERP1_PEEK_LANE0
pub const SIO_INTERP1_PEEK_LANE0_LOW:               u32 = 0;
pub const SIO_INTERP1_PEEK_LANE0_HIGH:              u32 = 31;
// INTERP1_PEEK_LANE1
pub const SIO_INTERP1_PEEK_LANE1_LOW:               u32 = 0;
pub const SIO_INTERP1_PEEK_LANE1_HIGH:              u32 = 31;
// INTERP1_PEEK_FULL
pub const SIO_INTERP1_PEEK_FULL_LOW:                u32 = 0;
pub const SIO_INTERP1_PEEK_FULL_HIGH:               u32 = 31;
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
// INTERP1_CTRL_LANE1
pub const SIO_INTERP1_CTRL_LANE1_FORCE_MSB_LOW:     u32 = 19;
pub const SIO_INTERP1_CTRL_LANE1_FORCE_MSB_HIGH:    u32 = 20;
pub const SIO_INTERP1_CTRL_LANE1_ADD_RAW_BIT:       u32 = 18;
pub const SIO_INTERP1_CTRL_LANE1_CROSS_RESULT_BIT:  u32 = 17;
pub const SIO_INTERP1_CTRL_LANE1_CROSS_INPUT_BIT:   u32 = 16;
pub const SIO_INTERP1_CTRL_LANE1_SIGNED_BIT:        u32 = 15;
pub const SIO_INTERP1_CTRL_LANE1_MASK_MSB_LOW:      u32 = 10;
pub const SIO_INTERP1_CTRL_LANE1_MASK_MSB_HIGH:     u32 = 14;
pub const SIO_INTERP1_CTRL_LANE1_MASK_LSB_LOW:      u32 = 5;
pub const SIO_INTERP1_CTRL_LANE1_MASK_LSB_HIGH:     u32 = 9;
pub const SIO_INTERP1_CTRL_LANE1_SHIFT_LOW:         u32 = 0;
pub const SIO_INTERP1_CTRL_LANE1_SHIFT_HIGH:        u32 = 4;
// INTERP1_ACCUM0_ADD
pub const SIO_INTERP1_ACCUM0_ADD_LOW:               u32 = 0;
pub const SIO_INTERP1_ACCUM0_ADD_HIGH:              u32 = 23;
// INTERP1_ACCUM1_ADD
pub const SIO_INTERP1_ACCUM1_ADD_LOW:               u32 = 0;
pub const SIO_INTERP1_ACCUM1_ADD_HIGH:              u32 = 23;
// INTERP1_BASE_1AND0
pub const SIO_INTERP1_BASE_1AND0_LOW:               u32 = 0;
pub const SIO_INTERP1_BASE_1AND0_HIGH:              u32 = 31;
// SPINLOCK0
pub const SIO_SPINLOCK0_LOW:                        u32 = 0;
pub const SIO_SPINLOCK0_HIGH:                       u32 = 31;
// SPINLOCK1
pub const SIO_SPINLOCK1_LOW:                        u32 = 0;
pub const SIO_SPINLOCK1_HIGH:                       u32 = 31;
// SPINLOCK2
pub const SIO_SPINLOCK2_LOW:                        u32 = 0;
pub const SIO_SPINLOCK2_HIGH:                       u32 = 31;
// SPINLOCK3
pub const SIO_SPINLOCK3_LOW:                        u32 = 0;
pub const SIO_SPINLOCK3_HIGH:                       u32 = 31;
// SPINLOCK4
pub const SIO_SPINLOCK4_LOW:                        u32 = 0;
pub const SIO_SPINLOCK4_HIGH:                       u32 = 31;
// SPINLOCK5
pub const SIO_SPINLOCK5_LOW:                        u32 = 0;
pub const SIO_SPINLOCK5_HIGH:                       u32 = 31;
// SPINLOCK6
pub const SIO_SPINLOCK6_LOW:                        u32 = 0;
pub const SIO_SPINLOCK6_HIGH:                       u32 = 31;
// SPINLOCK7
pub const SIO_SPINLOCK7_LOW:                        u32 = 0;
pub const SIO_SPINLOCK7_HIGH:                       u32 = 31;
// SPINLOCK8
pub const SIO_SPINLOCK8_LOW:                        u32 = 0;
pub const SIO_SPINLOCK8_HIGH:                       u32 = 31;
// SPINLOCK9
pub const SIO_SPINLOCK9_LOW:                        u32 = 0;
pub const SIO_SPINLOCK9_HIGH:                       u32 = 31;
// SPINLOCK10
pub const SIO_SPINLOCK10_LOW:                       u32 = 0;
pub const SIO_SPINLOCK10_HIGH:                      u32 = 31;
// SPINLOCK11
pub const SIO_SPINLOCK11_LOW:                       u32 = 0;
pub const SIO_SPINLOCK11_HIGH:                      u32 = 31;
// SPINLOCK12
pub const SIO_SPINLOCK12_LOW:                       u32 = 0;
pub const SIO_SPINLOCK12_HIGH:                      u32 = 31;
// SPINLOCK13
pub const SIO_SPINLOCK13_LOW:                       u32 = 0;
pub const SIO_SPINLOCK13_HIGH:                      u32 = 31;
// SPINLOCK14
pub const SIO_SPINLOCK14_LOW:                       u32 = 0;
pub const SIO_SPINLOCK14_HIGH:                      u32 = 31;
// SPINLOCK15
pub const SIO_SPINLOCK15_LOW:                       u32 = 0;
pub const SIO_SPINLOCK15_HIGH:                      u32 = 31;
// SPINLOCK16
pub const SIO_SPINLOCK16_LOW:                       u32 = 0;
pub const SIO_SPINLOCK16_HIGH:                      u32 = 31;
// SPINLOCK17
pub const SIO_SPINLOCK17_LOW:                       u32 = 0;
pub const SIO_SPINLOCK17_HIGH:                      u32 = 31;
// SPINLOCK18
pub const SIO_SPINLOCK18_LOW:                       u32 = 0;
pub const SIO_SPINLOCK18_HIGH:                      u32 = 31;
// SPINLOCK19
pub const SIO_SPINLOCK19_LOW:                       u32 = 0;
pub const SIO_SPINLOCK19_HIGH:                      u32 = 31;
// SPINLOCK20
pub const SIO_SPINLOCK20_LOW:                       u32 = 0;
pub const SIO_SPINLOCK20_HIGH:                      u32 = 31;
// SPINLOCK21
pub const SIO_SPINLOCK21_LOW:                       u32 = 0;
pub const SIO_SPINLOCK21_HIGH:                      u32 = 31;
// SPINLOCK22
pub const SIO_SPINLOCK22_LOW:                       u32 = 0;
pub const SIO_SPINLOCK22_HIGH:                      u32 = 31;
// SPINLOCK23
pub const SIO_SPINLOCK23_LOW:                       u32 = 0;
pub const SIO_SPINLOCK23_HIGH:                      u32 = 31;
// SPINLOCK24
pub const SIO_SPINLOCK24_LOW:                       u32 = 0;
pub const SIO_SPINLOCK24_HIGH:                      u32 = 31;
// SPINLOCK25
pub const SIO_SPINLOCK25_LOW:                       u32 = 0;
pub const SIO_SPINLOCK25_HIGH:                      u32 = 31;
// SPINLOCK26
pub const SIO_SPINLOCK26_LOW:                       u32 = 0;
pub const SIO_SPINLOCK26_HIGH:                      u32 = 31;
// SPINLOCK27
pub const SIO_SPINLOCK27_LOW:                       u32 = 0;
pub const SIO_SPINLOCK27_HIGH:                      u32 = 31;
// SPINLOCK28
pub const SIO_SPINLOCK28_LOW:                       u32 = 0;
pub const SIO_SPINLOCK28_HIGH:                      u32 = 31;
// SPINLOCK29
pub const SIO_SPINLOCK29_LOW:                       u32 = 0;
pub const SIO_SPINLOCK29_HIGH:                      u32 = 31;
// SPINLOCK30
pub const SIO_SPINLOCK30_LOW:                       u32 = 0;
pub const SIO_SPINLOCK30_HIGH:                      u32 = 31;
// SPINLOCK31
pub const SIO_SPINLOCK31_LOW:                       u32 = 0;
pub const SIO_SPINLOCK31_HIGH:                      u32 = 31;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
