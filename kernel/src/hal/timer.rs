use core::time::Duration;

use crate::kernel::drivers::timer_driver::TimerDriver;
use crate::util::time::*;

pub struct Timer<'a> {
    driver: &'a dyn TimerDriver,
}

impl<'a> Timer<'a> {
    pub fn new(driver: &'a dyn TimerDriver) -> Self {
        Self { driver }
    }

    pub fn now(&self) -> Time {
        return Time::from_ticks(self.driver.now_tick());
    }

    pub fn delay(&self, timeout: Duration) {
        let start_time = Time::from_ticks(self.driver.now_tick());

        let end_time = start_time + timeout;

        while Time::from_ticks(self.driver.now_tick()) < end_time {
            core::hint::spin_loop();
        }
    }

    pub fn release(self) {
        return ();
    }
}
