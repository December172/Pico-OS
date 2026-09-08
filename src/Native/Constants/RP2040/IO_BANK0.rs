#![allow(dead_code)]
// IO_BANK0
pub const IO_BANK0_BASE:                            u32 = 0x4001_4000;

pub const IO_BANK0_INTR0:                           u32 = IO_BANK0_BASE + 0xF0;
pub const IO_BANK0_INTR1:                           u32 = IO_BANK0_BASE + 0xF4;
pub const IO_BANK0_INTR2:                           u32 = IO_BANK0_BASE + 0xF8;
pub const IO_BANK0_INTR3:                           u32 = IO_BANK0_BASE + 0xFC;
pub const IO_BANK0_PROC0_INTE0:                     u32 = IO_BANK0_BASE + 0x100;
pub const IO_BANK0_PROC0_INTE1:                     u32 = IO_BANK0_BASE + 0x104;
pub const IO_BANK0_PROC0_INTE2:                     u32 = IO_BANK0_BASE + 0x108;
pub const IO_BANK0_PROC0_INTE3:                     u32 = IO_BANK0_BASE + 0x10C;
pub const IO_BANK0_PROC0_INTF0:                     u32 = IO_BANK0_BASE + 0x110;
pub const IO_BANK0_PROC0_INTF1:                     u32 = IO_BANK0_BASE + 0x114;
pub const IO_BANK0_PROC0_INTF2:                     u32 = IO_BANK0_BASE + 0x118;
pub const IO_BANK0_PROC0_INTF3:                     u32 = IO_BANK0_BASE + 0x11C;
pub const IO_BANK0_PROC0_INTS0:                     u32 = IO_BANK0_BASE + 0x120;
pub const IO_BANK0_PROC0_INTS1:                     u32 = IO_BANK0_BASE + 0x124;
pub const IO_BANK0_PROC0_INTS2:                     u32 = IO_BANK0_BASE + 0x128;
pub const IO_BANK0_PROC0_INTS3:                     u32 = IO_BANK0_BASE + 0x12C;
pub const IO_BANK0_PROC1_INTE0:                     u32 = IO_BANK0_BASE + 0x130;
pub const IO_BANK0_PROC1_INTE1:                     u32 = IO_BANK0_BASE + 0x134;
pub const IO_BANK0_PROC1_INTE2:                     u32 = IO_BANK0_BASE + 0x138;
pub const IO_BANK0_PROC1_INTE3:                     u32 = IO_BANK0_BASE + 0x13C;
pub const IO_BANK0_PROC1_INTF0:                     u32 = IO_BANK0_BASE + 0x140;
pub const IO_BANK0_PROC1_INTF1:                     u32 = IO_BANK0_BASE + 0x144;
pub const IO_BANK0_PROC1_INTF2:                     u32 = IO_BANK0_BASE + 0x148;
pub const IO_BANK0_PROC1_INTF3:                     u32 = IO_BANK0_BASE + 0x14C;
pub const IO_BANK0_PROC1_INTS0:                     u32 = IO_BANK0_BASE + 0x150;
pub const IO_BANK0_PROC1_INTS1:                     u32 = IO_BANK0_BASE + 0x154;
pub const IO_BANK0_PROC1_INTS2:                     u32 = IO_BANK0_BASE + 0x158;
pub const IO_BANK0_PROC1_INTS3:                     u32 = IO_BANK0_BASE + 0x15C;
pub const IO_BANK0_DORMANT_WAKE_INTE0:              u32 = IO_BANK0_BASE + 0x160;
pub const IO_BANK0_DORMANT_WAKE_INTE1:              u32 = IO_BANK0_BASE + 0x164;
pub const IO_BANK0_DORMANT_WAKE_INTE2:              u32 = IO_BANK0_BASE + 0x168;
pub const IO_BANK0_DORMANT_WAKE_INTE3:              u32 = IO_BANK0_BASE + 0x16C;
pub const IO_BANK0_DORMANT_WAKE_INTF0:              u32 = IO_BANK0_BASE + 0x170;
pub const IO_BANK0_DORMANT_WAKE_INTF1:              u32 = IO_BANK0_BASE + 0x174;
pub const IO_BANK0_DORMANT_WAKE_INTF2:              u32 = IO_BANK0_BASE + 0x178;
pub const IO_BANK0_DORMANT_WAKE_INTF3:              u32 = IO_BANK0_BASE + 0x17C;
pub const IO_BANK0_DORMANT_WAKE_INTS0:              u32 = IO_BANK0_BASE + 0x180;
pub const IO_BANK0_DORMANT_WAKE_INTS1:              u32 = IO_BANK0_BASE + 0x184;
pub const IO_BANK0_DORMANT_WAKE_INTS2:              u32 = IO_BANK0_BASE + 0x188;
pub const IO_BANK0_DORMANT_WAKE_INTS3:              u32 = IO_BANK0_BASE + 0x18C;


pub const fn ioBank0GpioCtrl(pin: u32) -> u32 {
     IO_BANK0_BASE + 0x04 + pin * 8
}

pub const fn ioBank0GpioStatus(pin: u32) -> u32 {
    IO_BANK0_BASE + 0x08 + pin * 8
}