use crate::Native::Constants::Config::*;
use crate::Native::Constants::RP2040::IO_BANK0::*;
use crate::Native::Constants::RP2040::SIO::*;

use crate::HAL::GPIO::GPIOMode;
use crate::Kernel::Drivers::GPIODriver::*;

pub struct _GPIODriver;

impl GPIODriver for _GPIODriver {
    fn initPin(&self, pin: u8) {
        unsafe {
            let gpioCtrl = (IO_BANK0_GPIO_CTRL(pin.into())) as *mut u32;
            // FUNCSEL = 5 (SIO)
            gpioCtrl.write_volatile(5);
        }
    }

    fn getMode(&self, pin: u8) -> GPIOMode {
        unsafe {
            let register = if pin >= GPIO_HI_PIN_START {
                SIO_GPIO_HI_OE as *mut u32
            } else {
                SIO_GPIO_OE as *mut u32
            };

            let bit = if pin >= GPIO_HI_PIN_START {
                pin - GPIO_HI_PIN_START
            } else {
                pin
            };

            let mask = 1u32 << bit;

            return if register.read_volatile() & mask != 0 {
                GPIOMode::Out
            } else {
                GPIOMode::In
            }
        }
    }

    fn toggleMode(&self, pin: u8) {
        unsafe {
            let register = if pin >= GPIO_HI_PIN_START {
                SIO_GPIO_HI_OE_XOR as *mut u32
            } else {
                SIO_GPIO_OE_XOR as *mut u32
            };

            let bit = if pin >= GPIO_HI_PIN_START {
                pin - GPIO_HI_PIN_START
            } else {
                pin
            };

            let mask = 1u32 << bit;
            register.write_volatile(mask);
        }
    }

    fn setMode(&self, pin: u8, mode: GPIOMode) {
        unsafe {
            let registerSet = if pin >= GPIO_HI_PIN_START {
                SIO_GPIO_HI_OE_SET as *mut u32
            } else {
                SIO_GPIO_OE_SET as *mut u32
            };

            let registerClear = if pin >= GPIO_HI_PIN_START {
                SIO_GPIO_HI_OE_CLR as *mut u32
            } else {
                SIO_GPIO_OE_CLR as *mut u32
            };

            let bit = if pin >= GPIO_HI_PIN_START {
                pin - GPIO_HI_PIN_START
            } else {
                pin
            };

            let mask = 1u32 << bit;

            if mode == GPIOMode::In {
                registerClear.write_volatile(mask);
            } else {
                registerSet.write_volatile(mask);
            }
        }
    }

    fn write(&self, pin: u8, state: bool) {
        unsafe {
            if self.getMode(pin) == GPIOMode::Out {
                let registerSet = if pin >= GPIO_HI_PIN_START {
                    SIO_GPIO_HI_OUT_SET as *mut u32
                } else {
                    SIO_GPIO_OUT_SET as *mut u32
                };

                let registerClear = if pin >= GPIO_HI_PIN_START {
                    SIO_GPIO_HI_OE_CLR as *mut u32
                } else {
                    SIO_GPIO_OE_CLR as *mut u32
                };

                let bit = if pin >= GPIO_HI_PIN_START {
                    pin - GPIO_HI_PIN_START
                } else {
                    pin
                };

                let mask = 1u32 << bit;

                if !state {
                    registerClear.write_volatile(mask);
                } else {
                    registerSet.write_volatile(mask);
                }
            }
        }
    }

    fn toggle(&self, pin: u8) {
        unsafe {
            let register = if pin >= GPIO_HI_PIN_START {
                SIO_GPIO_HI_OUT_XOR as *mut u32
            } else {
                SIO_GPIO_OUT_XOR as *mut u32
            };

            let bit = if pin >= GPIO_HI_PIN_START {
                pin - GPIO_HI_PIN_START
            } else {
                pin
            };

            let mask = 1u32 << bit;
            register.write_volatile(mask);
        }
    }

    fn read(&self, pin: u8) -> bool {
        unsafe {
            if self.getMode(pin) == GPIOMode::In {
                let register = if pin >= GPIO_HI_PIN_START {
                    SIO_GPIO_HI_IN as *mut u32
                } else {
                    SIO_GPIO_IN as *mut u32
                };

                let bit = if pin >= GPIO_HI_PIN_START {
                    pin - GPIO_HI_PIN_START
                } else {
                    pin
                };

                let mask = 1u32 << bit;

                let test = register.read_volatile() & mask;
                return if  test != 0 {
                    true
                } else {
                    false
                }
            }
            return false;
        }
    }
}

impl _GPIODriver {
    pub fn new() -> Self {
        Self
    }
}