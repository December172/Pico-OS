#[cfg(feature = "RP2040")]
pub mod rp2040;

#[cfg(feature = "RP2040")]
pub use rp2040::clock::rosc_driver::_ROSCDriver as ROSCDriver;
#[cfg(feature = "RP2040")]
pub use rp2040::clock::xosc_driver::_XOSCDriver as XOSCDriver;
#[cfg(feature = "RP2040")]
pub use rp2040::gpio::gpio_driver::_GPIODriver as GPIODriver;
#[cfg(feature = "RP2040")]
pub use rp2040::timer::timer_driver::_TimerDriver as TimerDriver;
