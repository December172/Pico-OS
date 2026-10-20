use crate::kernel::drivers::clock_driver::*;
use crate::util::low_level::_poll;
use crate::util::register::Register;

use crate::native::constants::rp2040::clocks::*;
use crate::native::constants::rp2040::config::XOSC_BASE_FREQ;
use crate::native::constants::rp2040::xosc::*;
use crate::native::drivers::rp2040::clock::pll_driver::*;

pub struct _XOSCDriver {
    pll_sys_driver: PLLDriver,
    pll_usb_driver: PLLDriver,

    xosc_ctrl: Register,
    xosc_status: Register,
    xosc_startup: Register,
}

impl ClockDriver for _XOSCDriver {
    fn init(&self) -> bool {
        // Program the crystal startup delay BEFORE enabling the oscillator.
        // With the reset value (0) the STABLE flag can assert almost
        // immediately - i.e. before the crystal actually oscillates. clk_ref
        // is then switched onto xosc, so a not-yet-running crystal leaves
        // clk_ref dead and the watchdog tick / timer stop counting.
        // Recommended delay: (freq_Hz / 1000) + 128 xosc cycles.
        let startup_delay: u32 = (XOSC_BASE_FREQ / 1000) + 128;
        self.xosc_startup.field_set(
            XOSC_STARTUP_DELAY_HIGH,
            XOSC_STARTUP_DELAY_LOW,
            startup_delay,
        );

        self.xosc_ctrl.field_set(
            XOSC_CTRL_FREQ_RANGE_HIGH,
            XOSC_CTRL_FREQ_RANGE_LOW,
            XOSC_CTRL_FREQ_RANGE_1_15MHZ,
        );
        self.xosc_ctrl.field_set(
            XOSC_CTRL_ENABLE_HIGH,
            XOSC_CTRL_ENABLE_LOW,
            XOSC_CTRL_ENABLE_ENABLE,
        );

        // give the crystal enough time to come up before trusting STABLE.
        if !_poll(100_000, || self.xosc_status.bit_get(XOSC_STATUS_STABLE_BIT)) {
            return false;
        }

        // set up clk_ref source to xosc
        let clock_ref_ctrl = Register::new(CLOCKS_CLK_REF_CTRL);
        let clock_ref_selected = Register::new(CLOCKS_CLK_REF_SELECTED);
        clock_ref_ctrl.field_set(
            CLOCKS_CLK_REF_CTRL_SRC_HIGH,
            CLOCKS_CLK_REF_CTRL_SRC_LOW,
            CLOCKS_CLK_REF_CTRL_SRC_XOSC_CLKSRC,
        );

        // check if xosc died
        if !_poll(10_000, || {
            clock_ref_selected.bit_get(CLOCKS_CLK_REF_CTRL_SRC_XOSC_CLKSRC)
        }) || !self.xosc_status.bit_get(XOSC_STATUS_STABLE_BIT)
        {
            // the cleanup work will be done at ROSCDriver.init();
            return false;
        }

        // init pll_usb & pll_sys
        if !self.pll_sys_driver.init(XOSC_BASE_FREQ) {
            return false;
        }
        if !self.pll_usb_driver.init(XOSC_BASE_FREQ) {
            return false;
        }

        // verify all plls
        if (!self.pll_sys_driver.is_locked() || !self.pll_usb_driver.is_locked()) {
            return false;
        }

        return true;
    }

    fn enable(&self) -> bool {
        // Assuming that the plls is initialized
        // init clk_sys
        let clock_sys_ctrl = Register::new(CLOCKS_CLK_SYS_CTRL);
        clock_sys_ctrl.field_set(
            CLOCKS_CLK_SYS_CTRL_AUXSRC_HIGH,
            CLOCKS_CLK_SYS_CTRL_AUXSRC_LOW,
            CLOCKS_CLK_SYS_CTRL_AUXSRC_CLKSRC_PLL_SYS,
        );
        clock_sys_ctrl.bit_set(CLOCKS_CLK_SYS_CTRL_SRC_BIT, true);

        // init clk_peri
        let clock_peri_ctrl = Register::new(CLOCKS_CLK_PERI_CTRL);
        clock_peri_ctrl.field_set(
            CLOCKS_CLK_PERI_CTRL_AUXSRC_HIGH,
            CLOCKS_CLK_PERI_CTRL_AUXSRC_LOW,
            CLOCKS_CLK_PERI_CTRL_AUXSRC_CLKSRC_PLL_SYS,
        );
        clock_peri_ctrl.bit_set(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, true);

        // init clk_usb
        let clock_usb_ctrl = Register::new(CLOCKS_CLK_USB_CTRL);
        clock_usb_ctrl.field_set(
            CLOCKS_CLK_CTRL_AUXSRC_HIGH,
            CLOCKS_CLK_CTRL_AUXSRC_LOW,
            CLOCKS_CLK_CTRL_AUXSRC_CLKSRC_PLL_USB,
        );
        clock_usb_ctrl.bit_set(CLOCKS_CLK_CTRL_ENABLE_BIT, true);

        return self.verify_clocks();
    }

    fn disable(&self, domain: ClockDomain) {
        // TODO: Implement disable xosc source itself, not only clk_*
        match domain {
            // clk_sys & clk_ref cannot be disabled
            ClockDomain::Reference => return (),
            ClockDomain::System => return (),
            ClockDomain::Peripherals => {
                let clock_peri_ctrl = Register::new(CLOCKS_CLK_PERI_CTRL);
                clock_peri_ctrl.bit_set(CLOCKS_CLK_PERI_CTRL_ENABLE_BIT, false);
            }
            ClockDomain::Usb => {
                let clock_usb_ctrl = Register::new(CLOCKS_CLK_USB_CTRL);
                clock_usb_ctrl.bit_set(CLOCKS_CLK_CTRL_ENABLE_BIT, false);
            }
        }
    }

    fn is_precise(&self) -> bool {
        return true;
    }

    /// Return corresponded clock's frequency in Hz
    fn get_frequency(&self, domain: ClockDomain) -> u32 {
        match domain {
            ClockDomain::Reference => return XOSC_BASE_FREQ,
            ClockDomain::System => return self.pll_sys_driver.get_frequency(XOSC_BASE_FREQ),
            ClockDomain::Peripherals => return self.pll_sys_driver.get_frequency(XOSC_BASE_FREQ),
            ClockDomain::Usb => return self.pll_usb_driver.get_frequency(XOSC_BASE_FREQ),
        }
    }
}

impl _XOSCDriver {
    pub fn new() -> Self {
        Self {
            pll_sys_driver: PLLDriver::new(PLLCONFIG_SYSTEM),
            pll_usb_driver: PLLDriver::new(PLLCONFIG_USB),
            xosc_ctrl: Register::new(XOSC_CTRL),
            xosc_status: Register::new(XOSC_STATUS),
            xosc_startup: Register::new(XOSC_STARTUP),
        }
    }

    /// Configures the frequency counter (fc0) to measure clock `src`,
    /// waits for the measurement to complete and checks it did not fail.
    /// `expected` is the frequency written to the fc0 reference/min/max
    /// registers, with an allowed tolerance of 1%.
    fn measure_frequency(&self, src: u32, expected_khz: u32) -> bool {
        let fc0_status = Register::new(CLOCKS_FC0_STATUS);
        let fc0_ref_khz = Register::new(CLOCKS_FC0_REF_KHZ);
        let fc0_min_khz = Register::new(CLOCKS_FC0_MIN_KHZ);
        let fc0_max_khz = Register::new(CLOCKS_FC0_MAX_KHZ);
        let fc0_src = Register::new(CLOCKS_FC0_SRC);

        let ref_khz = XOSC_BASE_FREQ / 1000;
        let tolerance_khz = expected_khz / 100;

        // FC0 reference is always CLK_REF.
        fc0_ref_khz.field_set(CLOCKS_FC0_REF_KHZ_HIGH, CLOCKS_FC0_REF_KHZ_LOW, ref_khz);

        // Expected frequency of the clock being measured.
        fc0_min_khz.field_set(
            CLOCKS_FC0_MIN_KHZ_HIGH,
            CLOCKS_FC0_MIN_KHZ_LOW,
            expected_khz - tolerance_khz,
        );

        fc0_max_khz.field_set(
            CLOCKS_FC0_MAX_KHZ_HIGH,
            CLOCKS_FC0_MAX_KHZ_LOW,
            expected_khz + tolerance_khz,
        );

        // Select clock to measure.
        fc0_src.field_set(CLOCKS_FC0_SRC_HIGH, CLOCKS_FC0_SRC_LOW, src);

        // Wait for measurement to finish.
        if !_poll(10000, || fc0_status.bit_get(CLOCKS_FC0_STATUS_DONE_BIT)) {
            return false;
        }

        // Frequency outside the configured range.
        if fc0_status.bit_get(CLOCKS_FC0_STATUS_FAIL_BIT) {
            return false;
        }

        return true;
    }

    fn verify_clocks(&self) -> bool {
        {
            let clock_sys_ctrl = Register::new(CLOCKS_CLK_SYS_CTRL);
            let status = Register::new(CLOCKS_CLK_SYS_SELECTED);
            if !_poll(1000, || {
                return (clock_sys_ctrl.field_get(
                    CLOCKS_CLK_SYS_CTRL_AUXSRC_HIGH,
                    CLOCKS_CLK_SYS_CTRL_AUXSRC_LOW,
                ) == CLOCKS_CLK_SYS_CTRL_AUXSRC_CLKSRC_PLL_SYS)
                    && (status.bit_get(CLOCKS_CLK_SYS_CTRL_SRC_CLKSRC_CLK_SYS_AUX));
            }) {
                return false;
            }
        }
        {
            let status = Register::new(CLOCKS_CLK_PERI_SELECTED);
            if !_poll(10000, || {
                status.bit_get(CLOCKS_CLK_PERI_CTRL_AUXSRC_CLKSRC_PLL_SYS)
            }) {
                return false;
            }
        }
        let expected_xosc = XOSC_BASE_FREQ / 1000;
        let expected_pll_sys = self.pll_sys_driver.get_frequency(XOSC_BASE_FREQ) / 1000;
        let expected_pll_usb = self.pll_usb_driver.get_frequency(XOSC_BASE_FREQ) / 1000;

        return self.measure_frequency(CLOCKS_FC0_SRC_XOSC_CLKSRC, expected_xosc)
            && self.measure_frequency(CLOCKS_FC0_SRC_CLK_REF, expected_xosc)
            && self.measure_frequency(CLOCKS_FC0_SRC_PLL_SYS_CLKSRC_PRIMARY, expected_pll_sys)
            && self.measure_frequency(CLOCKS_FC0_SRC_PLL_USB_CLKSRC_PRIMARY, expected_pll_usb)
            && self.measure_frequency(CLOCKS_FC0_SRC_CLK_SYS, expected_pll_sys)
            && self.measure_frequency(CLOCKS_FC0_SRC_CLK_PERI, expected_pll_sys)
            && self.measure_frequency(CLOCKS_FC0_SRC_CLK_USB, expected_pll_usb);
    }
}
