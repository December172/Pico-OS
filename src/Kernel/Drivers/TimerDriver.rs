pub trait TimerDriver {
    /// Return current hardware ticks
    fn nowTick(&self) -> u64;

    /// Bring up the hardware time base that `nowTick()` counts on.
    ///
    /// `refFreq` is the frequency, in Hz, of the clock the tick generator is
    /// referenced to (RP2040: clk_ref) - *not* the core/system clock. Feeding
    /// it the system clock scales every delay by refFreq/systemFreq.
    fn init(&self, refFreq: u32) -> bool;
}