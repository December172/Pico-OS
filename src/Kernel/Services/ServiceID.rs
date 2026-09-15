#[repr(usize)]
pub enum ServiceID {
    PinService,
    GPIOService,
    SPIService,
    UARTService,
}