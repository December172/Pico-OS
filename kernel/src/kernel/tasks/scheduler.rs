use core::cell::UnsafeCell;

use pico_abi::syscall::SyscallError;

use crate::arch::general::asm;

use crate::kernel::tasks::task::{Task, TaskId, TaskState};
use crate::kernel::tasks::task_table::{MAX_TASKS, TaskTable};

const IDLE_TASK_ID: TaskId = 0;

pub struct Scheduler {
    task_table: TaskTable,
    current_task_id: Option<TaskId>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            task_table: TaskTable::new(),
            current_task_id: None,
        }
    }

    pub fn get_current_task_id(&self) -> Option<TaskId> {
        return self.current_task_id;
    }

    pub fn insert_task(&mut self, task: Task) -> TaskId {
        return self
            .task_table
            .insert(task)
            .unwrap_or_else(|_| panic!("Task table is full"));
    }

    /// For the pend_sv bridge use only
    pub fn start(&mut self) -> *mut u32 {
        assert!(self.current_task_id.is_none(), "scheduler already started");
        let next_task_id = self.select_next_ready_task();
        let next_task = self
            .task_table
            .get_mut(next_task_id)
            .expect("Selected task disappeared");

        assert!(next_task.state == TaskState::Ready);
        next_task.state = TaskState::Running;
        self.current_task_id = Some(next_task_id);

        return next_task.saved_stack_pointer;
    }

    /// For the pend_sv bridge use only
    pub fn schedule_next_task(&mut self, current_stack_pointer: *mut u32) -> *mut u32 {
        {
            let current_task_id = self.current_task_id.expect("No running task");
            let current_task = self
                .task_table
                .get_mut(current_task_id)
                .expect("current task is missing in task table");
            if current_task.state == TaskState::Running {
                current_task.state = TaskState::Ready;
            }
            current_task.saved_stack_pointer = current_stack_pointer;
        }
        let next_task_id = self.select_next_ready_task();
        let next_task = self.task_table.get_mut(next_task_id).unwrap();

        next_task.state = TaskState::Running;
        self.current_task_id = Some(next_task_id);
        return next_task.saved_stack_pointer;
    }

    pub fn yield_task(&mut self) {
        let current_task_id = self
            .current_task_id
            .expect("Yielding without a current task");

        let current_task = self
            .task_table
            .get_mut(current_task_id)
            .expect("current task is missing from task table");

        assert!(current_task.state == TaskState::Running);

        current_task.state = TaskState::Ready;
    }

    pub fn kill_task(
        &mut self,
        caller_task_id: TaskId,
        target_task_id: TaskId,
    ) -> Result<bool, SyscallError> {
        if target_task_id == IDLE_TASK_ID {
            // trying to kill the idle task
            return Err(SyscallError::PermissionDenied);
        }

        let kill_current_task = self.current_task_id == Some(target_task_id);

        let target_task = self
            .task_table
            .get_mut(target_task_id)
            .ok_or(SyscallError::InvalidHandle)?;

        if target_task.state == TaskState::Dead {
            return Err(SyscallError::NotRunning);
        }

        target_task.state = TaskState::Dead;

        return Ok(kill_current_task);
    }

    fn select_next_ready_task(&self) -> TaskId {
        let start_index = match self.current_task_id {
            // wrap task ids inside the task_table (% MAX_TASKS)
            Some(task_id) => (task_id + 1) % MAX_TASKS,
            None => 0,
        };

        for offset in 0..MAX_TASKS {
            // wrap task ids inside the task_table (% MAX_TASKS)
            let task_id = (start_index + offset) % MAX_TASKS;
            if task_id == IDLE_TASK_ID {
                // choose non-idle task first
                continue;
            }

            let Some(task) = self.task_table.get(task_id) else {
                continue;
            };

            if task.state != TaskState::Ready {
                continue;
            }

            return task_id;
        }
        return IDLE_TASK_ID;
    }
}

pub(crate) struct SchedulerWrapper {
    inner: UnsafeCell<Scheduler>,
}

unsafe impl Sync for SchedulerWrapper {}

impl SchedulerWrapper {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(Scheduler::new()),
        }
    }

    pub(crate) unsafe fn with_exclusive<Output>(
        &self,
        method: impl FnOnce(&mut Scheduler) -> Output,
    ) -> Output {
        return asm::with_interrupts_disabled(|| {
            let scheduler = unsafe { &mut *self.inner.get() };

            return method(scheduler);
        });
    }
}

/// kernel singleton scheduler wrapper
pub(crate) static SCHEDULER: SchedulerWrapper = SchedulerWrapper::new();
