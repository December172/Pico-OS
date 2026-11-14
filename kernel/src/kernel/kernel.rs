use core::panic::PanicInfo;
use core::ptr::null_mut;

use pico_abi::image::USER_TASK_LOAD_FLAG;
use pico_abi::task::TaskPriority;

use crate::arch::current::mpu::init_mpu;
use crate::arch::general::asm;

use crate::kernel::drivers::clock_driver::ClockDriver;
use crate::kernel::syscalls::syscall_registry::SYSCALL_REGISTRY;
use crate::kernel::tasks::scheduler::SCHEDULER;
use crate::kernel::tasks::syscalls::TASK_SYSCALL_TABLE;
use crate::kernel::tasks::task::{TaskDomain, TaskEntry};
use crate::kernel::user::IMAGE_REGISTRY;

use crate::native::boot::_init;
use crate::native::drivers::*;
use crate::tasks::idle_task::idle_task_entry;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("wfi");
    }
    loop {}
}

/// kernel entry point
/// no reentry
pub fn kernel_main() -> ! {
    _init();

    // TODO: Switch to heap-allocated kernel object after finishing slab allocator
    let rosc_driver = ROSCDriver::new();
    let xosc_driver = XOSCDriver::new();
    let clock_driver: &dyn ClockDriver;
    // Fallback clock source
    rosc_driver.init();
    if !xosc_driver.init() {
        clock_driver = &rosc_driver;
    } else {
        clock_driver = &xosc_driver
    }
    clock_driver.enable();

    unsafe {
        SYSCALL_REGISTRY.initialize(|registry| {
            registry.register(&TASK_SYSCALL_TABLE);
        });
    }

    IMAGE_REGISTRY.init();

    unsafe {
        SCHEDULER.with_exclusive(|scheduler| {
            // Set up all privileged tasks.
            // Idle must be created first so it owns reserved task ID 0.
            scheduler
                .create_task(
                    TaskDomain::Kernel,
                    idle_task_entry as *const () as TaskEntry,
                    null_mut(),
                    TaskPriority::Idle,
                    0,
                )
                .expect("Failed to create idle task");

            // Load all user tasks with the `TASK_LOAD` flag when a user image
            // was linked into the kernel. Without one, only the privileged
            // tasks above are scheduled.
            IMAGE_REGISTRY.with(|registry| {
                for (index, image) in registry.iter_images() {
                    for entry in image.entries.iter().flatten() {
                        if entry.flags & USER_TASK_LOAD_FLAG == 0 {
                            continue;
                        }

                        scheduler
                            .create_task(
                                TaskDomain::User(index),
                                ((image.data.as_ptr() as u32) + entry.entry_offset) as TaskEntry,
                                null_mut(),
                                TaskPriority::Normal,
                                0,
                            )
                            .unwrap_or_else(|error| {
                                // the error should be documented into kernel log in the future
                                // currently it just causes a panic
                                unimplemented!("Failed to create task: {}", error as i32);
                            });
                    }
                }
            });
        });
    }

    unsafe {
        init_mpu();
    }
    asm::pull_idle_task();

    unreachable!();
}
