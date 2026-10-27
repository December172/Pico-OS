#![no_std]
#![no_main]

use pico_lib::task;
use pico_lib::task::task_entry;

use core::panic::PanicInfo;

#[task_entry]
pub fn idle_task_entry(_argument: *mut ()) {
    loop {
        core::hint::spin_loop();
        task::yield_task();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
