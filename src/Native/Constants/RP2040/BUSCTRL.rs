#![allow(dead_code)]
// BUSCTRL
pub const BUSCTRL_BASE:                             u32 = 0x4003_0000;
pub const BUSCTRL_BUS_PRIORITY:                     u32 = BUSCTRL_BASE + 0x0;
pub const BUSCTRL_BUS_PRIORITY_ACK:                 u32 = BUSCTRL_BASE + 0x4;
pub const BUSCTRL_PERFCTR0:                         u32 = BUSCTRL_BASE + 0x8;
pub const BUSCTRL_PERFSEL0:                         u32 = BUSCTRL_BASE + 0xC;
pub const BUSCTRL_PERFCTR1:                         u32 = BUSCTRL_BASE + 0x10;
pub const BUSCTRL_PERFSEL1:                         u32 = BUSCTRL_BASE + 0x14;
pub const BUSCTRL_PERFCTR2:                         u32 = BUSCTRL_BASE + 0x18;
pub const BUSCTRL_PERFSEL2:                         u32 = BUSCTRL_BASE + 0x1C;
pub const BUSCTRL_PERFCTR3:                         u32 = BUSCTRL_BASE + 0x20;
pub const BUSCTRL_PERFSEL3:                         u32 = BUSCTRL_BASE + 0x24;
