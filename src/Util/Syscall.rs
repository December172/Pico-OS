/// Arguments of the syscall,
/// grabbed from regs of the exception frame
pub type SyscallArguments = [u32; 4];

pub struct SyscallResult {
    pub status: i32,
    pub value: u32,
}

pub unsafe fn syscallWrapper<const TABLE_ID: u8>(
    syscallId: u32,
    arguments: SyscallArguments,
) -> SyscallResult {
    let status: i32;
    let value: u32;
    unsafe {
        core::arch::asm!(
            "mov r12, r4",

            "svc  #{table_id}",     // task syscall table

            inlateout("r0") arguments[0] => status,
            inlateout("r1") arguments[1] => value,
            in("r2") arguments[2],
            in("r3") arguments[3],

            in("r4") syscallId,

            table_id  = const(TABLE_ID)
        );
    }

    return SyscallResult { status, value };
}
