use pico_abi::syscall::RawSyscallArguments;

use crate::kernel::syscalls::syscall::*;

pub type SyscallEntry =
    fn(context: SyscallContext, arguments: RawSyscallArguments) -> SyscallDispatchResult;

pub struct SyscallTable {
    pub id: u8,
    pub entries: &'static [Option<SyscallEntry>],
}
