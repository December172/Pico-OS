use crate::util::low_level::_poll;
use crate::util::register::Register;

use crate::native::constants::config::*;
use crate::native::constants::rp2040::pll::*;

#[derive(Clone, Copy)]
pub struct PLLConfig {
    pub cs: u32,
    pub pwr: u32,
    pub fbdiv_int: u32,
    pub prim: u32,
    pub fbdiv: u32,
    pub postdiv1: u32,
    pub postdiv2: u32,
    pub refdiv: u32,
}

pub const PLLCONFIG_SYSTEM: PLLConfig = PLLConfig {
    cs: PLL_SYS_CS,
    pwr: PLL_SYS_PWR,
    fbdiv_int: PLL_SYS_FBDIV_INT,
    prim: PLL_SYS_PRIM,
    fbdiv: PLL_SYS_FBDIV,
    postdiv1: PLL_SYS_POSTDIV1,
    postdiv2: PLL_SYS_POSTDIV2,
    refdiv: 0x1,
};

pub const PLLCONFIG_USB: PLLConfig = PLLConfig {
    cs: PLL_USB_CS,
    pwr: PLL_USB_PWR,
    fbdiv_int: PLL_USB_FBDIV_INT,
    prim: PLL_USB_PRIM,
    fbdiv: PLL_USB_FBDIV,
    postdiv1: PLL_USB_POSTDIV1,
    postdiv2: PLL_USB_POSTDIV2,
    refdiv: 0x1,
};

pub struct PLLDriver {
    config: PLLConfig,
    cs: Register,
    pwr: Register,
    fbdiv_int: Register,
    prim: Register,
}

impl PLLDriver {
    pub fn new(config: PLLConfig) -> Self {
        Self {
            config,
            cs: Register::new(config.cs),
            pwr: Register::new(config.pwr),
            fbdiv_int: Register::new(config.fbdiv_int),
            prim: Register::new(config.prim),
        }
    }

    pub fn init(&self, source_freq: u32) -> bool {
        /// Note that all fields with suffix `_PD` mean powerdown - `true` for disable, and `false` for enable
        self.cs
            .field_set(PLL_CS_REFDIV_HIGH, PLL_CS_REFDIV_LOW, self.config.refdiv);

        // set 125MHz for fbdiv
        self.fbdiv_int.write(self.config.fbdiv);

        // power up vco & pll
        self.pwr.bit_set(PLL_PWR_PD_BIT, false);
        self.pwr.bit_set(PLL_PWR_VCOPD_BIT, false);

        // Return to rosc if lock failed
        if !_poll(1_000_000, || self.cs.bit_get(PLL_CS_LOCK_BIT)) {
            return false;
        }

        self.prim.field_set(
            PLL_PRIM_POSTDIV1_HIGH,
            PLL_PRIM_POSTDIV1_LOW,
            self.config.postdiv1,
        );
        self.prim.field_set(
            PLL_PRIM_POSTDIV2_HIGH,
            PLL_PRIM_POSTDIV2_LOW,
            self.config.postdiv2,
        );

        self.pwr.bit_set(PLL_PWR_POSTDIVPD_BIT, false);

        return true;
    }

    pub fn is_locked(&self) -> bool {
        return self.cs.bit_get(PLL_CS_LOCK_BIT);
    }

    pub fn get_frequency(&self, source_freq: u32) -> u32 {
        // From RP2040 datasheet
        return (source_freq / self.config.refdiv) * self.config.fbdiv
            / (self.config.postdiv1 * self.config.postdiv2);
    }
}
