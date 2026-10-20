#![no_std]
#![no_main]
#![allow(unused)]

#[cfg(feature = "RP2040")]
mod arch;

mod hal;
mod kernel;
mod native;
mod util;

/// Rust entry point
#[unsafe(no_mangle)]
pub extern "C" fn entry() {
    crate::kernel::kernel::kernel_main();
}
