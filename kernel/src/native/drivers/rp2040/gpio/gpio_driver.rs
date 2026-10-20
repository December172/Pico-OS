use crate::hal::gpio::GPIOMode;
use crate::hal::pin::PinCapability;
use crate::kernel::drivers::gpio_driver::*;
use crate::util::register::Register;

use crate::native::constants::config::*;
use crate::native::constants::rp2040::io_bank0::*;
use crate::native::constants::rp2040::sio::*;

use crate::native::drivers::rp2040::gpio::pin_capabilities::PIN_CAPABILITIES;

pub struct _GPIODriver;

impl GPIODriver for _GPIODriver {
    fn init_pin(&self, pin: u32) -> bool {
        let gpio_ctrl = Register::new(io_bank0_gpio_ctrl(pin));
        // FUNCSEL = 5 (sio)
        gpio_ctrl.write(5);
        return true;
    }

    fn get_mode(&self, pin: u32) -> GPIOMode {
        let register = if pin >= GPIO_HI_PIN_START {
            Register::new(SIO_GPIO_HI_OE)
        } else {
            Register::new(SIO_GPIO_OE)
        };
        let bit = if pin >= GPIO_HI_PIN_START {
            pin - GPIO_HI_PIN_START
        } else {
            pin
        };

        return if register.bit_get(bit) {
            GPIOMode::Out
        } else {
            GPIOMode::In
        };
    }

    fn toggle_mode(&self, pin: u32) {
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

        register.bit_set(bit, true);
    }

    fn set_mode(&self, pin: u32, mode: GPIOMode) {
        let register_set = if pin >= GPIO_HI_PIN_START {
            Register::new(SIO_GPIO_HI_OE_SET)
        } else {
            Register::new(SIO_GPIO_OE_SET)
        };
        let register_clear = if pin >= GPIO_HI_PIN_START {
            Register::new(SIO_GPIO_HI_OE_CLR)
        } else {
            Register::new(SIO_GPIO_OE_CLR)
        };
        let bit = if pin >= GPIO_HI_PIN_START {
            pin - GPIO_HI_PIN_START
        } else {
            pin
        };

        if mode == GPIOMode::In {
            register_clear.bit_set(bit, true);
        } else {
            register_set.bit_set(bit, true);
        }
    }

    fn write(&self, pin: u32, state: bool) {
        if self.get_mode(pin) == GPIOMode::Out {
            let register_set = if pin >= GPIO_HI_PIN_START {
                Register::new(SIO_GPIO_HI_OUT_SET)
            } else {
                Register::new(SIO_GPIO_OUT_SET)
            };
            let register_clear = if pin >= GPIO_HI_PIN_START {
                Register::new(SIO_GPIO_HI_OUT_CLR)
            } else {
                Register::new(SIO_GPIO_OUT_CLR)
            };
            let bit = if pin >= GPIO_HI_PIN_START {
                pin - GPIO_HI_PIN_START
            } else {
                pin
            };

            if !state {
                register_clear.bit_set(bit, true);
            } else {
                register_set.bit_set(bit, true);
            }
        }
    }

    fn toggle(&self, pin: u32) {
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

        register.bit_set(bit, true);
    }

    fn read(&self, pin: u32) -> bool {
        if self.get_mode(pin) == GPIOMode::In {
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

            return if register.bit_get(bit) { true } else { false };
        }
        return false;
    }

    fn get_pin_capabilities(&self) -> &'static [PinCapability] {
        return PIN_CAPABILITIES;
    }
}

impl _GPIODriver {
    pub fn new() -> Self {
        Self
    }
}
