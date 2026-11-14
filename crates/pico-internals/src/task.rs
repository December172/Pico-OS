use pico_abi::syscall::SyscallError;
use pico_abi::task::{TASK_SYSCALL_TABLE_ID, TaskSyscallId, TASKID_SELF};

use crate::util::syscall_wrapper;

#[doc(hidden)]
#[macro_export]
macro_rules! task_trampoline {
    (
        $(#[$attr:meta])*
        $vis:vis fn $name:ident($argument:ident: *mut ()) $body:block
    ) => {
        $(#[$attr])*
        #[unsafe(no_mangle)]  // for entry address in linker script
        $vis extern "C" fn $name(argument: *mut ()) -> ! {
            fn task_body($argument: *mut()) $body

            task_body(argument);

            $crate::task::exit();
        }
    };

    ($($other:tt)*) => {
        ::core::compile_error!(
            "expected fn name(argument: *mut ()) { ... }"
        );
    };
}

pub fn create_task(entry: u32, argument: u32) -> Result<u32, SyscallError> {
    let raw_result = unsafe {
        syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(
            TaskSyscallId::Create as u32,
            [entry, argument, 0, 0],
        )
    };

    if raw_result.status == 0 {
        Ok(raw_result.value)
    } else {
        Err(SyscallError::try_from(raw_result.status).expect("Unknown syscall error"))
    }
}

pub fn yield_task() {
    unsafe {
        syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(TaskSyscallId::Yield as u32, [0; 4]);
    }
}

pub fn kill_task(target: u32) -> Result<(), SyscallError> {
    let result = unsafe {
        syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(TaskSyscallId::Kill as u32, [target, 0, 0, 0])
    };

    if result.status == 0 {
        return Ok(());
    }

    Err(SyscallError::try_from(result.status).expect("Unknown syscall error"))
}

pub fn get_current_task_id() -> u32 {
    let result = unsafe {
        syscall_wrapper::<TASK_SYSCALL_TABLE_ID>(TaskSyscallId::GetCurrentTaskId as u32, [0; 4])
    };

    result.value
}

pub fn exit() -> ! {
    match kill_task(TASKID_SELF) {
        Ok(()) => unreachable!(),
        Err(error) => panic!("Task exit failed: {}", error as i32),
    }
}
