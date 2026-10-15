#[cfg(target_arch = "arm")]
pub mod CortexM;

#[cfg(target_arch = "arm")]
pub use CortexM::Current;

#[cfg(target_arch = "arm")]
pub use CortexM::General;
