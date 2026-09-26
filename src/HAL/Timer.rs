use core::time::Duration;

use crate::Util::Time::*;
use crate::Kernel::Drivers::TimerDriver::TimerDriver;

pub struct Timer<'a> {
    driver : &'a dyn TimerDriver,
}

impl<'a> Timer<'a> {
    pub fn new(driver: &'a dyn TimerDriver) -> Self {
        Self { 
            driver
        }        
    }

    pub fn now(&self) -> Time {
        return Time::fromTicks(self.driver.nowTick());
    }   

    pub fn delay(&self, timeout: Duration) {
        let startTime = Time::fromTicks(self.driver.nowTick());

        let endTime = startTime + timeout;

        while Time::fromTicks(self.driver.nowTick()) < endTime {
            core::hint::spin_loop();
        }
    }

    pub fn release(self) {
        return ();
    }
}