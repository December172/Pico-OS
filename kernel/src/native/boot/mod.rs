mod runtime;

#[cfg(board_chip = "RP2040")]
pub mod rp2040;

// Used for kernel init() - lowlevel init entry interface
#[cfg(board_chip = "RP2040")]
pub use crate::native::boot::rp2040::_init;
