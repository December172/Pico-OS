use pico_abi::syscall::SyscallError;
use pico_abi::task::{TASK_SYSCALL_TABLE_ID, TaskSyscallId};

use crate::task::{TASKID_SELF, TaskEntry, TaskId};
use crate::util::wrappers::syscall_wrapper;

pub fn create_task(entry: TaskEntry, argument: *mut ()) -> Result<TaskId, SyscallError> {
    let raw_result = unsafe {
        syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(
            TaskSyscallId::Create as u32,
            [entry as u32, argument as u32, 0, 0],
        )
    };

    if raw_result.status == 0 {
        return Ok(TaskId::from_abi(raw_result.value));
    } else {
        return Err(SyscallError::try_from(raw_result.status).expect("Unknown syscall error"));
    }
}

pub fn yield_task() {
    unsafe {
        syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(TaskSyscallId::Yield as u32, [0; 4]);
    }
}

pub fn kill_task(target: TaskId) -> Result<(), SyscallError> {
    let raw_result = unsafe {
        syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(
            TaskSyscallId::Kill as u32,
            [target.into_abi(), 0, 0, 0],
        )
    };

    if raw_result.status == 0 {
        return Ok(());
    }

    return Err(SyscallError::try_from(raw_result.status).expect("Unknown syscall error"));
}

pub fn get_current_task_id() -> TaskId {
    let raw_result = unsafe {
        syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(TaskSyscallId::GetCurrentTaskId as u32, [0; 4])
    };

    return TaskId::from_abi(raw_result.value);
}

pub fn exit() {
    // we can trigger atexit() handler here... in the future
    kill_task(TASKID_SELF);
}
