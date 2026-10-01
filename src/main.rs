#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

mod Kernel;
mod HAL;
mod Native;
mod Util;

/// Rust entry point
#[unsafe(no_mangle)]
pub extern "C" fn entry() {
    crate::Kernel::Kernel::kernelMain();
}

