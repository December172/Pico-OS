use crate::Native::Constants::Config::*;
use crate::Native::Constants::RP2040::PLL::*;
use crate::Util::Register::Register;

#[derive(Clone, Copy)]
pub struct PLLConfig {
    pub baseAddr: u32,
    pub fbdiv: u32,
    pub postdiv1: u32,
    pub postdiv2: u32,
    pub refdiv: u32,
}

pub const PLLCONFIG_SYSTEM : PLLConfig = PLLConfig {
    baseAddr : PLL_SYS_BASE,
    fbdiv: PLL_SYS_FBDIV,
    postdiv1: PLL_SYS_POSTDIV1,
    postdiv2: PLL_SYS_POSTDIV2,
    refdiv: 0x1
};

pub const PLLCONFIG_USB : PLLConfig = PLLConfig {
    baseAddr : PLL_USB_BASE,
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
            cs : Register::new(config.baseAddr + PLL_CS_OFFSET),
            pwr : Register::new(config.baseAddr + PLL_PWR_OFFSET),
            fbdiv_int : Register::new(config.baseAddr + PLL_FBDIV_INT_OFFSET),
            prim : Register::new(config.baseAddr + PLL_PRIM_OFFSET),
        }
    }

    /// TODO: Putting these fxxking reg shifts to corresponding constants files
    pub fn init(&self, sourceFreq: u32) {
        self.cs.fieldSet(PLL_CS_REFDIV_HIGH,PLL_CS_REFDIV_LOW, self.config.refdiv);

        // set 125MHz for fbdiv
        self.fbdiv_int.write(self.config.fbdiv);

        // power up vco & pll
        self.pwr.bitSet(PLL_PWR_PD_BIT, true);
        self.pwr.bitSet(PLL_PWR_VCOPD_BIT, true);

        
        while self.cs.bitGet(PLL_CS_LOCK_BIT) {
            // Stub here
            // not sure whether to make it noreturn
        }
        self.prim.fieldSet(PLL_PRIM_POSTDIV1_HIGH, PLL_PRIM_POSTDIV1_LOW, self.config.postdiv1);
        self.prim.fieldSet(PLL_PRIM_POSTDIV2_HIGH, PLL_PRIM_POSTDIV2_LOW, self.config.postdiv2);

        // power up post dividers
        let postdivShift: u32 = 0x3;
        self.prim.bitSet(postdivShift, true);
    }

    pub fn isLocked(&self) -> bool {
        return self.cs.bitGet(PLL_CS_LOCK_BIT);
    }

    pub fn getFrequency(&self, sourceFreq: u32) -> u32 {
        // From RP2040 datasheet
        return (sourceFreq / self.config.refdiv) * self.config.fbdiv / (self.config.postdiv1 * self.config.postdiv2); 
    }
}