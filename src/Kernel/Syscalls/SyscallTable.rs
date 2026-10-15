use crate::Kernel::Syscalls::Syscall::*;

/// Arguments of the syscall,
/// grabbed from regs of the exception frame
pub type RawSyscallArguments = [u32; 4];

pub type SyscallThunk = fn(
    context: SyscallContext,
    arguments: RawSyscallArguments,
) -> SyscallResult;

#[derive(Clone, Copy)]
pub struct SyscallEntry {
    invoke: SyscallThunk,
}

impl SyscallEntry {
    pub const fn new(invoke: SyscallThunk) -> Self {
        Self { 
            invoke 
        }
    }

    pub fn invoke(&self, context: SyscallContext, arguments: RawSyscallArguments) -> SyscallResult {
        return (self.invoke)(context, arguments);
    }
}

pub struct SyscallTable {
    pub id: u8,
    pub entries: &'static [Option<SyscallEntry>],
}




