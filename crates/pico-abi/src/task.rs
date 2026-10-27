pub const TASK_SYSCALL_TABLE_ID: u8 = 0x01;

pub const TASKID_SELF: u32 = 0xFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TaskSyscallId {
    Create = 0x00,
    Yield = 0x01,
    Kill = 0x02,
    GetCurrentTaskId = 0x03,
}
