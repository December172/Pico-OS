#![no_std]
#![no_main]
#![allow(non_snake_case)]

#![allow(unused)]

mod Native;
mod HAL;
mod Kernel;

use crate::HAL::GPIO::*;
use crate::Kernel::Services::Pin::PinService::PinService;
use crate::Kernel::Services::GPIO::GPIOService::GPIOService;

use crate::Native::Boot::RP2040::Init::init;
#[cfg(feature = "RP2040")]
use crate::Native::Drivers::RP2040::GPIODriver::_GPIODriver;

#[unsafe(no_mangle)]
pub extern "C" fn kernelMain() -> ! {
    init();
    let gpioDriver = _GPIODriver::new();
    let mut pinService = PinService::new();
    let gpioService = GPIOService::new(&gpioDriver);
    let led = gpioService.claim(&mut pinService, 24)
                                   .unwrap_or_else(|| panic!("GPIO initialization Failed"));
    let button = gpioService.claim(&mut pinService, 18)
                                      .unwrap_or_else(|| panic!("GPIO initialization failed"));
    led.setMode(GPIOMode::Out);
    button.setMode(GPIOMode::In);
                                            
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

pub fn delay() {
    for _ in 1..100 {
        core::hint::spin_loop();
    }
}