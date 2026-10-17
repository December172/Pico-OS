use core::ptr::null_mut;

use crate::kernel::drivers::clock_driver::ClockDriver;
use crate::kernel::syscalls::syscall_registry::SYSCALL_REGISTRY;
use crate::kernel::tasks::idle_task;
use crate::kernel::tasks::scheduler::SCHEDULER;
use crate::kernel::tasks::syscalls::TASK_SYSCALL_TABLE;
use crate::kernel::tasks::task::*;

use crate::util::syscall::syscall_wrapper;

use crate::native::boot::_init;
use crate::native::drivers::*;

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

        SCHEDULER.with_exclusive(|scheduler| {
            let idle_id = scheduler.insert_task(Task {
                id: TASKID_UNASSIGNED,
                state: TaskState::Ready,
                saved_stack_pointer: null_mut(),

                entry: idle_task::entry,
                argument: null_mut(),
            });
            assert_eq!(
                idle_id,
                idle_task::IDLE_TASK_ID,
                "idle_task is not at slot 0"
            )
        })
    }

    unsafe {
        syscall_wrapper::<0>(0, [0, 0, 0, 0]);
    }

    unreachable!();
}
