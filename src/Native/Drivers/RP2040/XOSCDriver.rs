use crate::Util::Register::Register;
use crate::Kernel::Drivers::ClockDriver::*;

use crate::Native::Constants::RP2040::CLOCKS::*;
use crate::Native::Constants::RP2040::Config::XOSC_BASE_FREQ;
use crate::Native::Constants::RP2040::PLL::PLL_SYS_BASE;
use crate::Native::Constants::RP2040::PLL::PLL_USB_BASE;
use crate::Native::Constants::RP2040::XOSC::*;
use crate::Native::Drivers::RP2040::PLLDriver::*;

pub struct _XOSCDriver {
    pllSysDriver : PLLDriver,
    pllUSBDriver : PLLDriver,

    xoscCtrl    : Register,
    xoscStatus  : Register,
    xoscStartup : Register
}

/// private tool poll()
/// used before clock system successfully initialized
/// at this point, we cannot trust hardware tick counter
fn poll(count: u32, func: impl Fn() -> bool) -> bool {
    let mut counter = count;
    while !func() {
        counter -= 1;
        core::hint::spin_loop();
        if counter == 0 {
            return false;
        }
    }
    return true;
}

impl ClockDriver for _XOSCDriver {
    fn init(&self) -> bool {
        // XOSC enable
        self.xoscCtrl.write(XOSC_CTRL_ENABLE_ENABLE);
        // TODO: Figure out exact startup delay
        if !poll(10000, || self.xoscStatus.bitGet(XOSC_STATUS_STABLE_BIT)) {
            return false;
        }

        // init clk_ref
        let clockRefCtrl = Register::new(CLOCKS_CLK_REF_CTRL);
        clockRefCtrl.fieldSet(CLOCKS_CLK_REF_CTRL_SRC_HIGH, 
                               CLOCKS_CLK_REF_CTRL_SRC_LOW,
                                CLOCKS_CLK_REF_XOSC_SRC);
        {
            let status = Register::new(CLOCKS_CLK_REF_SELECTED);
            if !poll(10000, || status.bitGet(CLOCKS_CLK_REF_XOSC_SRC)) {
                return false;
            }
        }

        // init pll_usb & pll_sys
        self.pllSysDriver.init(XOSC_BASE_FREQ);
        self.pllUSBDriver.init(XOSC_BASE_FREQ);

        // verify all plls
        if (!self.pllSysDriver.isLocked() || !self.pllUSBDriver.isLocked()) {
            return false;
        }

        return true;
    }

    fn enable(&self) -> bool {
        // Assuming that the plls is initialized
        // init clk_sys
        let clockSysCtrl = Register::new(CLOCKS_CLK_SYS_CTRL);
        clockSysCtrl.fieldSet(CLOCKS_CLK_SYS_CTRL_AUXSRC_HIGH, 
                               CLOCKS_CLK_SYS_CTRL_AUXSRC_LOW,
                                CLOCKS_CLK_SYS_PLL_SYS_AUXSOURCE);
        clockSysCtrl.bitSet(CLOCKS_CLK_SYS_CTRL_SRC_BIT, true);


        // init clk_peri
        let clockPeriCtrl = Register::new(CLOCKS_CLK_PERI_CTRL);
        clockPeriCtrl.fieldSet(CLOCKS_CLK_PERI_CTRL_AUXSRC_HIGH, 
                                CLOCKS_CLK_PERI_CTRL_AUXSRC_LOW,
                                 CLOCKS_CLK_PERI_PLL_SYS_AUXSOURCE);
        clockPeriCtrl.bitSet(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, true);


        // init clk_usb
        let clockUSBCtrl = Register::new(CLOCKS_CLK_USB_CTRL);
        clockUSBCtrl.fieldSet(CLOCKS_CLK_USB_CTRL_AUXSRC_HIGH, 
                               CLOCKS_CLK_USB_CTRL_AUXSRC_LOW, 
                                CLOCKS_CLK_USB_PLL_USB_AUXSOURCE);
        clockUSBCtrl.bitSet(CLOCKS_CLK_USB_CTRL_ENABLE_BIT, true);
        
        return self.verifyClocks();
    }

    fn disable(&self, domain: ClockDomain) {
        // TODO: Implement disable xosc source itself, not only clk_*
        match domain {
            ClockDomain::System => return (),
            ClockDomain::Peripherals => {
                        let clockPeriCtrl = Register::new(CLOCKS_CLK_PERI_CTRL);
                        clockPeriCtrl.bitSet(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, false);
            },
            ClockDomain::USB => {
                let clockUSBCtrl = Register::new(CLOCKS_CLK_USB_CTRL);
                clockUSBCtrl.bitSet(CLOCKS_CLK_USB_CTRL_ENABLE_BIT, false);
            },
        }
    }

    fn isPrecise(&self) -> bool {
        return true;
    }

    fn getFrequency(&self, domain: ClockDomain) -> u32 {
        match domain {
            ClockDomain::System => return self.pllSysDriver.getFrequency(XOSC_BASE_FREQ),
            ClockDomain::Peripherals => return self.pllSysDriver.getFrequency(XOSC_BASE_FREQ),
            ClockDomain::USB => return self.pllUSBDriver.getFrequency(XOSC_BASE_FREQ),
        }
    }


}

impl _XOSCDriver {
    pub fn new() -> Self {
        Self {
            pllSysDriver: PLLDriver::new(PLLCONFIG_SYSTEM),
            pllUSBDriver: PLLDriver::new(PLLCONFIG_USB),
            xoscCtrl:     Register::new(XOSC_CTRL),
            xoscStatus:   Register::new(XOSC_STATUS),
            xoscStartup:  Register::new(XOSC_STARTUP),
        }
    }

    /// Configures the frequency counter (fc0) to measure clock `src`,
    /// waits for the measurement to complete and checks it did not fail.
    /// `expected` is the frequency written to the fc0 reference/min/max
    /// registers, with an allowed tolerance of 1%.
    fn measureFrequency(&self, src: u32, expectedKHz: u32) -> bool {
        let fc0Status = Register::new(CLOCKS_FC0_STATUS);
        let fc0RefKHz = Register::new(CLOCKS_FC0_REF_KHZ);
        let fc0MinKHz = Register::new(CLOCKS_FC0_MIN_KHZ);
        let fc0MaxKHz = Register::new(CLOCKS_FC0_MAX_KHZ);
        let fc0Src = Register::new(CLOCKS_FC0_SRC);

        let refKHz = XOSC_BASE_FREQ / 1000;
        let toleranceKHz = expectedKHz / 100;

        // FC0 reference is always CLK_REF.
        fc0RefKHz.fieldSet(
            CLOCKS_FC0_REF_KHZ_HIGH,
            CLOCKS_FC0_REF_KHZ_LOW,
            refKHz
        );

        // Expected frequency of the clock being measured.
        fc0MinKHz.fieldSet(
            CLOCKS_FC0_MIN_KHZ_HIGH,
            CLOCKS_FC0_MIN_KHZ_LOW,
            expectedKHz - toleranceKHz
        );

        fc0MaxKHz.fieldSet(
            CLOCKS_FC0_MAX_KHZ_HIGH,
            CLOCKS_FC0_MAX_KHZ_LOW,
            expectedKHz + toleranceKHz
        );

        // Select clock to measure.
        fc0Src.fieldSet(
            CLOCKS_FC0_SRC_HIGH,
            CLOCKS_FC0_SRC_LOW,
            src
        );

        // Wait for measurement to finish.
        if !poll(10000, || {
            fc0Status.bitGet(CLOCKS_FC0_STATUS_DONE_BIT)
        }) {
            return false;
        }

        // Frequency outside the configured range.
        if fc0Status.bitGet(CLOCKS_FC0_STATUS_FAIL_BIT) {
            return false;
        }

        return true;
    }

    fn verifyClocks(&self) -> bool {
        {
            let clockSysCtrl = Register::new(CLOCKS_CLK_SYS_CTRL);
            let status = Register::new(CLOCKS_CLK_SYS_SELECTED);
            if !poll(1000, || {
                return (clockSysCtrl.fieldGet(CLOCKS_CLK_ADC_CTRL_AUXSRC_HIGH, CLOCKS_CLK_SYS_CTRL_AUXSRC_LOW) == CLOCKS_CLK_SYS_PLL_SYS_AUXSOURCE)
                       && (status.bitGet(CLOCKS_CLK_SYS_CLK_SYS_AUX_SRC));
            }) {
                return false;
            }
        }
        {
            let status = Register::new(CLOCKS_CLK_PERI_SELECTED);
            if !poll(10000, || status.bitGet(CLOCKS_CLK_PERI_PLL_SYS_AUXSOURCE)) {
                return false;
            }
        }
        let expectedXosc   = XOSC_BASE_FREQ / 1000;
        let expectedPllSys = self.pllSysDriver.getFrequency(XOSC_BASE_FREQ) / 1000;
        let expectedPllUsb = self.pllUSBDriver.getFrequency(XOSC_BASE_FREQ) / 1000;

        return self.measureFrequency(CLOCKS_FC_SRC_XOSC_CLKSRC, expectedXosc)
            && self.measureFrequency(CLOCKS_FC_SRC_CLK_REF, expectedXosc)
            && self.measureFrequency(CLOCKS_FC_SRC_PLL_SYS_CLKSRC_PRIMARY, expectedPllSys)
            && self.measureFrequency(CLOCKS_FC_SRC_PLL_USB_CLKSRC_PRIMARY, expectedPllUsb)
            && self.measureFrequency(CLOCKS_FC_SRC_CLK_SYS, expectedPllSys)
            && self.measureFrequency(CLOCKS_FC_SRC_CLK_PERI, expectedPllSys)
            && self.measureFrequency(CLOCKS_FC_SRC_CLK_USB, expectedPllUsb);     
    }
}