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
    pub fn new(driver: DriverType, refFreq: u32) -> Self {
        if !driver.init(refFreq) {
            panic!("Timer service initialization Failed")
        }
        Self {
            driver
        }
    }

    pub fn claimTimer(&self) -> Timer<'_> {
        return Timer::new(&self.driver);
    }
}

