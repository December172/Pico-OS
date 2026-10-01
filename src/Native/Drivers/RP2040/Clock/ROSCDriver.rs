use crate::Kernel::Drivers::ClockDriver::*;

use crate::Native::Drivers::RP2040::Clock::PLLDriver::*;
use crate::Native::Constants::RP2040::ROSC::*;
use crate::Native::Constants::RP2040::CLOCKS::*;
use crate::Native::Constants::RP2040::Config::ROSC_BASE_FREQ;
use crate::Util::Register::Register;

pub struct _ROSCDriver {
    roscCtrl    : Register,
    roscStatus  : Register,
}

impl ClockDriver for _ROSCDriver {
    fn init(&self) -> bool {
        // Always available, skipping ROSC initialization
        // without precise source, skipping init of plls
        return true;
    }

    fn enable(&self)  -> bool {
        let clockRefCtrl = Register::new(CLOCKS_CLK_REF_CTRL);
        let clockSysCtrl = Register::new(CLOCKS_CLK_SYS_CTRL);
        let clockPeriCtrl = Register::new(CLOCKS_CLK_PERI_CTRL);

        // initialize clocks, use non-precise ROSC
        clockRefCtrl.fieldSet(CLOCKS_CLK_REF_CTRL_SRC_HIGH, 
                               CLOCKS_CLK_REF_CTRL_SRC_LOW,
                                CLOCKS_CLK_REF_CTRL_SRC_ROSC_CLKSRC_PH);
        clockSysCtrl.bitSet(CLOCKS_CLK_SYS_CTRL_SRC_BIT, false);
        clockPeriCtrl.fieldSet(CLOCKS_CLK_PERI_CTRL_AUXSRC_HIGH, 
                                CLOCKS_CLK_PERI_CTRL_AUXSRC_LOW,
                                 CLOCKS_CLK_PERI_CTRL_AUXSRC_CLK_SYS);
        clockPeriCtrl.bitSet(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, true);
        return true;
    }

    fn disable(&self, domain: ClockDomain) {
        match domain {
            // clk_sys & clk_ref cannot be disabled
            ClockDomain::Reference => return (),
            ClockDomain::System => return (),
            ClockDomain::Peripherals => {
                        let clockPeriCtrl = Register::new(CLOCKS_CLK_PERI_CTRL);
                        clockPeriCtrl.bitSet(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, false);
            },
            // since clk_usb is not enabled under ROSCDriver - maybe this can be skipped?
            ClockDomain::USB => {
                let clockUSBCtrl = Register::new(CLOCKS_CLK_USB_CTRL);
                clockUSBCtrl.bitSet(CLOCKS_CLK_CTRL_ENABLE_BIT, false);
            },
        }
    }

    fn isPrecise(&self) -> bool {
        return false;
    }

    fn getFrequency(&self, domain: ClockDomain) -> u32 {
        match domain {
            ClockDomain::Reference => return ROSC_BASE_FREQ,
            ClockDomain::System => return ROSC_BASE_FREQ,
            ClockDomain::Peripherals => return ROSC_BASE_FREQ,
            ClockDomain::USB => return 0,
        }
    }
}

impl _ROSCDriver {
    pub fn new() -> Self {
        Self {
            roscCtrl : Register::new(ROSC_CTRL),
            roscStatus : Register::new(ROSC_STATUS),
        }
    }
}

