use crate::hal::gpio::GPIOMode;
use crate::hal::pin::PinCapability;

pub trait GPIODriver {
    /// Initialize a pin for GPIO function
    fn init_pin(&self, pin: u32) -> bool;

    fn get_mode(&self, pin: u32) -> GPIOMode;
    fn set_mode(&self, pin: u32, mode: GPIOMode);
    fn toggle_mode(&self, pin: u32);

    fn write(&self, pin: u32, state: bool);
    fn toggle(&self, pin: u32);
    fn read(&self, pin: u32) -> bool;

    fn get_pin_capabilities(&self) -> &'static [PinCapability];
}
