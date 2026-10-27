use pico_abi::syscall::RawSyscallArguments;

use crate::kernel::syscalls::syscall::*;

pub type SyscallThunk =
    fn(context: SyscallContext, arguments: RawSyscallArguments) -> SyscallDispatchResult;

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
        arguments: RawSyscallArguments,
    ) -> SyscallDispatchResult {
        return (self.invoke)(context, arguments);
    }
}

pub struct SyscallTable {
    pub id: u8,
    pub entries: &'static [Option<SyscallEntry>],
}
