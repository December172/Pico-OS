#![no_std]

#[cfg(target_arch = "arm")]
pub mod arm;

#[cfg(target_arch = "arm")]
pub use arm as current;
