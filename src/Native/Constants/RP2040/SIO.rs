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

pub const fn sioSpinlock(n: u32) -> u32 {
    if n > 31 {
        panic!("Spinlock number must be between 0 and 31");
    }
    const SIO_SPINLOCK_BASE: u32 = 0x100;
    return SIO_SPINLOCK_BASE + (n * 4) + SIO_BASE;
}