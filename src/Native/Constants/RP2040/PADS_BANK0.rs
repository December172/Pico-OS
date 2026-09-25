#![allow(dead_code)]
// PADS_BANK0
pub const PADS_BANK0_BASE:                          u32 = 0x4001_C000;
pub const PADS_BANK0_VOLTAGE_SELECT:                u32 = PADS_BANK0_BASE + 0x0;
pub const PADS_BANK0_GPIO0:                         u32 = PADS_BANK0_BASE + 0x4;
pub const PADS_BANK0_GPIO1:                         u32 = PADS_BANK0_BASE + 0x8;
pub const PADS_BANK0_GPIO2:                         u32 = PADS_BANK0_BASE + 0xC;
pub const PADS_BANK0_GPIO3:                         u32 = PADS_BANK0_BASE + 0x10;
pub const PADS_BANK0_GPIO4:                         u32 = PADS_BANK0_BASE + 0x14;
pub const PADS_BANK0_GPIO5:                         u32 = PADS_BANK0_BASE + 0x18;
pub const PADS_BANK0_GPIO6:                         u32 = PADS_BANK0_BASE + 0x1C;
pub const PADS_BANK0_GPIO7:                         u32 = PADS_BANK0_BASE + 0x20;
pub const PADS_BANK0_GPIO8:                         u32 = PADS_BANK0_BASE + 0x24;
pub const PADS_BANK0_GPIO9:                         u32 = PADS_BANK0_BASE + 0x28;
pub const PADS_BANK0_GPIO10:                        u32 = PADS_BANK0_BASE + 0x2C;
pub const PADS_BANK0_GPIO11:                        u32 = PADS_BANK0_BASE + 0x30;
pub const PADS_BANK0_GPIO12:                        u32 = PADS_BANK0_BASE + 0x34;
pub const PADS_BANK0_GPIO13:                        u32 = PADS_BANK0_BASE + 0x38;
pub const PADS_BANK0_GPIO14:                        u32 = PADS_BANK0_BASE + 0x3C;
pub const PADS_BANK0_GPIO15:                        u32 = PADS_BANK0_BASE + 0x40;
pub const PADS_BANK0_GPIO16:                        u32 = PADS_BANK0_BASE + 0x44;
pub const PADS_BANK0_GPIO17:                        u32 = PADS_BANK0_BASE + 0x48;
pub const PADS_BANK0_GPIO18:                        u32 = PADS_BANK0_BASE + 0x4C;
pub const PADS_BANK0_GPIO19:                        u32 = PADS_BANK0_BASE + 0x50;
pub const PADS_BANK0_GPIO20:                        u32 = PADS_BANK0_BASE + 0x54;
pub const PADS_BANK0_GPIO21:                        u32 = PADS_BANK0_BASE + 0x58;
pub const PADS_BANK0_GPIO22:                        u32 = PADS_BANK0_BASE + 0x5C;
pub const PADS_BANK0_GPIO23:                        u32 = PADS_BANK0_BASE + 0x60;
pub const PADS_BANK0_GPIO24:                        u32 = PADS_BANK0_BASE + 0x64;
pub const PADS_BANK0_GPIO25:                        u32 = PADS_BANK0_BASE + 0x68;
pub const PADS_BANK0_GPIO26:                        u32 = PADS_BANK0_BASE + 0x6C;
pub const PADS_BANK0_GPIO27:                        u32 = PADS_BANK0_BASE + 0x70;
pub const PADS_BANK0_GPIO28:                        u32 = PADS_BANK0_BASE + 0x74;
pub const PADS_BANK0_GPIO29:                        u32 = PADS_BANK0_BASE + 0x78;
pub const PADS_BANK0_SWCLK:                         u32 = PADS_BANK0_BASE + 0x7C;
pub const PADS_BANK0_SWD:                           u32 = PADS_BANK0_BASE + 0x80;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

// VOLTAGE_SELECT
pub const PADS_BANK0_VOLTAGE_SELECT_BIT:            u32 = 0;
// GPIO0
pub const PADS_BANK0_GPIO0_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO0_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO0_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO0_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO0_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO0_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO0_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO0_SLEWFAST_BIT:            u32 = 0;
// GPIO1
pub const PADS_BANK0_GPIO1_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO1_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO1_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO1_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO1_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO1_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO1_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO1_SLEWFAST_BIT:            u32 = 0;
// GPIO2
pub const PADS_BANK0_GPIO2_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO2_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO2_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO2_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO2_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO2_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO2_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO2_SLEWFAST_BIT:            u32 = 0;
// GPIO3
pub const PADS_BANK0_GPIO3_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO3_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO3_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO3_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO3_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO3_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO3_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO3_SLEWFAST_BIT:            u32 = 0;
// GPIO4
pub const PADS_BANK0_GPIO4_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO4_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO4_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO4_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO4_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO4_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO4_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO4_SLEWFAST_BIT:            u32 = 0;
// GPIO5
pub const PADS_BANK0_GPIO5_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO5_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO5_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO5_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO5_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO5_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO5_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO5_SLEWFAST_BIT:            u32 = 0;
// GPIO6
pub const PADS_BANK0_GPIO6_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO6_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO6_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO6_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO6_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO6_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO6_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO6_SLEWFAST_BIT:            u32 = 0;
// GPIO7
pub const PADS_BANK0_GPIO7_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO7_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO7_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO7_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO7_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO7_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO7_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO7_SLEWFAST_BIT:            u32 = 0;
// GPIO8
pub const PADS_BANK0_GPIO8_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO8_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO8_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO8_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO8_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO8_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO8_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO8_SLEWFAST_BIT:            u32 = 0;
// GPIO9
pub const PADS_BANK0_GPIO9_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_GPIO9_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_GPIO9_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_GPIO9_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_GPIO9_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_GPIO9_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_GPIO9_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_GPIO9_SLEWFAST_BIT:            u32 = 0;
// GPIO10
pub const PADS_BANK0_GPIO10_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO10_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO10_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO10_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO10_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO10_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO10_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO10_SLEWFAST_BIT:           u32 = 0;
// GPIO11
pub const PADS_BANK0_GPIO11_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO11_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO11_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO11_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO11_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO11_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO11_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO11_SLEWFAST_BIT:           u32 = 0;
// GPIO12
pub const PADS_BANK0_GPIO12_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO12_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO12_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO12_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO12_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO12_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO12_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO12_SLEWFAST_BIT:           u32 = 0;
// GPIO13
pub const PADS_BANK0_GPIO13_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO13_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO13_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO13_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO13_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO13_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO13_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO13_SLEWFAST_BIT:           u32 = 0;
// GPIO14
pub const PADS_BANK0_GPIO14_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO14_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO14_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO14_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO14_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO14_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO14_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO14_SLEWFAST_BIT:           u32 = 0;
// GPIO15
pub const PADS_BANK0_GPIO15_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO15_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO15_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO15_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO15_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO15_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO15_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO15_SLEWFAST_BIT:           u32 = 0;
// GPIO16
pub const PADS_BANK0_GPIO16_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO16_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO16_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO16_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO16_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO16_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO16_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO16_SLEWFAST_BIT:           u32 = 0;
// GPIO17
pub const PADS_BANK0_GPIO17_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO17_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO17_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO17_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO17_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO17_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO17_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO17_SLEWFAST_BIT:           u32 = 0;
// GPIO18
pub const PADS_BANK0_GPIO18_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO18_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO18_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO18_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO18_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO18_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO18_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO18_SLEWFAST_BIT:           u32 = 0;
// GPIO19
pub const PADS_BANK0_GPIO19_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO19_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO19_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO19_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO19_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO19_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO19_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO19_SLEWFAST_BIT:           u32 = 0;
// GPIO20
pub const PADS_BANK0_GPIO20_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO20_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO20_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO20_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO20_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO20_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO20_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO20_SLEWFAST_BIT:           u32 = 0;
// GPIO21
pub const PADS_BANK0_GPIO21_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO21_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO21_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO21_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO21_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO21_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO21_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO21_SLEWFAST_BIT:           u32 = 0;
// GPIO22
pub const PADS_BANK0_GPIO22_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO22_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO22_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO22_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO22_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO22_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO22_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO22_SLEWFAST_BIT:           u32 = 0;
// GPIO23
pub const PADS_BANK0_GPIO23_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO23_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO23_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO23_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO23_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO23_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO23_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO23_SLEWFAST_BIT:           u32 = 0;
// GPIO24
pub const PADS_BANK0_GPIO24_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO24_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO24_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO24_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO24_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO24_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO24_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO24_SLEWFAST_BIT:           u32 = 0;
// GPIO25
pub const PADS_BANK0_GPIO25_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO25_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO25_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO25_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO25_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO25_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO25_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO25_SLEWFAST_BIT:           u32 = 0;
// GPIO26
pub const PADS_BANK0_GPIO26_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO26_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO26_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO26_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO26_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO26_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO26_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO26_SLEWFAST_BIT:           u32 = 0;
// GPIO27
pub const PADS_BANK0_GPIO27_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO27_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO27_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO27_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO27_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO27_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO27_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO27_SLEWFAST_BIT:           u32 = 0;
// GPIO28
pub const PADS_BANK0_GPIO28_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO28_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO28_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO28_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO28_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO28_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO28_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO28_SLEWFAST_BIT:           u32 = 0;
// GPIO29
pub const PADS_BANK0_GPIO29_OD_BIT:                 u32 = 7;
pub const PADS_BANK0_GPIO29_IE_BIT:                 u32 = 6;
pub const PADS_BANK0_GPIO29_DRIVE_LOW:              u32 = 4;
pub const PADS_BANK0_GPIO29_DRIVE_HIGH:             u32 = 5;
pub const PADS_BANK0_GPIO29_PUE_BIT:                u32 = 3;
pub const PADS_BANK0_GPIO29_PDE_BIT:                u32 = 2;
pub const PADS_BANK0_GPIO29_SCHMITT_BIT:            u32 = 1;
pub const PADS_BANK0_GPIO29_SLEWFAST_BIT:           u32 = 0;
// SWCLK
pub const PADS_BANK0_SWCLK_OD_BIT:                  u32 = 7;
pub const PADS_BANK0_SWCLK_IE_BIT:                  u32 = 6;
pub const PADS_BANK0_SWCLK_DRIVE_LOW:               u32 = 4;
pub const PADS_BANK0_SWCLK_DRIVE_HIGH:              u32 = 5;
pub const PADS_BANK0_SWCLK_PUE_BIT:                 u32 = 3;
pub const PADS_BANK0_SWCLK_PDE_BIT:                 u32 = 2;
pub const PADS_BANK0_SWCLK_SCHMITT_BIT:             u32 = 1;
pub const PADS_BANK0_SWCLK_SLEWFAST_BIT:            u32 = 0;
// SWD
pub const PADS_BANK0_SWD_OD_BIT:                    u32 = 7;
pub const PADS_BANK0_SWD_IE_BIT:                    u32 = 6;
pub const PADS_BANK0_SWD_DRIVE_LOW:                 u32 = 4;
pub const PADS_BANK0_SWD_DRIVE_HIGH:                u32 = 5;
pub const PADS_BANK0_SWD_PUE_BIT:                   u32 = 3;
pub const PADS_BANK0_SWD_PDE_BIT:                   u32 = 2;
pub const PADS_BANK0_SWD_SCHMITT_BIT:               u32 = 1;
pub const PADS_BANK0_SWD_SLEWFAST_BIT:              u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
