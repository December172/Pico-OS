use crate::util::task::TaskEntry;

pub type TaskId = usize;

pub const TASKID_UNASSIGNED: TaskId = TaskId::MAX;

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Dead,
}

pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub saved_stack_pointer: *mut u32,

    pub entry: TaskEntry,
    pub argument: *mut (),
}
