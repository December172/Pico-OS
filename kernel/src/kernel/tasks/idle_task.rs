use crate::kernel::tasks::syscalls::TASK_SYSCALL_TABLE_ID;
use crate::kernel::tasks::task::TaskId;

use crate::util::syscall::syscall_wrapper;

/// the idle task of the kernel, by the kernel, for the kernel
pub const IDLE_TASK_ID: TaskId = 0;

pub fn entry(_argument: *mut ()) {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
        core::hint::spin_loop();
        unsafe {
            syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(0x1, [0; 4]);
        }
    }
}
