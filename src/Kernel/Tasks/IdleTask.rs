use core::ptr::{null, null_mut};

use crate::Kernel::Tasks::Task::*;
use crate::Kernel::Tasks::Syscalls::TASK_SYSCALL_TABLE_ID;

use crate::Util::Syscall::syscallWrapper;

/// the idle task of the kernel, by the kernel, for the kernel
pub const IDLE_TASK_ID: TaskId = 0;

fn entry(_argument: *mut ()) {
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

pub struct IdleTask;

impl IdleTask {
    pub fn new(argument: *mut ()) -> Task {
        Task {
            id: TASKID_UNASSIGNED,
            state: TaskState::Ready,
            savedStackPointer: 0x0 as *mut u32,

            entry,
            argument,
        }
    }
}
