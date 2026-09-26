use core::time::Duration;

use crate::HAL::GPIO::*;
use crate::Kernel::Drivers::ClockDriver::ClockDriver;
use crate::Kernel::Services::Pin::PinService::PinService;
use crate::Kernel::Services::GPIO::GPIOService::GPIOService;
use crate::Kernel::Services::Clock::ClockService::ClockService;
use crate::Kernel::Services::Timer::TimerService::TimerService;

use crate::Util::Time::*;

use crate::Native::Boot::_init;
use crate::Native::Drivers::*;

/// Kernel entry point
/// no reentry
pub fn kernelMain() {
    _init();
    let roscDriver = ROSCDriver::new();
    let xoscDriver = XOSCDriver::new();
    let clockDriver: &dyn ClockDriver = if !xoscDriver.init() {
        &roscDriver
    } else {
        &xoscDriver
    };
    let mut pinService = PinService::new();
    let clockService = ClockService::new(clockDriver);
    let timerService = TimerService::new(TimerDriver::new());
    let gpioService = GPIOService::new(GPIODriver::new());
    
    let timer = timerService.claimTimer();

    // Testing GPIO / Clock system is working correctly
    let led = gpioService.claim(&mut pinService, 24)
                                   .unwrap_or_else(|| panic!("GPIO initialization Failed"));
    let led2 = gpioService.claim(&mut pinService, 25)
                                   .unwrap_or_else(|| panic!("GPIO initialization Failed"));
    let button = gpioService.claim(&mut pinService, 18)
                                      .unwrap_or_else(|| panic!("GPIO initialization failed"));
    button.setMode(GPIOMode::In);  
    led.setMode(GPIOMode::Out);
    led2.setMode(GPIOMode::Out);
    let debounceTime = Duration::from_millis(1);
    let waitTime = Duration::from_millis(1);
    loop {
        timer.delay(waitTime);
        led2.toggle();
    }
}