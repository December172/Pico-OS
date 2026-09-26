#[cfg(feature = "RP2040")]
pub mod RP2040;

#[cfg(feature = "RP2040")]
pub use RP2040::ROSCDriver::_ROSCDriver as ROSCDriver;
#[cfg(feature = "RP2040")]
pub use RP2040::XOSCDriver::_XOSCDriver as XOSCDriver;
#[cfg(feature = "RP2040")]
pub use RP2040::GPIODriver::_GPIODriver as GPIODriver;
#[cfg(feature = "RP2040")]
pub use RP2040::TimerDriver::_TimerDriver as TimerDriver;