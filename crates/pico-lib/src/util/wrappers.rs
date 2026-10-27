use pico_abi::syscall::RawSyscallArguments;
use pico_abi::syscall::RawSyscallResult;

pub unsafe fn syscall_wrapper<const TABLE_ID: u8>(
    syscall_id: u32,
    arguments: RawSyscallArguments,
) -> RawSyscallResult {
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

            in("r4") syscall_id,

            lateout("r12") _,

            table_id  = const(TABLE_ID)
        );
    }

    return RawSyscallResult { status, value };
}
