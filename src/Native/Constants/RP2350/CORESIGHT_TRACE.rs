#![allow(dead_code)]
// CORESIGHT_TRACE

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const CORESIGHT_TRACE_BASE:                     u32 = 0x5070_0000;

pub const CORESIGHT_TRACE_CTRL_STATUS:              u32 = CORESIGHT_TRACE_BASE + 0x0;
pub const CORESIGHT_TRACE_TRACE_CAPTURE_FIFO:       u32 = CORESIGHT_TRACE_BASE + 0x4;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CTRL_STATUS
pub const CORESIGHT_TRACE_CTRL_STATUS_TRACE_CAPTURE_FIFO_OVERFLOW_BIT:u32 = 1;
pub const CORESIGHT_TRACE_CTRL_STATUS_TRACE_CAPTURE_FIFO_FLUSH_BIT:u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
