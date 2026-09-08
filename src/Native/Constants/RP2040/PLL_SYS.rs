#![allow(dead_code)]
// PLL_SYS
pub const PLL_SYS_BASE:                             u32 = 0x4002_8000;
pub const PLL_SYS_CS:                               u32 = PLL_SYS_BASE + 0x0;
pub const PLL_SYS_PWR:                              u32 = PLL_SYS_BASE + 0x4;
pub const PLL_SYS_FBDIV_INT:                        u32 = PLL_SYS_BASE + 0x8;
pub const PLL_SYS_PRIM:                             u32 = PLL_SYS_BASE + 0xC;
