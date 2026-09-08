#![allow(dead_code)]
// UART1
pub const UART1_BASE:                               u32 = 0x4003_8000;
pub const UART1_UARTDR:                             u32 = UART1_BASE + 0x0;
pub const UART1_UARTRSR:                            u32 = UART1_BASE + 0x4;
pub const UART1_UARTFR:                             u32 = UART1_BASE + 0x18;
pub const UART1_UARTILPR:                           u32 = UART1_BASE + 0x20;
pub const UART1_UARTIBRD:                           u32 = UART1_BASE + 0x24;
pub const UART1_UARTFBRD:                           u32 = UART1_BASE + 0x28;
pub const UART1_UARTLCR_H:                          u32 = UART1_BASE + 0x2C;
pub const UART1_UARTCR:                             u32 = UART1_BASE + 0x30;
pub const UART1_UARTIFLS:                           u32 = UART1_BASE + 0x34;
pub const UART1_UARTIMSC:                           u32 = UART1_BASE + 0x38;
pub const UART1_UARTRIS:                            u32 = UART1_BASE + 0x3C;
pub const UART1_UARTMIS:                            u32 = UART1_BASE + 0x40;
pub const UART1_UARTICR:                            u32 = UART1_BASE + 0x44;
pub const UART1_UARTDMACR:                          u32 = UART1_BASE + 0x48;
pub const UART1_UARTPERIPHID0:                      u32 = UART1_BASE + 0xFE0;
pub const UART1_UARTPERIPHID1:                      u32 = UART1_BASE + 0xFE4;
pub const UART1_UARTPERIPHID2:                      u32 = UART1_BASE + 0xFE8;
pub const UART1_UARTPERIPHID3:                      u32 = UART1_BASE + 0xFEC;
pub const UART1_UARTPCELLID0:                       u32 = UART1_BASE + 0xFF0;
pub const UART1_UARTPCELLID1:                       u32 = UART1_BASE + 0xFF4;
pub const UART1_UARTPCELLID2:                       u32 = UART1_BASE + 0xFF8;
pub const UART1_UARTPCELLID3:                       u32 = UART1_BASE + 0xFFC;
