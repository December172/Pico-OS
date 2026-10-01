use crate::Util::Register::Register;
use crate::Util::LowLevel::_poll;

use crate::Native::Constants::Config::*;
use crate::Native::Constants::RP2350::PLL::*;

#[derive(Clone, Copy)]
pub struct PLLConfig {
    pub cs: u32,
    pub pwr: u32,
    pub fbdivInt: u32,
    pub prim: u32,
    pub fbdiv: u32,
    pub postdiv1: u32,
    pub postdiv2: u32,
    pub refdiv: u32,
}

pub const PLLCONFIG_SYSTEM : PLLConfig = PLLConfig {
    cs: PLL_SYS_CS,
    pwr: PLL_SYS_PWR,
    fbdivInt: PLL_SYS_FBDIV_INT,
    prim: PLL_SYS_PRIM,
    fbdiv: PLL_SYS_FBDIV,
    postdiv1: PLL_SYS_POSTDIV1,
    postdiv2: PLL_SYS_POSTDIV2,
    refdiv: 0x1
};

pub const PLLCONFIG_USB : PLLConfig = PLLConfig {
    cs: PLL_USB_CS,
    pwr: PLL_USB_PWR,
    fbdivInt: PLL_USB_FBDIV_INT,
    prim: PLL_USB_PRIM,
    fbdiv: PLL_USB_FBDIV,
    postdiv1: PLL_USB_POSTDIV1,
    postdiv2: PLL_USB_POSTDIV2,
    refdiv: 0x1
};

pub struct PLLDriver {
    config: PLLConfig,
    cs : Register,
    pwr : Register,
    fbdiv_int : Register,
    prim : Register,
}

impl PLLDriver {
    pub fn new(config: PLLConfig) -> Self {
        Self {
            config,
            cs : Register::new(config.cs),
            pwr : Register::new(config.pwr),
            fbdiv_int : Register::new(config.fbdivInt),
            prim : Register::new(config.prim),
        }
    }

    pub fn init(&self, sourceFreq: u32) -> bool {
        self.cs.fieldSet(PLL_CS_REFDIV_HIGH,PLL_CS_REFDIV_LOW, self.config.refdiv);

        self.fbdiv_int.write(self.config.fbdiv);

        self.pwr.bitSet(PLL_PWR_PD_BIT, false);
        self.pwr.bitSet(PLL_PWR_VCOPD_BIT, false);

        if !_poll(1_000_000, || self.cs.bitGet(PLL_CS_LOCK_BIT)) {
            return false;
        }

        self.prim.fieldSet(PLL_PRIM_POSTDIV1_HIGH, PLL_PRIM_POSTDIV1_LOW, self.config.postdiv1);
        self.prim.fieldSet(PLL_PRIM_POSTDIV2_HIGH, PLL_PRIM_POSTDIV2_LOW, self.config.postdiv2);

        self.pwr.bitSet(PLL_PWR_POSTDIVPD_BIT, false);

        return true;
    }

    pub fn isLocked(&self) -> bool {
        return self.cs.bitGet(PLL_CS_LOCK_BIT);
    }

    pub fn getFrequency(&self, sourceFreq: u32) -> u32 {
        return (sourceFreq / self.config.refdiv) * self.config.fbdiv / (self.config.postdiv1 * self.config.postdiv2);
    }
}
