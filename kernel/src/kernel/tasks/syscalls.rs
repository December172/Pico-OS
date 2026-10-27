use core::ptr::null_mut;

use pico_abi::syscall::RawSyscallArguments;
use pico_abi::syscall::SyscallError;
use pico_abi::task::TASK_SYSCALL_TABLE_ID;
use pico_abi::task::TASKID_SELF;

use crate::kernel::syscalls::syscall::*;
use crate::kernel::syscalls::syscall_table::*;
use crate::kernel::tasks::scheduler::SCHEDULER;
use crate::kernel::tasks::task::*;

/// syscall create_task() kernel implementation
/// # Arguments
/// arguments[0] : TaskEntry = task_entry
///
/// arguments[1] : *mut ()   = task_argument
fn create_task(_context: SyscallContext, arguments: RawSyscallArguments) -> SyscallDispatchResult {
    let address: u32 = arguments[0];

    // invaild address or non-thumb address
    if address == 0 || address & 1 == 0 {
        return SyscallError::InvalidArgument.into();
    }

    unsafe {
        let task_id = SCHEDULER.with_exclusive(|scheduler| {
            return scheduler.insert_task(Task {
                id: TASKID_UNASSIGNED,
                state: TaskState::Ready,
                saved_stack_pointer: null_mut(),

                entry: arguments[0],
                argument: arguments[1],
            });
        });
        return SyscallDispatchResult {
            status: 0,
            value: task_id as u32,
            action: SyscallAction::Reschedule,
        };
    };
}

/// syscall yield_task() kernel implementation
/// # Arguments
/// None
fn yield_task(_context: SyscallContext, _arguments: RawSyscallArguments) -> SyscallDispatchResult {
    unsafe {
        SCHEDULER.with_exclusive(|scheduler| {
            scheduler.yield_task();
        });
    }

    return SyscallAction::Reschedule.into();
}

/// syscall kill_task() kernel implementation
/// # Arguments
/// arguments[0]: TaskId = TargetTaskId
fn kill_task(context: SyscallContext, arguments: RawSyscallArguments) -> SyscallDispatchResult {
    let mut target_task_id = arguments[0] as usize;
    let result = unsafe {
        SCHEDULER.with_exclusive(|scheduler| {
            if target_task_id == TASKID_SELF as usize {
                target_task_id = context.current_task_id;
            }
            return scheduler.kill_task(context.current_task_id, target_task_id);
        })
    };
    match result {
        Ok(true) => return SyscallAction::Reschedule.into(),
        Ok(false) => return SyscallAction::Return.into(),
        Err(err) => return err.into(),
    }
}

fn get_current_task_id(
    context: SyscallContext,
    _arguments: RawSyscallArguments,
) -> SyscallDispatchResult {
    return (context.current_task_id as u32).into();
}

pub static TASK_SYSCALL_TABLE: SyscallTable = SyscallTable {
    id: TASK_SYSCALL_TABLE_ID,
    entries: &[
        Some(SyscallEntry::new(create_task)),
        Some(SyscallEntry::new(yield_task)),
        Some(SyscallEntry::new(kill_task)),
        Some(SyscallEntry::new(get_current_task_id)),
    ],
};
