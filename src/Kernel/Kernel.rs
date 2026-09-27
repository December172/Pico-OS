use core::time::Duration;

use crate::HAL::GPIO::*;
use crate::Kernel::Drivers::ClockDriver::{ClockDomain, ClockDriver};
use crate::Kernel::Services::Pin::PinService::PinService;
use crate::Kernel::Services::GPIO::GPIOService::GPIOService;
use crate::Kernel::Services::Clock::ClockService::ClockService;
use crate::Kernel::Services::Timer::TimerService::TimerService;

use crate::Native::Boot::_init;
use crate::Native::Drivers::*;

/// Kernel entry point
/// no reentry
pub fn kernelMain() {
    _init();
    let roscDriver = ROSCDriver::new();
    let xoscDriver = XOSCDriver::new();
    let clockDriver: &dyn ClockDriver;
    if !xoscDriver.init() {
        roscDriver.init();
        clockDriver = &roscDriver;
    } else {
        clockDriver = &xoscDriver
    }
    clockDriver.enable();

    let mut pinService = PinService::new();
    let clockService = ClockService::new(clockDriver);

    // The tick generator counts on clk_ref, not on clk_sys, so ask for the
    // *reference* frequency. Using ClockDomain::System here makes every delay
    // clk_sys/clk_ref times too long (125/12 ~= 10x on the RP2040).
    let timerService = TimerService::new(TimerDriver::new(), 
                                         clockService.getFrequency(ClockDomain::Reference));

    let gpioService = GPIOService::new(GPIODriver::new());

    let timer = timerService.claimTimer();

    // Testing GPIO / Clock system is working correctly
    let led2 = gpioService.claim(&mut pinService, 25)
                          .unwrap_or_else(|| panic!("GPIO initialization Failed"));
    led2.setMode(GPIOMode::Out);
    let waitTime = Duration::from_millis(100);
    loop {
        timer.delay(waitTime);
        led2.toggle();
    }
}