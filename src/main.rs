#![no_std]
#![no_main]
#![allow(non_snake_case)]

mod Native;
use crate::Native::Boot::RP2040::Init::init;
use crate::Native::Constants::RP2040::SIO::*;

#[unsafe(no_mangle)]
pub extern "C" fn kernelMain() -> ! {
    init();
    unsafe {
        let gpioXor = SIO_GPIO_OUT_XOR as *mut u32;
        loop {
            gpioXor.write_volatile(1 << 25);

            // for dev profile loop
            for _ in 0..10000 {
                core::hint::spin_loop();
            }
        }
    }
}