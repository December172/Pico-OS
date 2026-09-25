#![no_std]
#![no_main]
#![allow(non_snake_case)]

#![allow(unused)]

mod Kernel;
mod HAL;
mod Native;
mod Util;

use crate::Kernel::Kernel::kernelMain;

/// Rust entry point
#[unsafe(no_mangle)]
pub extern "C" fn entry() {
    kernelMain();
}

