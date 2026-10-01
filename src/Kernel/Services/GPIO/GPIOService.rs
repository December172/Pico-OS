use crate::HAL::Pin::*;
use crate::HAL::GPIO::GPIO;

use crate::Kernel::Drivers::GPIODriver::GPIODriver;
use crate::Kernel::Managers::Pin::PinManager::PinManager;
use crate::Kernel::Services::Service::Service;
use crate::Kernel::Services::ServiceID::ServiceID;

pub struct GPIOService<DriverType: GPIODriver> {
    driver: DriverType
}

impl<DriverType: GPIODriver> GPIOService<DriverType> {
    pub const fn new(driver: DriverType) -> GPIOService<DriverType> {
        GPIOService {
            driver
        }
    }

    pub fn claim(&self, pinManager: &mut PinManager, pin: Pin) -> Option<GPIO<'_>> {
        if self.driver.initPin(pin.get()) {
            return Some(GPIO::new(&self.driver, pin));
        } else {
            pinManager.release(pin);
        }
        return None;
    }

    pub fn release(&self, pinManager: &mut PinManager, gpio: GPIO<'_>) {
        let basePin = gpio.destroy();
        pinManager.release(basePin);
    }

    pub fn getPinCapabilities(&self) -> &'static [PinCapability] {
        return self.driver.getPinCapabilities();
    }
}

impl<DriverType: GPIODriver> Service for GPIOService<DriverType> {
    fn getID(&self) -> ServiceID {
        return ServiceID::GPIOService;
    }
}