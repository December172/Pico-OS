#![allow(dead_code)]
// TICKS

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const TICKS_BASE:                               u32 = 0x4010_8000;

// PROC0_CTRL..PROC1_CTRL
pub fn TICKS_PROC_CTRL(n: u32) -> u32 {
    return TICKS_BASE + 0x0 + n * 0xC
}

// PROC0_CYCLES..PROC1_CYCLES
pub fn TICKS_PROC_CYCLES(n: u32) -> u32 {
    return TICKS_BASE + 0x4 + n * 0xC
}

// PROC0_COUNT..PROC1_COUNT
pub fn TICKS_PROC_COUNT(n: u32) -> u32 {
    return TICKS_BASE + 0x8 + n * 0xC
}

// TIMER0_CTRL..TIMER1_CTRL
pub fn TICKS_TIMER_CTRL(n: u32) -> u32 {
    return TICKS_BASE + 0x18 + n * 0xC
}

// TIMER0_CYCLES..TIMER1_CYCLES
pub fn TICKS_TIMER_CYCLES(n: u32) -> u32 {
    return TICKS_BASE + 0x1C + n * 0xC
}

// TIMER0_COUNT..TIMER1_COUNT
pub fn TICKS_TIMER_COUNT(n: u32) -> u32 {
    return TICKS_BASE + 0x20 + n * 0xC
}

pub const TICKS_WATCHDOG_CTRL:                      u32 = TICKS_BASE + 0x30;
pub const TICKS_WATCHDOG_CYCLES:                    u32 = TICKS_BASE + 0x34;
pub const TICKS_WATCHDOG_COUNT:                     u32 = TICKS_BASE + 0x38;
pub const TICKS_RISCV_CTRL:                         u32 = TICKS_BASE + 0x3C;
pub const TICKS_RISCV_CYCLES:                       u32 = TICKS_BASE + 0x40;
pub const TICKS_RISCV_COUNT:                        u32 = TICKS_BASE + 0x44;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// PROC0_CTRL, PROC1_CTRL, TIMER0_CTRL, TIMER1_CTRL, WATCHDOG_CTRL, RISCV_CTRL
pub const TICKS_CTRL_RUNNING_BIT:                   u32 = 1;
pub const TICKS_CTRL_ENABLE_BIT:                    u32 = 0;

// PROC0_CYCLES, PROC1_CYCLES
pub const TICKS_PROC_CYCLES_LOW:                    u32 = 0;
pub const TICKS_PROC_CYCLES_HIGH:                   u32 = 8;

// PROC0_COUNT, PROC1_COUNT
pub const TICKS_PROC_COUNT_LOW:                     u32 = 0;
pub const TICKS_PROC_COUNT_HIGH:                    u32 = 8;

// TIMER0_CYCLES, TIMER1_CYCLES
pub const TICKS_TIMER_CYCLES_LOW:                   u32 = 0;
pub const TICKS_TIMER_CYCLES_HIGH:                  u32 = 8;

// TIMER0_COUNT, TIMER1_COUNT
pub const TICKS_TIMER_COUNT_LOW:                    u32 = 0;
pub const TICKS_TIMER_COUNT_HIGH:                   u32 = 8;

// WATCHDOG_CYCLES
pub const TICKS_WATCHDOG_CYCLES_LOW:                u32 = 0;
pub const TICKS_WATCHDOG_CYCLES_HIGH:               u32 = 8;

// WATCHDOG_COUNT
pub const TICKS_WATCHDOG_COUNT_LOW:                 u32 = 0;
pub const TICKS_WATCHDOG_COUNT_HIGH:                u32 = 8;

// RISCV_CYCLES
pub const TICKS_RISCV_CYCLES_LOW:                   u32 = 0;
pub const TICKS_RISCV_CYCLES_HIGH:                  u32 = 8;

// RISCV_COUNT
pub const TICKS_RISCV_COUNT_LOW:                    u32 = 0;
pub const TICKS_RISCV_COUNT_HIGH:                   u32 = 8;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
