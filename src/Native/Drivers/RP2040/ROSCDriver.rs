use crate::Kernel::Drivers::ClockDriver::*;

use crate::Native::Drivers::RP2040::PLLDriver::*;
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
                                CLOCKS_CLK_REF_ROSC_PH_SRC);
        clockSysCtrl.bitSet(CLOCKS_CLK_SYS_CTRL_SRC_BIT, false);
        clockPeriCtrl.fieldSet(CLOCKS_CLK_PERI_CTRL_AUXSRC_HIGH, 
                                CLOCKS_CLK_PERI_CTRL_AUXSRC_LOW,
                                 CLOCKS_CLK_PERI_CLK_SYS_AUXSOURCE);
        clockPeriCtrl.bitSet(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, true);
        return true;
    }

    fn disable(&self, domain: ClockDomain) {
        match domain {
            // clk_ref must run continuously, it cannot be disabled.
            ClockDomain::Reference => return (),
            ClockDomain::System => return (),
            ClockDomain::Peripherals => {
                        let clockPeriCtrl = Register::new(CLOCKS_CLK_PERI_CTRL);
                        clockPeriCtrl.bitSet(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, false);
            },
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

