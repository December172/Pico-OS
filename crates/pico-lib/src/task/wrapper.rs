use pico_internals::task as task_internals;

use crate::task::*;
use crate::util::SyscallError;

pub use task_internals::yield_task;

pub fn create_task(
    entry: TaskEntry,
    argument: *mut (),
    priority: TaskPriority,
) -> Result<TaskId, SyscallError> {
    return task_internals::create_task(entry as u32, argument as u32, priority as u32)
        .map(TaskId::from_abi);
}

pub fn kill_task(target: TaskId) -> Result<(), SyscallError> {
    task_internals::kill_task(target.into_abi())
}

pub fn get_current_task_id() -> TaskId {
    let raw_result = task_internals::get_current_task_id();

    TaskId::from_abi(raw_result)
}

pub fn exit() -> ! {
    // we can trigger atexit() handler here... in the future
    match kill_task(TASKID_SELF) {
        Ok(()) => unreachable!(),
        Err(error) => panic!("Unexpected error occurred on exit: {}", error as i32),
    }
}
