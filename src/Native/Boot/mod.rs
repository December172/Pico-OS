#[cfg(feature = "Pico_16M")]
pub mod RP2040;

#[cfg(feature = "RP2350")]
pub mod RP2350;

// Used for kernel init() - lowlevel init entry interface
#[cfg(feature = "RP2040")]
pub use crate::Native::Boot::RP2040::_init;

#[cfg(feature = "RP2350")]
pub use crate::Native::Boot::RP2350::_init;