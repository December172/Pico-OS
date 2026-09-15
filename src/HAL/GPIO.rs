use crate::HAL::Pin::*;
use crate::Kernel::Drivers::GPIODriver::GPIODriver;

#[derive(PartialEq)]
pub enum GPIOMode {
    In,
    Out
}

pub struct GPIO<'a> {
    driver: &'a dyn GPIODriver,
    pin: Pin,
}

impl<'a> GPIO<'a> {
    pub fn new(driver: &'a dyn GPIODriver, pin: Pin) -> GPIO<'a> {
        driver.initPin(pin.get());
        return GPIO {
            driver,
            pin,
        }
    }

    pub fn setHigh(&self) {
        self.driver.write(self.pin.get(), true);
    }

    pub fn setLow(&self) {
        self.driver.write(self.pin.get(), false);
    }

    pub fn read(&self) -> bool {
        return self.driver.read(self.pin.get());
    }

    pub fn toggle(&self) {
        self.driver.toggle(self.pin.get());
    }

    pub fn destroy(self) -> Pin {
        return self.pin;
    }

    pub fn getMode(&self) -> GPIOMode {
        return self.driver.getMode(self.pin.get());
    }

    pub fn setMode(&self, mode: GPIOMode) {
        self.driver.setMode(self.pin.get(), mode);
    }

    pub fn toggleMode(&self) {
        self.driver.toggleMode(self.pin.get());
    }
}