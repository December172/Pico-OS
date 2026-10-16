use crate::Kernel::Tasks::Syscalls::TASK_SYSCALL_TABLE_ID;
use crate::Kernel::Tasks::Task::TaskId;

use crate::Util::Syscall::syscallWrapper;

/// the idle task of the kernel, by the kernel, for the kernel
pub const IDLE_TASK_ID: TaskId = 0;

pub fn entry(_argument: *mut ()) {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
        core::hint::spin_loop();
        unsafe {
            syscallWrapper::<TASK_SYSCALL_TABLE_ID>(0x1, [0; 4]);
        }
    }
}