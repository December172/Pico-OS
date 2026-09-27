#![allow(dead_code)]
// SHA256

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const SHA256_BASE:                              u32 = 0x400F_8000;

pub const SHA256_CSR:                               u32 = SHA256_BASE + 0x0;
pub const SHA256_WDATA:                             u32 = SHA256_BASE + 0x4;
// SUM0..SUM7
pub fn SHA256_SUM(n: u32) -> u32 {
    return SHA256_BASE + 0x8 + n * 0x4
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CSR
pub const SHA256_CSR_BSWAP_BIT:                     u32 = 12;
pub const SHA256_CSR_DMA_SIZE_LOW:                  u32 = 8;
pub const SHA256_CSR_DMA_SIZE_HIGH:                 u32 = 9;
pub const SHA256_CSR_ERR_WDATA_NOT_RDY_BIT:         u32 = 4;
pub const SHA256_CSR_SUM_VLD_BIT:                   u32 = 2;
pub const SHA256_CSR_WDATA_RDY_BIT:                 u32 = 1;
pub const SHA256_CSR_START_BIT:                     u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CSR: DMA_SIZE
pub const SHA256_CSR_DMA_SIZE_8BIT:                 u32 = 0x0;
pub const SHA256_CSR_DMA_SIZE_16BIT:                u32 = 0x1;
pub const SHA256_CSR_DMA_SIZE_32BIT:                u32 = 0x2;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
