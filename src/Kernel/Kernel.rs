use core::ptr::null_mut;

use crate::Kernel::Drivers::ClockDriver::ClockDriver;
use crate::Kernel::Syscalls::SyscallRegistry::SYSCALL_REGISTRY;
use crate::Kernel::Tasks::Task::*;
use crate::Kernel::Tasks::Scheduler::SCHEDULER;
use crate::Kernel::Tasks::Syscalls::TASK_SYSCALL_TABLE;
use crate::Kernel::Tasks::IdleTask;

use crate::Util::Syscall::syscallWrapper;

use crate::Native::Boot::_init;
use crate::Native::Drivers::*;

/// Kernel entry point
/// no reentry
pub fn kernelMain() -> ! {
    _init();

    // TODO: Switch to heap-allocated kernel object after finishing slab allocator
    let roscDriver = ROSCDriver::new();
    let xoscDriver = XOSCDriver::new();
    let clockDriver: &dyn ClockDriver;
    // Fallback clock source
    roscDriver.init();
    if !xoscDriver.init() {
        clockDriver = &roscDriver;
    } else {
        clockDriver = &xoscDriver
    }
    clockDriver.enable();

    unsafe {
        SYSCALL_REGISTRY.initialize(|registry| {
            registry.register(&TASK_SYSCALL_TABLE);
        });

        SCHEDULER.withExclusive(|scheduler| {
            let idleId = scheduler.insertTask(Task {
                id: TASKID_UNASSIGNED,
                state: TaskState::Ready,
                savedStackPointer: null_mut(),

                entry: IdleTask::entry,
                argument: null_mut(),
            });
            assert_eq!(idleId, IdleTask::IDLE_TASK_ID, "IdleTask is not at slot 0")
        })
    }

    unsafe {
        syscallWrapper::<0>(0, [0, 0, 0, 0]);
    }

    unreachable!();
}
