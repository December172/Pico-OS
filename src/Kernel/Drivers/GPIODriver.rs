use crate::HAL::GPIO::GPIOMode;

pub trait GPIODriver {
    fn initPin(&self, pin: u8);

    fn getMode(&self, pin: u8) -> GPIOMode;
    fn setMode(&self, pin: u8, mode: GPIOMode);
    fn toggleMode(&self, pin: u8);

    fn write(&self, pin: u8, state: bool);
    fn toggle(&self, pin: u8);
    fn read(&self, pin: u8) -> bool;
}