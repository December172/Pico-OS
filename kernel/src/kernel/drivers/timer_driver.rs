pub trait TimerDriver {
    /// Return current hardware ticks
    fn now_tick(&self) -> u64;

    /// Bring up the hardware time base that `now_tick()` counts on.
    ///
    /// `ref_freq` is the frequency, in Hz, of the clock the tick generator is
    /// referenced to (RP2040: clk_ref) - *not* the core/system clock. Feeding
    /// it the system clock scales every delay by ref_freq/system_freq.
    fn init(&self, ref_freq: u32) -> bool;
}
