use core::mem::size_of;

use crate::arch::current::context::initialize_task_stack;
use crate::kernel::tasks::task::{Task, TaskId};

pub const MAX_TASKS: usize = 8;
pub const TASK_STACK_SIZE: usize = 2048;
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

    pub fn insert(&mut self, mut task: Task) -> Result<TaskId, Task> {
        for task_id in 0..MAX_TASKS {
            if self.tasks[task_id].is_none() {
                task.id = task_id;

                self.tasks[task_id] = Some(task);
                unsafe {
                    let moved_task = self.tasks[task_id].as_mut().unwrap();
                    moved_task.saved_stack_pointer = initialize_task_stack(
                        &mut TASK_STACKS.stacks[task_id],
                        moved_task.entry,
                        moved_task.argument,
                    )
                }
                return Ok(task_id);
            }
        }
        return Err(task);
    }

    pub fn remove(&mut self, task_id: TaskId) -> Option<Task> {
        return self.tasks.get_mut(task_id)?.take();
    }

    pub fn get(&self, task_id: TaskId) -> Option<&Task> {
        return self.tasks.get(task_id)?.as_ref();
    }

    pub fn get_mut(&mut self, task_id: TaskId) -> Option<&mut Task> {
        return self.tasks.get_mut(task_id)?.as_mut();
    }
}
