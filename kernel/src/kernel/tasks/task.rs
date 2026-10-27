/// Task entry address, with C calling convention `extern "C"`
pub type TaskEntry = u32;

pub type TaskId = usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Dead,
}

pub const TASKID_UNASSIGNED: TaskId = TaskId::MAX;

pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub saved_stack_pointer: *mut u32,

    pub entry: TaskEntry,
    pub argument: u32,
}
