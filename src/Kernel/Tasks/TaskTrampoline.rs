use crate::Kernel::Tasks::Task::Task;
use crate::Kernel::Tasks::Syscalls::TASK_SYSCALL_TABLE_ID;
use crate::Kernel::Tasks::Scheduler::TASK_SELF;

use crate::Util::Syscall::syscallWrapper;

pub extern "C" fn taskEntry(rawTaskPtr: *mut Task) {
    let (entry, argument) = unsafe {
        let task = &*rawTaskPtr;
        (task.entry, task.argument)
    };

    entry(argument);
}

/// Called automatically when `taskEntry()` returns
pub extern "C" fn taskExit() -> ! {
    unsafe {
        let result = syscallWrapper::<TASK_SYSCALL_TABLE_ID>(
            0x2,
            [TASK_SELF as u32, 0, 0, 0],
        );
    }
    unreachable!();
}