#[cfg(feature = "RP2040")]
pub mod RP2040;

#[cfg(feature = "RP2040")]
pub use RP2040::Clock::ROSCDriver::_ROSCDriver as ROSCDriver;
#[cfg(feature = "RP2040")]
pub use RP2040::Clock::XOSCDriver::_XOSCDriver as XOSCDriver;
#[cfg(feature = "RP2040")]
pub use RP2040::GPIO::GPIODriver::_GPIODriver as GPIODriver;
#[cfg(feature = "RP2040")]
pub use RP2040::Timer::TimerDriver::_TimerDriver as TimerDriver;
