use crate::util::register::Register;

use crate::native::constants::rp2040::resets::*;

/// Low-level boot init.
pub fn _init() {
    let reset = Register::new(RESETS_RESET);
    let reset_done = Register::new(RESETS_RESET_DONE);

    // Release every peripheral from reset explicitly. (for debug purposes)
    reset.write(0);

    // Wait for the peripherals the kernel touches to actually come out of reset.
    while !(reset_done.bit_get(RESETS_RESET_DONE_IO_BANK0_BIT)
        && reset_done.bit_get(RESETS_RESET_DONE_PADS_BANK0_BIT)
        && reset_done.bit_get(RESETS_RESET_DONE_TIMER_BIT))
    {}
}
