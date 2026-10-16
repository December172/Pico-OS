use core::mem::size_of;

use crate::Arch::CortexM::General::ExceptionFrame::HardwareFrame;

use crate::Kernel::Tasks::Task::Task;

const CONTEXT_WORDS: usize = size_of::<InitialTaskContext>() / size_of::<u32>();

pub type TaskEntry = extern "C" fn(*mut Task);
pub type TaskExit = extern "C" fn() -> !;

/// the regs layout is intentional - for symmetric load / save
#[repr(C)]
pub struct SoftwareFrame {
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
}

#[repr(C, align(8))]
pub struct InitialTaskContext {
    pub softwareContext: SoftwareFrame,
    pub hardwareFrame: HardwareFrame,
}

pub fn initializeTaskStack(
    stack: &'static mut [u32],
    taskEntry: TaskEntry,
    taskBody: *mut Task,
    taskExit: TaskExit,
) -> *mut u32 {
    let stackTop = unsafe {
        // Note that raw pointer's `.add()` / `.sub` shifts the pointed address by size of the pointer's type (word length, for u32)
        stack.as_mut_ptr().add(stack.len())
    };

    let framePointer = unsafe {
        stackTop.sub(CONTEXT_WORDS).cast::<InitialTaskContext>()
    };

    unsafe {
        framePointer.write(InitialTaskContext {
            softwareContext: SoftwareFrame {
                r4: 0,
                r5: 0,
                r6: 0,
                r7: 0,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
            },
            hardwareFrame: HardwareFrame {
                r0: taskBody as u32,
                r1: 0,
                r2: 0,
                r3: 0,
                r12: 0,
                lr: taskExit as u32,
                pc: ((taskEntry as usize) & !1) as u32, // `& !1` to ensure pc[0] = 0; to ensure thumb mode
                xpsr: 1 << 24,
            },
        });
    }

    return framePointer.cast::<u32>();
}
