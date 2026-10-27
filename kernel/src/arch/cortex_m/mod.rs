pub mod general;

#[cfg(board_chip = "RP2040")]
pub mod cortex_m0p;

#[cfg(board_chip = "RP2040")]
pub use cortex_m0p as current;
