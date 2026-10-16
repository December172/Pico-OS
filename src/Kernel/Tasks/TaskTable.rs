use core::mem::size_of;

use crate::Arch::Current::Context::initializeTaskStack;
use crate::Kernel::Tasks::Task::*;
use crate::Kernel::Tasks::TaskTrampoline::*;

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
        for taskId in 0..MAX_TASKS {
            if self.tasks[taskId].is_none() {
                task.id = taskId;

                self.tasks[taskId] = Some(task);
                unsafe {
                    let movedTask = self.tasks[taskId].as_mut().unwrap();
                    movedTask.savedStackPointer = initializeTaskStack(&mut TASK_STACKS.stacks[taskId],
                                                                 taskEntry,
                                                                 movedTask as *mut Task,
                                                                 taskExit)
                }
                return Ok(taskId);
            }
        }
        return Err(task);
    }

    pub fn remove(&mut self, taskId: TaskId) -> Option<Task> {
        return self.tasks.get_mut(taskId)?.take();
    }

    pub fn get(&self, taskId: TaskId) -> Option<&Task> {
        return self.tasks.get(taskId)?.as_ref();
    }

    pub fn getMut(&mut self, taskId: TaskId) -> Option<&mut Task> {
        return self.tasks.get_mut(taskId)?.as_mut();
    }
}