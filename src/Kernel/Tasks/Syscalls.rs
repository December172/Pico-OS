use core::mem::transmute;
use core::ptr::null_mut;

use crate::Kernel::Syscalls::Syscall::*;
use crate::Kernel::Syscalls::SyscallTable::*;
use crate::Kernel::Tasks::Scheduler::SCHEDULER;
use crate::Kernel::Tasks::Scheduler::Scheduler;
use crate::Kernel::Tasks::Task::*;

use crate::Util::Syscall::SyscallArguments;
use crate::Util::Task::TaskEntry;

pub const TASK_SYSCALL_TABLE_ID: u8 = 0x1;

/// Syscall yieldCurrentTask() kernel implementation
/// # Arguments
/// arguments[0] : TaskEntry = taskEntry
///
/// arguments[1] : *mut ()   = taskArgument
fn createTask(_context: SyscallContext, arguments: SyscallArguments) -> SyscallDispatchResult {
    let address: u32 = arguments[0];

    /// invaild address or non-thumb address
    if address == 0 || address & 1 == 0 {
        return SyscallError::InvalidArgument.into();
    }

    let entry = unsafe { transmute::<usize, TaskEntry>(address as usize) };

    unsafe {
        SCHEDULER.withExclusive(|scheduler| {
            scheduler.insertTask(Task {
                id: TASKID_UNASSIGNED,
                state: TaskState::Ready,
                savedStackPointer: null_mut(),

                entry,
                argument: arguments[1] as *mut (),
            });
        });
    };
    return SyscallAction::Return.into();
}

/// Syscall yieldCurrentTask() kernel implementation
/// # Arguments
/// None
fn yieldCurrentTask(
    _context: SyscallContext,
    _arguments: SyscallArguments,
) -> SyscallDispatchResult {
    unsafe {
        SCHEDULER.withExclusive(|scheduler| {
            scheduler.yieldCurrentTask();
        });
    }

    return SyscallAction::Reschedule.into();
}

/// Syscall killTask() kernel implementation
/// # Arguments
/// arguments[0]: TaskId = TargetTaskId
fn killTask(context: SyscallContext, arguments: SyscallArguments) -> SyscallDispatchResult {
    let targetTaskId = arguments[0] as usize;
    let result = unsafe {
        SCHEDULER.withExclusive(|scheduler| {
            return scheduler.killTask(context.currentTaskId, targetTaskId);
        })
    };
    match result {
        Ok(true) => return SyscallAction::Reschedule.into(),
        Ok(false) => return SyscallAction::Return.into(),
        Err(err) => return err.into(),
    }
}

fn getCurrentTaskId(
    context: SyscallContext,
    _arguments: SyscallArguments,
) -> SyscallDispatchResult {
    return (context.currentTaskId as u32).into();
}

pub static TASK_SYSCALL_TABLE: SyscallTable = SyscallTable {
    id: TASK_SYSCALL_TABLE_ID,
    entries: &[
        Some(SyscallEntry::new(createTask)),
        Some(SyscallEntry::new(yieldCurrentTask)),
        Some(SyscallEntry::new(killTask)),
        Some(SyscallEntry::new(getCurrentTaskId)),
    ],
};
