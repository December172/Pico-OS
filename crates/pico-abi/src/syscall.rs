use crate::primitive_enum;

pub type RawSyscallArguments = [u32; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RawSyscallResult {
    pub status: i32,
    pub value: u32,
}

primitive_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SyscallError: i32 {
        RegistryNotReady = -1,
        UnknownSyscall = -2,
        NotRunning = -3,
        InvalidArgument = -4,
        InvalidHandle = -5,
        PermissionDenied = -6,
        OutOfResource = -7,
        UnknownError = -0xFF
    }
}
