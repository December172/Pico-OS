use pico_abi::task::TaskPriority;

use crate::kernel::memory::layout::MemoryConfig;

/// Task entry address, with C calling convention `extern "C"`
pub type TaskEntry = u32;

pub type TaskId = usize;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Dead,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TaskDomain {
    Kernel,
    User(usize),
}

#[allow(dead_code)]
pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub domain: TaskDomain,
    pub priority: TaskPriority, // reserved
    pub parent_id: TaskId,      // reserved

    pub memory_config: MemoryConfig,
    pub saved_stack_pointer: *mut u32,
    pub entry: TaskEntry,
    pub argument: u32,
}
