#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ClockDomain {
    Reference,
    System,
    Peripherals,
    Usb,
}

// TODO: Configuable system clock dividers
pub trait ClockDriver {
    fn init(&self) -> bool;

    fn enable(&self) -> bool;
    fn disable(&self, domain: ClockDomain);

    fn is_precise(&self) -> bool;

    fn get_frequency(&self, domain: ClockDomain) -> u32;
}
