pub const PINOUT_UNUSED: u32 = 0xFF;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PinFunction {
    GPIO,

    UartTx,
    UartRx,
    UartCts,
    UartRts,

    SpiTx,
    SpiRx,
    SpiSck,
    SpiCs,

    I2cSda,
    I2cScl,

    PwmA,
    PwmB,
}

#[derive(Clone, Copy)]
pub struct PinCapability {
    pub pin: u32,
    pub function: PinFunction,
    pub peripheral_block: u32,
}

pub struct Pin {
    n: u32,
}

impl Pin {
    pub(crate) fn new(n: u32) -> Pin {
        Pin { n }
    }

    pub fn get(&self) -> u32 {
        return self.n;
    }
}
