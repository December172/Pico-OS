use crate::Native::Constants::RP2040::RESETS::*;
use crate::Native::Constants::RP2040::SIO::*;
use crate::Native::Constants::RP2040::IO_BANK0::*;

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

const LED: u32 = 1 << 25;

unsafe fn initLed() {
    unsafe {
        // GPIO25 control register
        let gpio25Ctrl =
            (ioBank0GpioCtrl(25)) as *mut u32;

        // FUNCSEL = 5 (SIO)
        gpio25Ctrl.write_volatile(5);

        // Enable GPIO25 output
        let gpioOeSet =
            (SIO_GPIO_OE_SET) as *mut u32;

        gpioOeSet.write_volatile(LED);

        // Set GPIO25 high
        let gpioOutSet =
            (SIO_GPIO_OUT_SET) as *mut u32;

        gpioOutSet.write_volatile(LED);
    }
}

pub fn init() {
    unsafe {
        initPeripherals();
        initLed();
    }
}