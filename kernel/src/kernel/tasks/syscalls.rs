use pico_abi::syscall::RawSyscallArguments;
use pico_abi::syscall::SyscallError;
use pico_abi::task::TASK_SYSCALL_TABLE_ID;
use pico_abi::task::TASKID_SELF;
use pico_abi::task::TaskPriority;

use crate::kernel::syscalls::syscall::*;
use crate::kernel::syscalls::syscall_table::*;
use crate::kernel::tasks::scheduler::SCHEDULER;

/// syscall create_task() kernel implementation
/// # Arguments
/// arguments[0] : TaskEntry = task_entry
///
/// arguments[1] : *mut ()   = task_argument
fn create_task(context: SyscallContext, arguments: RawSyscallArguments) -> SyscallDispatchResult {
    let entry: u32 = arguments[0];
    let Ok(priority) = TaskPriority::try_from(arguments[2]) else {
        return SyscallError::InvalidArgument.into();
    };

    // invaild address or non-thumb address
    if entry == 0 || entry & 1 == 0 {
        return SyscallError::InvalidArgument.into();
    }

    unsafe {
        let result = SCHEDULER.with_exclusive(|scheduler| {
            let domain = scheduler.get_task(context.current_task_id).unwrap().domain;

            return scheduler.create_task(
                domain,
                entry,
                arguments[1] as *mut (),
                priority,
                context.current_task_id,
            );
        });
        return result.map(|task_id| task_id as u32).into();
    };
}

/// syscall yield_task() kernel implementation
/// # Arguments
/// None
fn yield_task(_context: SyscallContext, _arguments: RawSyscallArguments) -> SyscallDispatchResult {
    SyscallAction::Reschedule.into()
}

/// syscall kill_task() kernel implementation
/// # Arguments
/// arguments[0]: TaskId = TargetTaskId
fn kill_task(context: SyscallContext, arguments: RawSyscallArguments) -> SyscallDispatchResult {
    let mut target_task_id = arguments[0] as usize;
    if target_task_id == TASKID_SELF as usize {
        target_task_id = context.current_task_id;
    }
    let result = unsafe {
        SCHEDULER.with_exclusive(|scheduler| {
            return scheduler.kill_task(context.current_task_id, target_task_id);
        })
    };
    result
        .map(|reschedule| {
            if reschedule {
                SyscallAction::Reschedule
            } else {
                SyscallAction::Return
            }
        })
        .into()
}

/// syscall get_current_task_id() kernel implementation
/// # Arguments
/// None
fn get_current_task_id(
    context: SyscallContext,
    _arguments: RawSyscallArguments,
) -> SyscallDispatchResult {
    (context.current_task_id as u32).into()
}

pub static TASK_SYSCALL_TABLE: SyscallTable = SyscallTable {
    id: TASK_SYSCALL_TABLE_ID,
    entries: &[
        Some(create_task),
        Some(yield_task),
        Some(kill_task),
        Some(get_current_task_id),
    ],
};
