use crate::Kernel::Drivers::ClockDriver::*;

pub struct ClockService<'a> {
    driver: &'a dyn ClockDriver,
}

impl<'a> ClockService<'a> {
    pub const fn new(driver: &'a dyn ClockDriver) -> Self {
        Self { driver }
    }

    pub fn getFrequency(&self, domain: ClockDomain) -> u32 {
        return self.driver.getFrequency(domain);
    }
}
