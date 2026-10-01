pub const PINOUT_UNUSED: u32 = 0xFF;

#[repr(u8)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum PinFunction {
    GPIO,

    UART_TX,
    UART_RX,
    UART_CTS,
    UART_RTS,

    SPI_TX,
    SPI_RX,
    SPI_SCK,
    SPI_CS,

    I2C_SDA,
    I2C_SCL,

    PWM_A,
    PWM_B,
}

#[derive(Clone, Copy)]
pub struct PinCapability {
    pub pin: u32,
    pub function: PinFunction,
    pub peripheralBlock: u32,
}

pub struct Pin {
    N : u32
}

impl Pin {
    pub(crate) fn new(n : u32) -> Pin {
        Pin {
            N : n 
        }
    }

    pub fn get(&self) -> u32 {
        return self.N;
    }
}