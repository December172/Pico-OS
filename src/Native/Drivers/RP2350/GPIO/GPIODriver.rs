use crate::HAL::GPIO::GPIOMode;
use crate::HAL::Pin::PinCapability;
use crate::Util::Register::Register;
use crate::Kernel::Drivers::GPIODriver::*;

use crate::Native::Constants::Config::*;
use crate::Native::Constants::RP2350::IO_BANK0::*;
use crate::Native::Constants::RP2350::PADS_BANK0::*;
use crate::Native::Constants::RP2350::SIO::*;

use crate::Native::Drivers::RP2350::GPIO::PinCapabilities::PIN_CAPABILITIES;

pub struct _GPIODriver;

impl GPIODriver for _GPIODriver {
    fn initPin(&self, pin: u32) {
        let gpioCtrl = Register::new(IO_BANK0_GPIO_CTRL(pin));
        // FUNCSEL = 5 (SIO)
        gpioCtrl.write(5);

        // RP2350 pads come out of reset *isolated* (PADS_BANK0.GPIOx.ISO = 1)
        // and the isolation latches keep the pad's output enable / level frozen,
        // so nothing the SIO writes ever reaches the pin. The bit has to be
        // cleared once the pad is muxed. RP2040 had no ISO bit, so ports from
        // it (like this driver) tend to miss this step.
        let padCtrl = Register::new(PADS_BANK0_GPIO(pin));
        padCtrl.bitSet(PADS_BANK0_GPIO_ISO_BIT, false);
    }

    fn getMode(&self, pin: u32) -> GPIOMode {
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

        return if register.bitGet(bit) {
            GPIOMode::Out
        } else {
            GPIOMode::In
        }
    }

    fn toggleMode(&self, pin: u32) {
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

        register.bitSet(bit, true);
    }

    fn setMode(&self, pin: u32, mode: GPIOMode) {
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

        if mode == GPIOMode::In {
            registerClear.bitSet(bit, true);
        } else {
            registerSet.bitSet(bit, true);
        }
    }

    fn write(&self, pin: u32, state: bool) {
        if self.getMode(pin) == GPIOMode::Out {
            let registerSet = if pin >= GPIO_HI_PIN_START {
                Register::new(SIO_GPIO_HI_OUT_SET)
            } else {
                Register::new(SIO_GPIO_OUT_SET)
            };
            let registerClear = if pin >= GPIO_HI_PIN_START {
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
                registerClear.bitSet(bit, true);
            } else {
                registerSet.bitSet(bit, true);
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

        register.bitSet(bit, true);
    }

    fn read(&self, pin: u32) -> bool {
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

            return if register.bitGet(bit) {
                true
            } else {
                false
            }
        }
        return false;
    }

    fn getPinCapabilities(&self) -> &'static [PinCapability] {
        return PIN_CAPABILITIES;
    }
}

impl _GPIODriver {
    pub fn new() -> Self {
        Self
    }
}