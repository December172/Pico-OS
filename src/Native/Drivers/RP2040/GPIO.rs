use crate::HAL::GPIO::GPIO;
use crate::Native::Constants::RP2040::IO_BANK0::ioBank0GpioCtrl;
use crate::Native::Constants::RP2040::SIO::SIO_GPIO_OUT_XOR;
use crate::Native::Constants::RP2040::SIO::SIO_GPIO_OUT_HIGH;
use crate::Native::Constants::RP2040::SIO::SIO_GPIO_OUT_LOW;

pub struct _GPIO<const PIN: u8>;

impl<const PIN: u8> GPIO for _GPIO<PIN> {
    const PIN: u8 = PIN;

    pub fn setHigh(&self) {
        unsafe {
            useGPIO(PIN);
            let gpioHigh = crate::Native::Constants::RP2040::SIO::SIO_GPIO_OUT_HIGH as *mut u32;
            gpioHigh.write_volatile(1 << PIN);
        }
    }

    pub fn setLow(&self) {
        unsafe {
            useGPIO(PIN);
            let gpioLow = crate::Native::Constants::RP2040::SIO::SIO_GPIO_OUT_LOW as *mut u32;
            gpioLow.write_volatile(1 << PIN);
        }
    }

    pub fn toggle(&self) {
        unsafe {
            useGPIO(PIN);
            let gpioXor = crate::Native::Constants::RP2040::SIO::SIO_GPIO_OUT_XOR as *mut u32;
            gpioXor.write_volatile(1 << PIN);
        }
    }

    pub fn getState(&self) -> bool {
        unsafe {
            let gpioIn = crate::Native::Constants::RP2040::SIO::SIO_GPIO_IN as *const u32;
            let state = gpioIn.read_volatile();
            (state & (1 << PIN)) != 0
        }
    }

    fn useGPIO(pin: u8) {
        unsafe {
            let gpioCtrl = crate::Native::Constants::RP2040::IO_BANK0::ioBank0GpioCtrl(pin) as *mut u32;
            gpioCtrl.write_volatile(5); // FUNCSEL = 5 (SIO)
        }
    }
} 