use crate::Kernel::Drivers::ClockDriver::*;
use crate::Kernel::Managers::Pin::PinManager::PinManager;
use crate::Kernel::Services::GPIO::GPIOService::GPIOService;
use crate::Kernel::Services::Clock::ClockService::ClockService;
use crate::Kernel::Services::Timer::TimerService::TimerService;

use crate::Native::Boot::_init;
use crate::Native::Drivers::*;

/// Kernel entry point
/// no reentry
pub fn kernelMain() {
    _init();

    // TODO: Switch to heap-allocated kernel object after finishing slab allocator
    let roscDriver = ROSCDriver::new();
    let xoscDriver = XOSCDriver::new();
    let clockDriver: &dyn ClockDriver;
    // Fallback clock source
    roscDriver.init();
    if !xoscDriver.init() {        
        clockDriver = &roscDriver;
    } else {
        clockDriver = &xoscDriver
    }
    clockDriver.enable();
    let clockService = ClockService::new(clockDriver);

    let timerService = TimerService::new(TimerDriver::new(), 
                                         clockService.getFrequency(ClockDomain::Reference));

    let mut pinManager = PinManager::new();
    let gpioService = GPIOService::new(GPIODriver::new());

    pinManager.registerCapabilities(gpioService.getPinCapabilities());

    loop {}
}