#[cfg(feature = "Pico_16M")]
pub mod RP2040;

// Used for kernel init() - lowlevel init entry interface
#[cfg(feature = "RP2040")]
pub use crate::Native::Boot::RP2040::_init;