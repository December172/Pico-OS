#[cfg(feature = "RP2040")]
pub mod RP2040;

#[cfg(feature = "RP2350")]
pub mod RP2350;

#[cfg(feature = "RP2040")]
pub use RP2040::Clock::ROSCDriver::_ROSCDriver as ROSCDriver;
#[cfg(feature = "RP2040")]
pub use RP2040::Clock::XOSCDriver::_XOSCDriver as XOSCDriver;
#[cfg(feature = "RP2040")]
pub use RP2040::GPIO::GPIODriver::_GPIODriver as GPIODriver;
#[cfg(feature = "RP2040")]
pub use RP2040::Timer::TimerDriver::_TimerDriver as TimerDriver;

#[cfg(feature = "RP2350")]
pub use RP2350::Clock::ROSCDriver::_ROSCDriver as ROSCDriver;
#[cfg(feature = "RP2350")]
pub use RP2350::Clock::XOSCDriver::_XOSCDriver as XOSCDriver;
#[cfg(feature = "RP2350")]
pub use RP2350::GPIO::GPIODriver::_GPIODriver as GPIODriver;
#[cfg(feature = "RP2350")]
pub use RP2350::Timer::TimerDriver::_TimerDriver as TimerDriver;