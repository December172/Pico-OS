#[cfg(feature = "RP2040")]
pub mod RP2040;

#[cfg(feature = "RP2350")]
pub mod RP2350;

#[cfg(feature = "RP2040")]
pub use RP2040::Config;

#[cfg(feature = "RP2350")]
pub use RP2350::Config;