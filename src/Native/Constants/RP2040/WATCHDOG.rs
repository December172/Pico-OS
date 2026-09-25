#![allow(dead_code)]
// WATCHDOG
pub const WATCHDOG_BASE:                            u32 = 0x4005_8000;
pub const WATCHDOG_CTRL:                            u32 = WATCHDOG_BASE + 0x0;
pub const WATCHDOG_LOAD:                            u32 = WATCHDOG_BASE + 0x4;
pub const WATCHDOG_REASON:                          u32 = WATCHDOG_BASE + 0x8;
pub const WATCHDOG_SCRATCH0:                        u32 = WATCHDOG_BASE + 0xC;
pub const WATCHDOG_SCRATCH1:                        u32 = WATCHDOG_BASE + 0x10;
pub const WATCHDOG_SCRATCH2:                        u32 = WATCHDOG_BASE + 0x14;
pub const WATCHDOG_SCRATCH3:                        u32 = WATCHDOG_BASE + 0x18;
pub const WATCHDOG_SCRATCH4:                        u32 = WATCHDOG_BASE + 0x1C;
pub const WATCHDOG_SCRATCH5:                        u32 = WATCHDOG_BASE + 0x20;
pub const WATCHDOG_SCRATCH6:                        u32 = WATCHDOG_BASE + 0x24;
pub const WATCHDOG_SCRATCH7:                        u32 = WATCHDOG_BASE + 0x28;
pub const WATCHDOG_TICK:                            u32 = WATCHDOG_BASE + 0x2C;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// CTRL
pub const WATCHDOG_CTRL_TRIGGER_BIT:                u32 = 31;
pub const WATCHDOG_CTRL_ENABLE_BIT:                 u32 = 30;
pub const WATCHDOG_CTRL_PAUSE_DBG1_BIT:             u32 = 26;
pub const WATCHDOG_CTRL_PAUSE_DBG0_BIT:             u32 = 25;
pub const WATCHDOG_CTRL_PAUSE_JTAG_BIT:             u32 = 24;
pub const WATCHDOG_CTRL_TIME_LOW:                   u32 = 0;
pub const WATCHDOG_CTRL_TIME_HIGH:                  u32 = 23;
// LOAD
pub const WATCHDOG_LOAD_LOW:                        u32 = 0;
pub const WATCHDOG_LOAD_HIGH:                       u32 = 23;
// REASON
pub const WATCHDOG_REASON_FORCE_BIT:                u32 = 1;
pub const WATCHDOG_REASON_TIMER_BIT:                u32 = 0;
// SCRATCH0
pub const WATCHDOG_SCRATCH0_LOW:                    u32 = 0;
pub const WATCHDOG_SCRATCH0_HIGH:                   u32 = 31;
// SCRATCH1
pub const WATCHDOG_SCRATCH1_LOW:                    u32 = 0;
pub const WATCHDOG_SCRATCH1_HIGH:                   u32 = 31;
// SCRATCH2
pub const WATCHDOG_SCRATCH2_LOW:                    u32 = 0;
pub const WATCHDOG_SCRATCH2_HIGH:                   u32 = 31;
// SCRATCH3
pub const WATCHDOG_SCRATCH3_LOW:                    u32 = 0;
pub const WATCHDOG_SCRATCH3_HIGH:                   u32 = 31;
// SCRATCH4
pub const WATCHDOG_SCRATCH4_LOW:                    u32 = 0;
pub const WATCHDOG_SCRATCH4_HIGH:                   u32 = 31;
// SCRATCH5
pub const WATCHDOG_SCRATCH5_LOW:                    u32 = 0;
pub const WATCHDOG_SCRATCH5_HIGH:                   u32 = 31;
// SCRATCH6
pub const WATCHDOG_SCRATCH6_LOW:                    u32 = 0;
pub const WATCHDOG_SCRATCH6_HIGH:                   u32 = 31;
// SCRATCH7
pub const WATCHDOG_SCRATCH7_LOW:                    u32 = 0;
pub const WATCHDOG_SCRATCH7_HIGH:                   u32 = 31;
// TICK
pub const WATCHDOG_TICK_COUNT_LOW:                  u32 = 11;
pub const WATCHDOG_TICK_COUNT_HIGH:                 u32 = 19;
pub const WATCHDOG_TICK_RUNNING_BIT:                u32 = 10;
pub const WATCHDOG_TICK_ENABLE_BIT:                 u32 = 9;
pub const WATCHDOG_TICK_CYCLES_LOW:                 u32 = 0;
pub const WATCHDOG_TICK_CYCLES_HIGH:                u32 = 8;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
