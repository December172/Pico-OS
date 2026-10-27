#![no_std]
#![no_main]
#![allow(unused)]

use pico_runtime::current;

mod arch;

mod hal;
mod kernel;
mod native;
mod util;

/// Rust entry point
pub extern "C" fn entry() {
    crate::kernel::kernel::kernel_main();
}
