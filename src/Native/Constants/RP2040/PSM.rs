#![allow(dead_code)]
// PSM
pub const PSM_BASE:                                 u32 = 0x4001_0000;
pub const PSM_FRCE_ON:                              u32 = PSM_BASE + 0x0;
pub const PSM_FRCE_OFF:                             u32 = PSM_BASE + 0x4;
pub const PSM_WDSEL:                                u32 = PSM_BASE + 0x8;
pub const PSM_DONE:                                 u32 = PSM_BASE + 0xC;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// FRCE_ON
pub const PSM_FRCE_ON_PROC1_BIT:                    u32 = 16;
pub const PSM_FRCE_ON_PROC0_BIT:                    u32 = 15;
pub const PSM_FRCE_ON_SIO_BIT:                      u32 = 14;
pub const PSM_FRCE_ON_VREG_AND_CHIP_RESET_BIT:      u32 = 13;
pub const PSM_FRCE_ON_XIP_BIT:                      u32 = 12;
pub const PSM_FRCE_ON_SRAM5_BIT:                    u32 = 11;
pub const PSM_FRCE_ON_SRAM4_BIT:                    u32 = 10;
pub const PSM_FRCE_ON_SRAM3_BIT:                    u32 = 9;
pub const PSM_FRCE_ON_SRAM2_BIT:                    u32 = 8;
pub const PSM_FRCE_ON_SRAM1_BIT:                    u32 = 7;
pub const PSM_FRCE_ON_SRAM0_BIT:                    u32 = 6;
pub const PSM_FRCE_ON_ROM_BIT:                      u32 = 5;
pub const PSM_FRCE_ON_BUSFABRIC_BIT:                u32 = 4;
pub const PSM_FRCE_ON_RESETS_BIT:                   u32 = 3;
pub const PSM_FRCE_ON_CLOCKS_BIT:                   u32 = 2;
pub const PSM_FRCE_ON_XOSC_BIT:                     u32 = 1;
pub const PSM_FRCE_ON_ROSC_BIT:                     u32 = 0;
// FRCE_OFF
pub const PSM_FRCE_OFF_PROC1_BIT:                   u32 = 16;
pub const PSM_FRCE_OFF_PROC0_BIT:                   u32 = 15;
pub const PSM_FRCE_OFF_SIO_BIT:                     u32 = 14;
pub const PSM_FRCE_OFF_VREG_AND_CHIP_RESET_BIT:     u32 = 13;
pub const PSM_FRCE_OFF_XIP_BIT:                     u32 = 12;
pub const PSM_FRCE_OFF_SRAM5_BIT:                   u32 = 11;
pub const PSM_FRCE_OFF_SRAM4_BIT:                   u32 = 10;
pub const PSM_FRCE_OFF_SRAM3_BIT:                   u32 = 9;
pub const PSM_FRCE_OFF_SRAM2_BIT:                   u32 = 8;
pub const PSM_FRCE_OFF_SRAM1_BIT:                   u32 = 7;
pub const PSM_FRCE_OFF_SRAM0_BIT:                   u32 = 6;
pub const PSM_FRCE_OFF_ROM_BIT:                     u32 = 5;
pub const PSM_FRCE_OFF_BUSFABRIC_BIT:               u32 = 4;
pub const PSM_FRCE_OFF_RESETS_BIT:                  u32 = 3;
pub const PSM_FRCE_OFF_CLOCKS_BIT:                  u32 = 2;
pub const PSM_FRCE_OFF_XOSC_BIT:                    u32 = 1;
pub const PSM_FRCE_OFF_ROSC_BIT:                    u32 = 0;
// WDSEL
pub const PSM_WDSEL_PROC1_BIT:                      u32 = 16;
pub const PSM_WDSEL_PROC0_BIT:                      u32 = 15;
pub const PSM_WDSEL_SIO_BIT:                        u32 = 14;
pub const PSM_WDSEL_VREG_AND_CHIP_RESET_BIT:        u32 = 13;
pub const PSM_WDSEL_XIP_BIT:                        u32 = 12;
pub const PSM_WDSEL_SRAM5_BIT:                      u32 = 11;
pub const PSM_WDSEL_SRAM4_BIT:                      u32 = 10;
pub const PSM_WDSEL_SRAM3_BIT:                      u32 = 9;
pub const PSM_WDSEL_SRAM2_BIT:                      u32 = 8;
pub const PSM_WDSEL_SRAM1_BIT:                      u32 = 7;
pub const PSM_WDSEL_SRAM0_BIT:                      u32 = 6;
pub const PSM_WDSEL_ROM_BIT:                        u32 = 5;
pub const PSM_WDSEL_BUSFABRIC_BIT:                  u32 = 4;
pub const PSM_WDSEL_RESETS_BIT:                     u32 = 3;
pub const PSM_WDSEL_CLOCKS_BIT:                     u32 = 2;
pub const PSM_WDSEL_XOSC_BIT:                       u32 = 1;
pub const PSM_WDSEL_ROSC_BIT:                       u32 = 0;
// DONE
pub const PSM_DONE_PROC1_BIT:                       u32 = 16;
pub const PSM_DONE_PROC0_BIT:                       u32 = 15;
pub const PSM_DONE_SIO_BIT:                         u32 = 14;
pub const PSM_DONE_VREG_AND_CHIP_RESET_BIT:         u32 = 13;
pub const PSM_DONE_XIP_BIT:                         u32 = 12;
pub const PSM_DONE_SRAM5_BIT:                       u32 = 11;
pub const PSM_DONE_SRAM4_BIT:                       u32 = 10;
pub const PSM_DONE_SRAM3_BIT:                       u32 = 9;
pub const PSM_DONE_SRAM2_BIT:                       u32 = 8;
pub const PSM_DONE_SRAM1_BIT:                       u32 = 7;
pub const PSM_DONE_SRAM0_BIT:                       u32 = 6;
pub const PSM_DONE_ROM_BIT:                         u32 = 5;
pub const PSM_DONE_BUSFABRIC_BIT:                   u32 = 4;
pub const PSM_DONE_RESETS_BIT:                      u32 = 3;
pub const PSM_DONE_CLOCKS_BIT:                      u32 = 2;
pub const PSM_DONE_XOSC_BIT:                        u32 = 1;
pub const PSM_DONE_ROSC_BIT:                        u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
