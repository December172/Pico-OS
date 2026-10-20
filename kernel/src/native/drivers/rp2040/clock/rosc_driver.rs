use crate::kernel::drivers::clock_driver::*;

use crate::native::constants::rp2040::clocks::*;
use crate::native::constants::rp2040::config::ROSC_BASE_FREQ;
use crate::native::constants::rp2040::rosc::*;
use crate::native::drivers::rp2040::clock::pll_driver::*;
use crate::util::register::Register;

pub struct _ROSCDriver {
    rosc_ctrl: Register,
    rosc_status: Register,
}

impl ClockDriver for _ROSCDriver {
    fn init(&self) -> bool {
        // Always available, skipping rosc initialization
        // without precise source, skipping init of plls
        return true;
    }

    fn enable(&self) -> bool {
        let clock_ref_ctrl = Register::new(CLOCKS_CLK_REF_CTRL);
        let clock_sys_ctrl = Register::new(CLOCKS_CLK_SYS_CTRL);
        let clock_peri_ctrl = Register::new(CLOCKS_CLK_PERI_CTRL);

        // initialize clocks, use non-precise rosc
        clock_ref_ctrl.field_set(
            CLOCKS_CLK_REF_CTRL_SRC_HIGH,
            CLOCKS_CLK_REF_CTRL_SRC_LOW,
            CLOCKS_CLK_REF_CTRL_SRC_ROSC_CLKSRC_PH,
        );
        clock_sys_ctrl.bit_set(CLOCKS_CLK_SYS_CTRL_SRC_BIT, false);
        clock_peri_ctrl.field_set(
            CLOCKS_CLK_PERI_CTRL_AUXSRC_HIGH,
            CLOCKS_CLK_PERI_CTRL_AUXSRC_LOW,
            CLOCKS_CLK_PERI_CTRL_AUXSRC_CLK_SYS,
        );
        clock_peri_ctrl.bit_set(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, true);
        return true;
    }

    fn disable(&self, domain: ClockDomain) {
        match domain {
            // clk_sys & clk_ref cannot be disabled
            ClockDomain::Reference => return (),
            ClockDomain::System => return (),
            ClockDomain::Peripherals => {
                let clock_peri_ctrl = Register::new(CLOCKS_CLK_PERI_CTRL);
                clock_peri_ctrl.bit_set(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, false);
            }
            // since clk_usb is not enabled under ROSCDriver - maybe this can be skipped?
            ClockDomain::Usb => {
                let clock_usb_ctrl = Register::new(CLOCKS_CLK_USB_CTRL);
                clock_usb_ctrl.bit_set(CLOCKS_CLK_CTRL_ENABLE_BIT, false);
            }
        }
    }

    fn is_precise(&self) -> bool {
        return false;
    }

    fn get_frequency(&self, domain: ClockDomain) -> u32 {
        match domain {
            ClockDomain::Reference => return ROSC_BASE_FREQ,
            ClockDomain::System => return ROSC_BASE_FREQ,
            ClockDomain::Peripherals => return ROSC_BASE_FREQ,
            ClockDomain::Usb => return 0,
        }
    }
}

impl _ROSCDriver {
    pub fn new() -> Self {
        Self {
            rosc_ctrl: Register::new(ROSC_CTRL),
            rosc_status: Register::new(ROSC_STATUS),
        }
    }
}
