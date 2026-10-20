use crate::hal::pin::*;
use crate::kernel::drivers::gpio_driver::GPIODriver;

#[derive(PartialEq)]
pub enum GPIOMode {
    In,
    Out,
}

pub struct GPIO<'a> {
    driver: &'a dyn GPIODriver,
    pin: Pin,
}

impl<'a> GPIO<'a> {
    pub fn new(driver: &'a dyn GPIODriver, pin: Pin) -> Self {
        Self { driver, pin }
    }

    pub fn destroy(self) -> Pin {
        return self.pin;
    }

    pub fn set_high(&self) {
        self.driver.write(self.pin.get(), true);
    }

    pub fn set_low(&self) {
        self.driver.write(self.pin.get(), false);
    }

    pub fn read(&self) -> bool {
        return self.driver.read(self.pin.get());
    }

    pub fn toggle(&self) {
        self.driver.toggle(self.pin.get());
    }

    pub fn get_mode(&self) -> GPIOMode {
        return self.driver.get_mode(self.pin.get());
    }

    pub fn set_mode(&self, mode: GPIOMode) {
        self.driver.set_mode(self.pin.get(), mode);
    }

    pub fn toggle_mode(&self) {
        self.driver.toggle_mode(self.pin.get());
    }
}
