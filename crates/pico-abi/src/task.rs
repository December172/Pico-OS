use crate::primitive_enum;

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

primitive_enum!(
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum TaskPriority : u32 {
        Idle = 0x0,
        Low = 0x1,
        Normal = 0x2,
        High = 0x3,
        RealTime = 0xFF,
    }
);
