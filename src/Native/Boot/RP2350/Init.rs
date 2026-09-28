use crate::Util::Register::Register;

use crate::Native::Constants::RP2350::RESETS::*;

/// Low-level boot init.
pub fn _init() {
    let reset = Register::new(RESETS_RESET);
    let resetDone = Register::new(RESETS_RESET_DONE);

    // Release every peripheral from reset explicitly. (for debug purposes)
    reset.write(0);

    // Wait for the peripherals the kernel touches to actually come out of reset.
    while !(resetDone.bitGet(RESETS_RESET_DONE_IO_BANK0_BIT)
        && resetDone.bitGet(RESETS_RESET_DONE_PADS_BANK0_BIT)
        && resetDone.bitGet(RESETS_RESET_DONE_TIMER0_BIT))
    {}
}
