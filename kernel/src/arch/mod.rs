#[cfg(target_arch = "arm")]
pub mod cortex_m;

#[cfg(target_arch = "arm")]
pub use cortex_m::current;

#[cfg(target_arch = "arm")]
pub use cortex_m::general;
