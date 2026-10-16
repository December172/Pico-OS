use crate::Kernel::Syscalls::Syscall::*;

use crate::Util::Syscall::SyscallArguments;

pub type SyscallThunk =
    fn(context: SyscallContext, arguments: SyscallArguments) -> SyscallDispatchResult;

#[derive(Clone, Copy)]
pub struct SyscallEntry {
    invoke: SyscallThunk,
}

impl SyscallEntry {
    pub const fn new(invoke: SyscallThunk) -> Self {
        Self { invoke }
    }

    pub fn invoke(
        &self,
        context: SyscallContext,
        arguments: SyscallArguments,
    ) -> SyscallDispatchResult {
        return (self.invoke)(context, arguments);
    }
}

pub struct SyscallTable {
    pub id: u8,
    pub entries: &'static [Option<SyscallEntry>],
}
