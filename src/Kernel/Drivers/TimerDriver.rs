pub trait TimerDriver {
    /// Return current hardware ticks
    fn nowTick(&self) -> u64;
}