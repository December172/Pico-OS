use crate::Native::Constants::RP2040::RESETS::*;

const IO_BANK0_RESET: u32 = 1 << 5;

unsafe fn initPeripherals() {
    unsafe {
        let reset =
            (RESETS_RESET) as *mut u32;

        let resetDone =
            (RESETS_RESET_DONE) as *mut u32;


        // release IO_BANK0 reset
        reset.write_volatile(
            reset.read_volatile() & !IO_BANK0_RESET
        );

        // wait until reset is released
        while resetDone.read_volatile() & IO_BANK0_RESET == 0 {}
    }
}

pub fn init() {
    unsafe {
        initPeripherals();
    }
}