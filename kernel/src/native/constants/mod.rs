#[cfg(board_chip = "RP2040")]
pub mod rp2040;

#[cfg(board_chip = "RP2040")]
pub use rp2040::config;
