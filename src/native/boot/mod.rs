mod runtime;

#[cfg(feature = "RP2040")]
pub mod rp2040;

// Used for kernel init() - lowlevel init entry interface
#[cfg(feature = "RP2040")]
pub use crate::native::boot::rp2040::_init;
