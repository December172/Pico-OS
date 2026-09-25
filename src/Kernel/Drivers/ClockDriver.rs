#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ClockDomain {
    System,
    Peripherals,
    USB,
}
// TODO: Configuable system clock dividers
pub trait ClockDriver {
    fn init(&self) -> bool;

    fn enable(&self) -> bool;
    fn disable(&self, domain: ClockDomain);

    fn isPrecise(&self) -> bool;

    fn getFrequency(&self, domain: ClockDomain) -> u32;

}