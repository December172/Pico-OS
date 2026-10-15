pub mod General;

#[cfg(feature = "RP2040")]
pub mod CortexM0p;

#[cfg(feature = "RP2040")]
pub use CortexM0p as Current;
