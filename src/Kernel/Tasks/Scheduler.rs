use core::cell::UnsafeCell;

use crate::Arch::General::ASM;

use crate::Kernel::Syscalls::Syscall::*;
use crate::Kernel::Tasks::IdleTask::IDLE_TASK_ID;
use crate::Kernel::Tasks::Task::*;
use crate::Kernel::Tasks::TaskTable::*;

pub const TASK_SELF: TaskId = 0xFF;

pub struct Scheduler {
    taskTable: TaskTable,
    currentTaskId: Option<TaskId>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            taskTable: TaskTable::new(),
            currentTaskId: None,
        }
    }

    pub fn getCurrentTaskId(&self) -> Option<TaskId> {
        return self.currentTaskId;
    }

    pub fn insertTask(&mut self, task: Task) -> TaskId {
        return self
            .taskTable
            .insert(task)
            .unwrap_or_else(|_| panic!("Task table is full"));
    }

    /// For the PendSv bridge use only
    pub fn start(&mut self) -> *mut u32 {
        assert!(self.currentTaskId.is_none(), "scheduler already started");
        let nextTaskId = self.selectNextReadyTask();
        let nextTask = self
            .taskTable
            .getMut(nextTaskId)
            .expect("Selected task disappeared");

        assert!(nextTask.state == TaskState::Ready);
        nextTask.state = TaskState::Running;
        self.currentTaskId = Some(nextTaskId);

        return nextTask.savedStackPointer;
    }

    /// For the PendSv bridge use only
    pub fn scheduleNextTask(&mut self, currentStackPointer: *mut u32) -> *mut u32 {
        {
            let currentTaskId = self.currentTaskId.expect("No running task");
            let currentTask = self
                .taskTable
                .getMut(currentTaskId)
                .expect("Current task is missing in task table");
            if currentTask.state == TaskState::Running {
                currentTask.state = TaskState::Ready;
            }
            currentTask.savedStackPointer = currentStackPointer;
        }
        let nextTaskId = self.selectNextReadyTask();
        let nextTask = self.taskTable.getMut(nextTaskId).unwrap();

        nextTask.state = TaskState::Running;
        self.currentTaskId = Some(nextTaskId);
        return nextTask.savedStackPointer;
    }

    pub fn yieldCurrentTask(&mut self) {
        let currentTaskId = self.currentTaskId.expect("Yielding without a current task");

        let currentTask = self
            .taskTable
            .getMut(currentTaskId)
            .expect("Current task is missing from task table");

        assert!(currentTask.state == TaskState::Running);

        currentTask.state = TaskState::Ready;
    }

    pub fn killTask(
        &mut self,
        callerTaskId: TaskId,
        targetTaskId: TaskId,
    ) -> Result<bool, SyscallError> {
        let rawTargetTaskId = if targetTaskId == TASK_SELF {
            callerTaskId
        } else {
            targetTaskId
        };

        if rawTargetTaskId == IDLE_TASK_ID {
            // trying to kill the idle task
            return Err(SyscallError::PermissionDenied);
        }

        let killCurrentTask = self.currentTaskId == Some(rawTargetTaskId);

        let targetTask = self
            .taskTable
            .getMut(rawTargetTaskId)
            .ok_or(SyscallError::InvalidHandle)?;

        if targetTask.state == TaskState::Dead {
            return Err(SyscallError::NotRunning);
        }

        targetTask.state = TaskState::Dead;

        return Ok(killCurrentTask);
    }

    fn selectNextReadyTask(&self) -> TaskId {
        let startIndex = match self.currentTaskId {
            // wrap task ids inside the taskTable (% MAX_TASKS)
            Some(taskId) => (taskId + 1) % MAX_TASKS,
            None => 0,
        };

        for offset in 0..MAX_TASKS {
            // wrap task ids inside the taskTable (% MAX_TASKS)
            let taskId = (startIndex + offset) % MAX_TASKS;
            if taskId == IDLE_TASK_ID {
                // choose non-idle task first
                continue;
            }

            let Some(task) = self.taskTable.get(taskId) else {
                continue;
            };

            if task.state != TaskState::Ready {
                continue;
            }

            return taskId;
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

    pub(crate) unsafe fn withExclusive<Output>(
        &self,
        method: impl FnOnce(&mut Scheduler) -> Output,
    ) -> Output {
        return ASM::withInterruptsDisabled(|| {
            let scheduler = unsafe { &mut *self.inner.get() };

            return method(scheduler);
        });
    }
}

/// Kernel singleton scheduler wrapper
pub(crate) static SCHEDULER: SchedulerWrapper = SchedulerWrapper::new();
