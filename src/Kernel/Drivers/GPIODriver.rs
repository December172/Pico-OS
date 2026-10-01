use crate::HAL::GPIO::GPIOMode; 
use crate::HAL::Pin::PinCapability;

pub trait GPIODriver {
    /// Initialize a pin for GPIO function
    fn initPin(&self, pin: u32) -> bool;

    fn getMode(&self, pin: u32) -> GPIOMode;
    fn setMode(&self, pin: u32, mode: GPIOMode);
    fn toggleMode(&self, pin: u32);

    fn write(&self, pin: u32, state: bool);
    fn toggle(&self, pin: u32);
    fn read(&self, pin: u32) -> bool;

    fn getPinCapabilities(&self) -> &'static [PinCapability];
}