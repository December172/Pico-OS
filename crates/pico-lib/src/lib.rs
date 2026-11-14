#![no_std]
#![allow(unused)]

use pico_runtime::current;

#[doc(hidden)]
pub use pico_internals::task_trampoline;

pub mod task;
pub mod util;
