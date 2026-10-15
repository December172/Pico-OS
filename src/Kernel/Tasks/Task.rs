pub type TaskId = usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Dead,
}

pub trait TaskBody: 'static {
    fn taskEntry(&mut self) -> !;
    fn taskExit(&mut self);
}

pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub savedStackPointer: *mut u32,
}
