use crate::HAL::GPIO::GPIOMode;
use crate::Util::Register::Register;
use crate::Kernel::Drivers::GPIODriver::*;

use crate::Native::Constants::Config::*;

use crate::Native::Constants::RP2040::IO_BANK0::*;
use crate::Native::Constants::RP2040::SIO::*;

pub struct _GPIODriver;

impl GPIODriver for _GPIODriver {
    fn initPin(&self, pin: u8) {
        let gpioCtrl = Register::new(IO_BANK0_GPIO_CTRL(pin.into()));
        // FUNCSEL = 5 (SIO)
        gpioCtrl.write(5);
    }

    fn getMode(&self, pin: u8) -> GPIOMode {
        let register = if pin >= GPIO_HI_PIN_START {
            Register::new(SIO_GPIO_HI_OE)
        }else {
            Register::new(SIO_GPIO_OE)
        };
        let bit = if pin >= GPIO_HI_PIN_START {
            pin - GPIO_HI_PIN_START
        } else {
            pin
        };

        let mask = 1u32 << bit;

        return if register.read() & mask != 0 {
            GPIOMode::Out
        } else {
            GPIOMode::In
        }
    }

    fn toggleMode(&self, pin: u8) {
        let register = if pin >= GPIO_HI_PIN_START {
            Register::new(SIO_GPIO_HI_OE_XOR)
        } else {
            Register::new(SIO_GPIO_OE_XOR)
        };
        let bit = if pin >= GPIO_HI_PIN_START {
            pin - GPIO_HI_PIN_START
        } else {
            pin
        };

        let mask = 1u32 << bit;

        register.write(mask);
    }

    fn setMode(&self, pin: u8, mode: GPIOMode) {
        let registerSet = if pin >= GPIO_HI_PIN_START {
            Register::new(SIO_GPIO_HI_OE_SET)
        } else {
            Register::new(SIO_GPIO_OE_SET)
        };
        let registerClear = if pin >= GPIO_HI_PIN_START {
            Register::new(SIO_GPIO_HI_OE_CLR)
        } else {
            Register::new(SIO_GPIO_OE_CLR)
        };
        let bit = if pin >= GPIO_HI_PIN_START {
            pin - GPIO_HI_PIN_START
        } else {
            pin
        };

        let mask = 1u32 << bit;

        if mode == GPIOMode::In {
            registerClear.write(mask);
        } else {
            registerSet.write(mask);
        }
    }

    fn write(&self, pin: u8, state: bool) {
        if self.getMode(pin) == GPIOMode::Out {
            let registerSet = if pin >= GPIO_HI_PIN_START {
                Register::new(SIO_GPIO_HI_OUT_SET)
            } else {
                Register::new(SIO_GPIO_OUT_SET)
            };
            let registerClear = if pin >= GPIO_HI_PIN_START {
                Register::new(SIO_GPIO_HI_OE_CLR)
            } else {
                Register::new(SIO_GPIO_OE_CLR)
            };
            let bit = if pin >= GPIO_HI_PIN_START {
                pin - GPIO_HI_PIN_START
            } else {
                pin
            };

            let mask = 1u32 << bit;

            if !state {
                registerClear.write(mask);
            } else {
                registerSet.write(mask);
            }
        }
    }

    fn toggle(&self, pin: u8) {
        let register = if pin >= GPIO_HI_PIN_START {
            Register::new(SIO_GPIO_HI_OUT_XOR)
        } else {
            Register::new(SIO_GPIO_OUT_XOR)
        };
        let bit = if pin >= GPIO_HI_PIN_START {
            pin - GPIO_HI_PIN_START
        } else {
            pin
        };

        let mask = 1u32 << bit;
        register.write(mask);
    }

    fn read(&self, pin: u8) -> bool {
        if self.getMode(pin) == GPIOMode::In {
            let register = if pin >= GPIO_HI_PIN_START {
                Register::new(SIO_GPIO_HI_IN)
            } else {
                Register::new(SIO_GPIO_IN)
            };
            let bit = if pin >= GPIO_HI_PIN_START {
                pin - GPIO_HI_PIN_START
            } else {
                pin
            };

            let mask = 1u32 << bit;

            return if register.read() & mask != 0 {
                true
            } else {
                false
            }
        }
        return false;
    }
}

impl _GPIODriver {
    pub fn new() -> Self {
        Self
    }
}