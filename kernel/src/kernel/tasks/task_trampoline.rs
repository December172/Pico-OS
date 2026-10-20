use crate::kernel::tasks::scheduler::TASK_SELF;
use crate::kernel::tasks::syscalls::TASK_SYSCALL_TABLE_ID;
use crate::kernel::tasks::task::Task;

use crate::util::syscall::syscall_wrapper;

pub extern "C" fn task_entry(raw_task_ptr: *mut Task) {
    let (entry, argument) = unsafe {
        let task = &*raw_task_ptr;
        (task.entry, task.argument)
    };

    entry(argument);
}

/// Called automatically when `task_entry()` returns
pub extern "C" fn task_exit() -> ! {
    unsafe {
        let result = syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(0x2, [TASK_SELF as u32, 0, 0, 0]);
    }
    unreachable!();
}
