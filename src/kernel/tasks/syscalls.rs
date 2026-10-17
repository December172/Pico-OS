use core::mem::transmute;
use core::ptr::null_mut;

use crate::kernel::syscalls::syscall::*;
use crate::kernel::syscalls::syscall_table::*;
use crate::kernel::tasks::scheduler::SCHEDULER;
use crate::kernel::tasks::scheduler::Scheduler;
use crate::kernel::tasks::task::*;

use crate::util::syscall::SyscallArguments;
use crate::util::task::TaskEntry;

pub const TASK_SYSCALL_TABLE_ID: u8 = 0x1;

/// syscall yield_current_task() kernel implementation
/// # Arguments
/// arguments[0] : TaskEntry = task_entry
///
/// arguments[1] : *mut ()   = task_argument
fn create_task(_context: SyscallContext, arguments: SyscallArguments) -> SyscallDispatchResult {
    let address: u32 = arguments[0];

    /// invaild address or non-thumb address
    if address == 0 || address & 1 == 0 {
        return SyscallError::InvalidArgument.into();
    }

    let entry = unsafe { transmute::<usize, TaskEntry>(address as usize) };

    unsafe {
        SCHEDULER.with_exclusive(|scheduler| {
            scheduler.insert_task(Task {
                id: TASKID_UNASSIGNED,
                state: TaskState::Ready,
                saved_stack_pointer: null_mut(),

                entry,
                argument: arguments[1] as *mut (),
            });
        });
    };
    return SyscallAction::Return.into();
}

/// syscall yield_current_task() kernel implementation
/// # Arguments
/// None
fn yield_current_task(
    _context: SyscallContext,
    _arguments: SyscallArguments,
) -> SyscallDispatchResult {
    unsafe {
        SCHEDULER.with_exclusive(|scheduler| {
            scheduler.yield_current_task();
        });
    }

    return SyscallAction::Reschedule.into();
}

/// syscall kill_task() kernel implementation
/// # Arguments
/// arguments[0]: TaskId = TargetTaskId
fn kill_task(context: SyscallContext, arguments: SyscallArguments) -> SyscallDispatchResult {
    let target_task_id = arguments[0] as usize;
    let result = unsafe {
        SCHEDULER.with_exclusive(|scheduler| {
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
    _arguments: SyscallArguments,
) -> SyscallDispatchResult {
    return (context.current_task_id as u32).into();
}

pub static TASK_SYSCALL_TABLE: SyscallTable = SyscallTable {
    id: TASK_SYSCALL_TABLE_ID,
    entries: &[
        Some(SyscallEntry::new(create_task)),
        Some(SyscallEntry::new(yield_current_task)),
        Some(SyscallEntry::new(kill_task)),
        Some(SyscallEntry::new(get_current_task_id)),
    ],
};
