#[repr(u8)]
pub enum ServiceID {
    PinService,
    GPIOService,
    SPIService,
    UARTService,

    ClockService,
    TimerService
}