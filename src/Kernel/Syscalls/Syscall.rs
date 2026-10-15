use crate::Kernel::Tasks::Task::TaskId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SyscallError {
    RegistryNotReady = -1,
    UnknownSyscall   = -2,
    NotRunning       = -3,
    InvalidArgument  = -4,
    InvalidHandle    = -5,
    PermissionDenied = -6,
}

#[repr(u16)]
pub enum SyscallAction {
    Return = 0x00,
    Reschedule = 0x01,
}

pub struct SyscallResult {
    pub status: i32,
    pub value:  u32,
    pub action: SyscallAction
}

pub struct SyscallContext {
    pub currentTaskId: TaskId,
}

impl From<()> for SyscallResult {
    fn from(_: ()) -> Self {
        Self {
            status: 0,
            value: 0,
            action: SyscallAction::Return
        }
    }
}

impl From<u32> for SyscallResult {
    fn from(value: u32) -> Self {
        Self {
            status: 0,
            value,
            action: SyscallAction::Return
        }
    }
}

impl From<SyscallError> for SyscallResult {
    fn from(error: SyscallError) -> Self {
        Self {
            status: error as i32,
            value: 0,
            action: SyscallAction::Return
        }
    }
}

impl From<SyscallAction> for SyscallResult {
    fn from(action: SyscallAction) -> Self {
        Self {
            status: 0,
            value: 0,
            action,
        }
    }
}

impl<T> From<Result<T, SyscallError>> for SyscallResult where T: Into<SyscallResult> {
    fn from(result: Result<T, SyscallError>) -> Self {
        match result {
            Ok(value) => return value.into(),
            Err(error)=> return error.into(),        
        }
    }
}