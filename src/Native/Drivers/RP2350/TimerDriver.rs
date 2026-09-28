use crate::Util::Register::Register;
use crate::Util::LowLevel::_poll;
use crate::Kernel::Drivers::TimerDriver::TimerDriver;

use crate::Native::Constants::RP2350::RESETS::*;
use crate::Native::Constants::RP2350::TIMER::*;
use crate::Native::Constants::RP2350::TICKS::*;

pub struct _TimerDriver {
    timehr : Register,
    timelr : Register,

    // The 1us reference lives in the TICKS block on RP2350.
    ticksTimerCtrl : Register,
    ticksTimerCycles : Register,
}

impl TimerDriver for _TimerDriver {
    fn nowTick(&self) -> u64 {
        let low = self.timelr.read();
        let high = self.timehr.read();
        return ((high as u64) << 32) | (low as u64);
    }

    fn init(&self, refFreq: u32) -> bool {
        // `refFreq` is the clk_ref frequency: the tick generator divides clk_ref
        // down to the 1us reference.
        self.ticksTimerCycles.fieldSet(TICKS_TIMER_CYCLES_HIGH,
                                       TICKS_TIMER_CYCLES_LOW,
                                       refFreq / 1_000_000);

        self.ticksTimerCtrl.bitSet(TICKS_CTRL_ENABLE_BIT, true);
        if !_poll(100_000, || self.ticksTimerCtrl.bitGet(TICKS_CTRL_RUNNING_BIT)) {
            return false;
        }

        let reset = Register::new(RESETS_RESET);
        let resetDone = Register::new(RESETS_RESET_DONE);
        reset.bitSet(RESETS_RESET_TIMER0_BIT, true);
        reset.bitSet(RESETS_RESET_TIMER0_BIT, false);
        if !_poll(100_000, || resetDone.bitGet(RESETS_RESET_DONE_TIMER0_BIT)) {
            return false;
        }

        let timerPause = Register::new(TIMER0_PAUSE);
        let timerDbgPause = Register::new(TIMER0_DBGPAUSE);
        let timerSource = Register::new(TIMER0_SOURCE);
        timerPause.write(0);
        timerDbgPause.write(0);
        // SOURCE = TICK, so TIMER0 counts the 1us tick.
        timerSource.bitSet(TIMER_SOURCE_CLK_SYS_BIT, false);
        if timerDbgPause.read() != 0 {
            return false;
        }

        return _poll(100_000, || self.nowTick() != 0);
    }
}

impl _TimerDriver {
    pub fn new() -> Self {
        Self {
            timehr : Register::new(TIMER0_TIMEHR),
            timelr : Register::new(TIMER0_TIMELR),
            ticksTimerCtrl : Register::new(TICKS_TIMER_CTRL(0)),
            ticksTimerCycles : Register::new(TICKS_TIMER_CYCLES(0)),
        }
    }
}
