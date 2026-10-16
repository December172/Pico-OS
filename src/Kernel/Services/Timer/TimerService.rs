use crate::HAL::Timer::Timer;
use crate::Kernel::Drivers::TimerDriver::TimerDriver;

pub struct TimerService<DriverType: TimerDriver> {
    driver: DriverType,
}

impl<DriverType: TimerDriver> TimerService<DriverType> {
    pub fn new(driver: DriverType, refFreq: u32) -> Self {
        if !driver.init(refFreq) {
            panic!("Timer service initialization Failed")
        }
        Self { driver }
    }

    pub fn claimTimer(&self) -> Timer<'_> {
        return Timer::new(&self.driver);
    }
}
