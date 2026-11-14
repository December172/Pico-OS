use core::cell::UnsafeCell;
use core::ptr::null_mut;

use pico_abi::syscall::SyscallError;
use pico_abi::task::TaskPriority;

use crate::arch::general::asm;

use crate::kernel::memory::layout::{MemoryConfig, MemoryRange};
use crate::kernel::tasks::task::*;
use crate::kernel::tasks::task_table::{MAX_TASKS, TaskTable};
#[cfg(pico_user_image)]
use crate::kernel::user::IMAGE_REGISTRY;

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

    pub(crate) fn get_task(&self, task_id: TaskId) -> Option<&Task> {
        self.task_table.get(task_id)
    }

    pub fn get_current_task_id(&self) -> Option<TaskId> {
        self.current_task_id
    }

    /// For the pend_sv bridge use only
    pub fn start(&mut self) -> *mut u32 {
        assert!(self.current_task_id.is_none(), "scheduler already started");
        self.activate_next_ready_task()
    }

    /// For the pend_sv bridge use only
    pub fn schedule_next_task(&mut self, current_stack_pointer: *mut u32) -> *mut u32 {
        {
            let current_task_id = self.current_task_id.expect("No running task");
            if self.task_table.get(current_task_id).as_ref().unwrap().state == TaskState::Dead {
                self.task_table.remove(current_task_id);
            } else {
                let current_task = self
                    .task_table
                    .get_mut(current_task_id)
                    .expect("current task is missing in task table");
                if current_task.state == TaskState::Running {
                    current_task.state = TaskState::Ready;
                }
                current_task.saved_stack_pointer = current_stack_pointer;
            }
        }
        self.activate_next_ready_task()
    }

    /// For the hard_fault bridge use only
    pub fn discard_current_task(&mut self) -> Result<*mut u32, SyscallError> {
        let id = self.current_task_id.ok_or(SyscallError::NotRunning)?;

        self.kill_task(id, id)?;
        self.task_table.remove(id);

        Ok(self.activate_next_ready_task())
    }

    pub fn create_task(
        &mut self,
        domain: TaskDomain,
        entry: TaskEntry,
        argument: *mut (),
        priority: TaskPriority,
        parent_id: TaskId,
    ) -> Result<TaskId, SyscallError> {
        let map = match domain {
            TaskDomain::Kernel => None,
            #[cfg(pico_user_image)]
            TaskDomain::User(image_id) => Some(IMAGE_REGISTRY.with(|registry| {
                let image = registry
                    .images
                    .get(image_id)
                    .and_then(Option::as_ref)
                    .ok_or(SyscallError::InvalidArgument)?;

                // Normalize the address before checking image ownership.
                if !image.contains_entry(entry) {
                    return Err(SyscallError::InvalidArgument);
                }

                Ok(image.mem_map)
            })?),
            #[cfg(not(pico_user_image))]
            TaskDomain::User(_) => return Err(SyscallError::InvalidArgument),
        };

        return self
            .task_table
            .insert_with(|id: TaskId, stack_region: MemoryRange| {
                let memory_config = match map {
                    Some(map) => MemoryConfig::user(map, stack_region),
                    None => MemoryConfig::kernel(),
                };

                return Task {
                    id,
                    state: TaskState::Ready,
                    domain,
                    priority,
                    parent_id,

                    memory_config,
                    saved_stack_pointer: null_mut(),
                    entry,
                    argument: argument as u32,
                };
            })
            .ok_or(SyscallError::OutOfResource);
    }

    pub fn kill_task(
        &mut self,
        caller_task_id: TaskId,
        target_task_id: TaskId,
    ) -> Result<bool, SyscallError> {
        if target_task_id == IDLE_TASK_ID {
            return Err(SyscallError::PermissionDenied);
        }

        let kill_current_task = self.current_task_id == Some(target_task_id);

        let caller_task_domain = self.task_table.get(caller_task_id).unwrap().domain;

        let target_task = self
            .task_table
            .get_mut(target_task_id)
            .ok_or(SyscallError::InvalidHandle)?;

        if caller_task_domain != TaskDomain::Kernel && target_task.domain == TaskDomain::Kernel {
            return Err(SyscallError::PermissionDenied);
        }

        if target_task.state == TaskState::Dead {
            return Err(SyscallError::NotRunning);
        }

        target_task.state = TaskState::Dead;
        if !kill_current_task {
            self.task_table.remove(target_task_id);
        }

        Ok(kill_current_task)
    }

    fn activate_next_ready_task(&mut self) -> *mut u32 {
        let start_index = match self.current_task_id {
            // wrap task ids inside the task_table (% MAX_TASKS)
            Some(task_id) => (task_id + 1) % MAX_TASKS,
            None => 0,
        };

        let mut next_task_id = IDLE_TASK_ID;
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

            next_task_id = task_id;
            break;
        }

        let next_task = self
            .task_table
            .get_mut(next_task_id)
            .expect("Selected task disappeared");

        assert!(next_task.state == TaskState::Ready);
        next_task.state = TaskState::Running;
        self.current_task_id = Some(next_task_id);

        next_task.memory_config.apply();
        match next_task.domain {
            TaskDomain::Kernel => asm::set_privileged_state(false),
            TaskDomain::User(_) => asm::set_privileged_state(true),
        }
        next_task.saved_stack_pointer
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

            method(scheduler)
        });
    }
}

/// kernel singleton scheduler wrapper
pub(crate) static SCHEDULER: SchedulerWrapper = SchedulerWrapper::new();
