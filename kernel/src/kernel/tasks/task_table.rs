use core::mem::size_of;

use crate::arch::current::context::initialize_task_stack;
use crate::kernel::memory::layout::MemoryRange;
use crate::kernel::tasks::task::{Task, TaskId};

pub const MAX_TASKS: usize = TASK_STACK_POOL_SIZE / TASK_STACK_SIZE;
pub const TASK_STACK_SIZE: usize = 2 * 1024;
pub const TASK_STACK_POOL_SIZE: usize = 32 * 1024;
pub const TASK_STACK_WORDS: usize = TASK_STACK_SIZE / size_of::<u32>();

#[repr(align(8))]
pub struct TaskStacks {
    pub stacks: [[u32; TASK_STACK_WORDS]; MAX_TASKS],
}

#[unsafe(link_section = ".task_stacks")]
pub static mut TASK_STACKS: TaskStacks = TaskStacks {
    stacks: [[0; TASK_STACK_WORDS]; MAX_TASKS],
};

pub struct TaskTable {
    tasks: [Option<Task>; MAX_TASKS],
}

impl TaskTable {
    pub const fn new() -> Self {
        Self {
            tasks: [const { None }; MAX_TASKS],
        }
    }

    pub fn insert_with(
        &mut self,
        prepare: impl FnOnce(TaskId, MemoryRange) -> Task,
    ) -> Option<TaskId> {
        let task_id = self.tasks.iter().position(Option::is_none)?;
        let stack = unsafe { &mut TASK_STACKS.stacks[task_id] };

        let mut task = prepare(
            task_id,
            MemoryRange {
                base: stack.as_ptr() as u32,
                size: TASK_STACK_SIZE,
            },
        );

        task.saved_stack_pointer = initialize_task_stack(stack, task.entry, task.argument);
        self.tasks[task_id] = Some(task);
        Some(task_id)
    }

    pub fn remove(&mut self, task_id: TaskId) -> Option<Task> {
        self.tasks.get_mut(task_id)?.take()
    }

    pub fn get(&self, task_id: TaskId) -> Option<&Task> {
        self.tasks.get(task_id)?.as_ref()
    }

    pub fn get_mut(&mut self, task_id: TaskId) -> Option<&mut Task> {
        self.tasks.get_mut(task_id)?.as_mut()
    }
}
