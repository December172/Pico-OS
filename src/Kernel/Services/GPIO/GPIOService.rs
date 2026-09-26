use crate::HAL::GPIO::GPIO;
use crate::Kernel::Drivers::GPIODriver::GPIODriver;
use crate::Kernel::Services::Pin::PinService::PinService;
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

    pub fn claim(&self, pinService: &mut PinService, pin: u8) -> Option<GPIO<'_>> {
        if let Some(basePin) = pinService.claim(pin) {
            return Some(GPIO::new(&self.driver, basePin));
        }
        return None;
    }

    pub fn release(&self, pinService: &mut PinService, gpio: GPIO<'_>) {
        let basePin = gpio.destroy();
        pinService.release(basePin);
    }
}

impl<DriverType: GPIODriver> Service for GPIOService<DriverType> {
    fn getID(&self) -> ServiceID {
        return ServiceID::GPIOService;
    }
}