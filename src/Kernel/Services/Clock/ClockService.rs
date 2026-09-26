use crate::Kernel::Drivers::ClockDriver::*;
use crate::Kernel::Services::Service::Service;
use crate::Kernel::Services::ServiceID::ServiceID;

pub struct ClockService<'a> {
    driver : &'a dyn ClockDriver
}

impl Service for ClockService<'_> {
    fn getID(&self) -> ServiceID {
        return ServiceID::ClockService;
    }
} 

impl<'a> ClockService<'a> {
    pub const fn new(driver: &'a dyn ClockDriver) -> Self {
       Self {
            driver
       }
    }

    pub fn getFrequency(&self, domain: ClockDomain) -> u32 {
        return self.driver.getFrequency(domain);
    }
}
