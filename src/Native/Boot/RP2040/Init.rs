use crate::Util::Register::Register;

use crate::Native::Constants::RP2040::RESETS::*;

pub fn _init() {
    let reset = Register::new(RESETS_RESET);

    let resetDone = Register::new(RESETS_RESET_DONE);

    // release IO_BANK0 reset
    reset.bitSet(RESETS_RESET_IO_BANK0_BIT, false);

    // wait until reset is released
    while !resetDone.bitGet(RESETS_RESET_DONE_IO_BANK0_BIT) {}
}