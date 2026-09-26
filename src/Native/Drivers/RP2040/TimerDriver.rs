use crate::HAL::Timer;
use crate::Util::Register::Register;
use crate::Kernel::Drivers::TimerDriver::TimerDriver;

use crate::Native::Constants::RP2040::TIMER::*;

pub struct _TimerDriver {
    timehr : Register,
    timelr : Register,

}

impl TimerDriver for _TimerDriver {
    fn nowTick(&self) -> u64 {
        // read low bits first, then high bits; to ensure tick consistency
        let low = self.timelr.read();
        let high = self.timehr.read();
        // Combine high bits and low bits
        return ((high as u64) << 32) | (low as u64);
    }
}

impl _TimerDriver {
    pub fn new() -> Self {
        Self {
            timehr : Register::new(TIMER_TIMEHR),
            timelr : Register::new(TIMER_TIMELR),
        }
    }
}