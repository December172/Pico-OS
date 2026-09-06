#![no_std]
#![no_main]
#![allow(non_snake_case)]

use core::panic::PanicInfo;

mod Boot;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

#[link_section = ".text._start"]
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "bl kernelMain",
        "b ."
    );
}
const RESETS_BASE: usize = 0x4000_C000;

const RESET: usize = 0x00;
const RESET_DONE: usize = 0x08;

const IO_BANK0_RESET: u32 = 1 << 5;


unsafe fn initPeripherals() {
    let reset =
        (RESETS_BASE + RESET) as *mut u32;

    let reset_done =
        (RESETS_BASE + RESET_DONE) as *mut u32;


    // release IO_BANK0 reset
    reset.write_volatile(
        reset.read_volatile() & !IO_BANK0_RESET
    );

    // wait until reset is released
    while reset_done.read_volatile() & IO_BANK0_RESET == 0 {}
}

const IO_BANK0_BASE: usize = 0x40014000;
const SIO_BASE: usize = 0xD0000000;

const GPIO_OUT_SET: usize = 0x14;
const GPIO_OUT_XOR: usize = 0x1c;
const GPIO_OE_SET: usize = 0x24;

const LED: u32 = 1 << 25;

unsafe fn initLed() {
    // GPIO25 control register
    // GPIOx_CTRL = IO_BANK0_BASE + 0x04 + x*8
    let gpio25_ctrl =
        (IO_BANK0_BASE + 0x04 + 25 * 8) as *mut u32;

    // FUNCSEL = 5 (SIO)
    gpio25_ctrl.write_volatile(5);

    // Enable GPIO25 output
    let gpio_oe_set =
        (SIO_BASE + GPIO_OE_SET) as *mut u32;

    gpio_oe_set.write_volatile(LED);

    // Set GPIO25 high
    let gpio_out_set =
        (SIO_BASE + GPIO_OUT_SET) as *mut u32;

    gpio_out_set.write_volatile(LED);
}

#[no_mangle]
pub extern "C" fn kernelMain() -> ! {
    unsafe {
        initPeripherals();
        initLed();

        let gpio_xor =
            (SIO_BASE + GPIO_OUT_XOR) as *mut u32;

        loop {
            gpio_xor.write_volatile(LED);

            for _ in 0..1_000_000 {
                core::hint::spin_loop();
            }
        }
    }
}