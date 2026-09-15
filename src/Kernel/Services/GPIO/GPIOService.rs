use crate::HAL::GPIO::GPIO;
use crate::Kernel::Drivers::GPIODriver::GPIODriver;
use crate::Kernel::Services::Pin::PinService::PinService;
use crate::Kernel::Services::Service::Service;
use crate::Kernel::Services::ServiceID::ServiceID;

pub struct GPIOService<'a> {
    driver: &'a dyn GPIODriver
}

impl<'a> GPIOService<'a> {
    pub fn new(driver: &'a dyn GPIODriver) -> GPIOService<'a> {
        GPIOService {
            driver
        }
    }

    pub fn claim(&'a self, pinService: &mut PinService, pin: u8) -> Option<GPIO<'a>> {
        if let Some(basePin) = pinService.claim(pin) {
            return Some(GPIO::new(self.driver, basePin));
        }
        return None;
    }

    pub fn release(&self, pinService: &mut PinService, gpio: GPIO<'a>) {
        let basePin = gpio.destroy();
        pinService.release(basePin);
    }
}

impl Service for GPIOService<'_> {
    fn getID(&self) -> ServiceID {
        return ServiceID::GPIOService;
    }
}