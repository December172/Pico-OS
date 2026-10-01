#[repr(u8)]
pub enum ServiceID {
    GPIOService,
    SPIService,
    UARTService,

    ClockService,
    TimerService
}