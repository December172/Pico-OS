pub type TaskEntry = extern "C" fn(*mut ()) -> !;

use pico_abi::task::TASKID_SELF as RAW_TASKID_SELF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TaskId(u32);

impl TaskId {
    pub(crate) const fn from_abi(value: u32) -> Self {
        return Self(value);
    }

    pub(crate) const fn into_abi(self) -> u32 {
        return self.0;
    }
}

pub const TASKID_SELF: TaskId = TaskId(RAW_TASKID_SELF);

mod trampoline;
pub mod wrapper;

pub use pico_macros::task_entry;
pub use wrapper::*;
