#[cfg(board_chip = "RP2040")]
pub mod rp2040;

#[cfg(board_chip = "RP2040")]
pub use rp2040::clock::rosc_driver::_ROSCDriver as ROSCDriver;
#[cfg(board_chip = "RP2040")]
pub use rp2040::clock::xosc_driver::_XOSCDriver as XOSCDriver;
