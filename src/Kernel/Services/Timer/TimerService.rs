use crate::HAL::Timer::Timer; 
use crate::Kernel::Drivers::TimerDriver::TimerDriver;
use crate::Kernel::Services::Service::Service;
use crate::Kernel::Services::ServiceID::ServiceID;

pub struct TimerService<DriverType: TimerDriver> {
    driver: DriverType
}

impl<DriverType: TimerDriver> Service for TimerService<DriverType> {
    fn getID(&self) -> ServiceID {
        return ServiceID::TimerService;
    }
}

impl<DriverType: TimerDriver> TimerService<DriverType>  {
    pub fn new(driver: DriverType) -> Self {
        Self {
            driver
        }
    }

    pub fn claimTimer(&self) -> Timer<'_> {
        return Timer::new(&self.driver);
    }

    pub fn claimAlarm(&self) {
        // Stub
    }

    pub fn releaseAlarm(&mut self) {
        // Stub
        // Note that the async alarm system may involve interrupts and core1 related codes
        // it will be a hard work to implement full-edged alarm system
        // TODO: alarm & rtc-alarm system
    }
}

