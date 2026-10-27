use pico_abi::syscall::SyscallError;

use crate::kernel::tasks::task::TaskId;

#[derive(PartialEq)]
pub enum SyscallAction {
    Return,
    Reschedule,
}

pub struct SyscallDispatchResult {
    pub status: i32,
    pub value: u32,
    pub action: SyscallAction,
}

pub struct SyscallContext {
    pub current_task_id: TaskId,
}

impl From<()> for SyscallDispatchResult {
    fn from(_: ()) -> Self {
        Self {
            status: 0,
            value: 0,
            action: SyscallAction::Return,
        }
    }
}

impl From<u32> for SyscallDispatchResult {
    fn from(value: u32) -> Self {
        Self {
            status: 0,
            value,
            action: SyscallAction::Return,
        }
    }
}

impl From<SyscallError> for SyscallDispatchResult {
    fn from(error: SyscallError) -> Self {
        Self {
            status: error as i32,
            value: 0,
            action: SyscallAction::Return,
        }
    }
}

impl From<SyscallAction> for SyscallDispatchResult {
    fn from(action: SyscallAction) -> Self {
        Self {
            status: 0,
            value: 0,
            action,
        }
    }
}

impl<T> From<Result<T, SyscallError>> for SyscallDispatchResult
where
    T: Into<SyscallDispatchResult>,
{
    fn from(result: Result<T, SyscallError>) -> Self {
        match result {
            Ok(value) => return value.into(),
            Err(error) => return error.into(),
        }
    }
}
