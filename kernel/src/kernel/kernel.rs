use core::panic::PanicInfo;
use core::ptr::null_mut;

use pico_abi::image::USER_TASK_FLAG_IDLE;

use crate::arch::general::asm;

use crate::kernel::drivers::clock_driver::ClockDriver;
use crate::kernel::syscalls::syscall_registry::SYSCALL_REGISTRY;
use crate::kernel::tasks::scheduler::SCHEDULER;
use crate::kernel::tasks::syscalls::TASK_SYSCALL_TABLE;
use crate::kernel::tasks::task::{TASKID_UNASSIGNED, Task, TaskState};
use crate::kernel::user::IMAGE_REGISTRY;

use crate::native::boot::_init;
use crate::native::drivers::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
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
            IMAGE_REGISTRY.with(|registry| {
                for (image, entries) in registry.iter_images() {
                    for entry in entries.iter().flatten() {
                        if entry.flags & USER_TASK_FLAG_IDLE != 0 {
                            scheduler.insert_task(Task {
                                id: 0,
                                state: TaskState::Ready,
                                saved_stack_pointer: null_mut(),
                                entry: image.as_ptr() as u32 + entry.entry_offset,
                                argument: 0,
                            });
                            continue;
                        }
                    }

                    for entry in entries.iter().flatten() {
                        if entry.flags & USER_TASK_FLAG_IDLE != 0 {
                            continue;
                        }
                        scheduler.insert_task(Task {
                            id: TASKID_UNASSIGNED,
                            state: TaskState::Ready,
                            saved_stack_pointer: null_mut(),
                            entry: image.as_ptr() as u32 + entry.entry_offset,
                            argument: 0,
                        });
                    }
                }
            });
        });
    }

    asm::pull_idle_task();

    unreachable!();
}
