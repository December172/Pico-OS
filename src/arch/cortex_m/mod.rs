pub mod general;

#[cfg(feature = "RP2040")]
pub mod cortex_m0p;

#[cfg(feature = "RP2040")]
pub use cortex_m0p as current;
