#![allow(dead_code)]
// PLL

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const PLL_SYS_BASE:                             u32 = 0x4005_0000;
pub const PLL_USB_BASE:                             u32 = 0x4005_8000;

pub const PLL_CS_OFFSET:                            u32 = 0x0;
pub const PLL_PWR_OFFSET:                           u32 = 0x4;
pub const PLL_FBDIV_INT_OFFSET:                     u32 = 0x8;
pub const PLL_PRIM_OFFSET:                          u32 = 0xC;
pub const PLL_INTR_OFFSET:                          u32 = 0x10;
pub const PLL_INTE_OFFSET:                          u32 = 0x14;
pub const PLL_INTF_OFFSET:                          u32 = 0x18;
pub const PLL_INTS_OFFSET:                          u32 = 0x1C;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CS
pub const PLL_CS_LOCK_BIT:                          u32 = 31;
pub const PLL_CS_LOCK_N_BIT:                        u32 = 30;
pub const PLL_CS_BYPASS_BIT:                        u32 = 8;
pub const PLL_CS_REFDIV_LOW:                        u32 = 0;
pub const PLL_CS_REFDIV_HIGH:                       u32 = 5;

// PWR
pub const PLL_PWR_VCOPD_BIT:                        u32 = 5;
pub const PLL_PWR_POSTDIVPD_BIT:                    u32 = 3;
pub const PLL_PWR_DSMPD_BIT:                        u32 = 2;
pub const PLL_PWR_PD_BIT:                           u32 = 0;

// FBDIV_INT
pub const PLL_FBDIV_INT_LOW:                        u32 = 0;
pub const PLL_FBDIV_INT_HIGH:                       u32 = 11;

// PRIM
pub const PLL_PRIM_POSTDIV1_LOW:                    u32 = 16;
pub const PLL_PRIM_POSTDIV1_HIGH:                   u32 = 18;
pub const PLL_PRIM_POSTDIV2_LOW:                    u32 = 12;
pub const PLL_PRIM_POSTDIV2_HIGH:                   u32 = 14;

// INTR
pub const PLL_INTR_LOCK_N_STICKY_BIT:               u32 = 0;

// INTE
pub const PLL_INTE_LOCK_N_STICKY_BIT:               u32 = 0;

// INTF
pub const PLL_INTF_LOCK_N_STICKY_BIT:               u32 = 0;

// INTS
pub const PLL_INTS_LOCK_N_STICKY_BIT:               u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
