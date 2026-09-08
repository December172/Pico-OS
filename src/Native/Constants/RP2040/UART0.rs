#![allow(dead_code)]
// UART0
pub const UART0_BASE:                               u32 = 0x4003_4000;
pub const UART0_UARTDR:                             u32 = UART0_BASE + 0x0;
pub const UART0_UARTRSR:                            u32 = UART0_BASE + 0x4;
pub const UART0_UARTFR:                             u32 = UART0_BASE + 0x18;
pub const UART0_UARTILPR:                           u32 = UART0_BASE + 0x20;
pub const UART0_UARTIBRD:                           u32 = UART0_BASE + 0x24;
pub const UART0_UARTFBRD:                           u32 = UART0_BASE + 0x28;
pub const UART0_UARTLCR_H:                          u32 = UART0_BASE + 0x2C;
pub const UART0_UARTCR:                             u32 = UART0_BASE + 0x30;
pub const UART0_UARTIFLS:                           u32 = UART0_BASE + 0x34;
pub const UART0_UARTIMSC:                           u32 = UART0_BASE + 0x38;
pub const UART0_UARTRIS:                            u32 = UART0_BASE + 0x3C;
pub const UART0_UARTMIS:                            u32 = UART0_BASE + 0x40;
pub const UART0_UARTICR:                            u32 = UART0_BASE + 0x44;
pub const UART0_UARTDMACR:                          u32 = UART0_BASE + 0x48;
pub const UART0_UARTPERIPHID0:                      u32 = UART0_BASE + 0xFE0;
pub const UART0_UARTPERIPHID1:                      u32 = UART0_BASE + 0xFE4;
pub const UART0_UARTPERIPHID2:                      u32 = UART0_BASE + 0xFE8;
pub const UART0_UARTPERIPHID3:                      u32 = UART0_BASE + 0xFEC;
pub const UART0_UARTPCELLID0:                       u32 = UART0_BASE + 0xFF0;
pub const UART0_UARTPCELLID1:                       u32 = UART0_BASE + 0xFF4;
pub const UART0_UARTPCELLID2:                       u32 = UART0_BASE + 0xFF8;
pub const UART0_UARTPCELLID3:                       u32 = UART0_BASE + 0xFFC;
