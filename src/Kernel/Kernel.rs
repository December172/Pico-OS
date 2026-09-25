use crate::HAL::GPIO::*;
use crate::Kernel::Drivers::ClockDriver::ClockDriver;
use crate::Kernel::Services::Pin::PinService::PinService;
use crate::Kernel::Services::GPIO::GPIOService::GPIOService;
use crate::Kernel::Services::Clock::ClockService::ClockService;

use crate::Native::Boot::_init;
use crate::Native::Drivers::*;

fn delay() {
    for _ in 1..100 {
        core::hint::spin_loop();
    }
}

/// Kernel entry point
/// no reentry
pub fn kernelMain() {
    _init();
    let roscDriver = ROSCDriver::new();
    let xoscDriver = XOSCDriver::new();
    let gpioDriver= GPIODriver::new();
    let clockDriver: &dyn ClockDriver = if !xoscDriver.init() {
        &roscDriver
    } else {
        &xoscDriver
    };
    let clockService = ClockService::new(clockDriver);
    let gpioService = GPIOService::new(&gpioDriver);
    let mut pinService = PinService::new();
    
    // Testing GPIO / Clock system is working correctly
    let led = gpioService.claim(&mut pinService, 24)
                                   .unwrap_or_else(|| panic!("GPIO initialization Failed"));
    let button = gpioService.claim(&mut pinService, 18)
                                      .unwrap_or_else(|| panic!("GPIO initialization failed"));
    button.setMode(GPIOMode::In);  
    led.setMode(GPIOMode::Out);
    loop {
        // debounce
        if !button.read() {

            delay();

            if !button.read() {
                led.toggle();

                while !button.read() {
                    // wait for release
                }
            }
        }
    }
}