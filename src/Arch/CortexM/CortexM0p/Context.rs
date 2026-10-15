use core::mem::size_of;

use crate::Arch::CortexM::General::ExceptionFrame::BasicExceptionFrame;

pub type TaskEntry = extern "C" fn(*mut ()) -> !;
pub type TaskExit = extern "C" fn() -> !;

#[repr(C)]
pub struct SoftwareContext {
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
}

#[repr(C, align(8))]
pub struct InitialTaskContext {
    pub softwareContext: SoftwareContext,
    pub hardwareFrame: BasicExceptionFrame,
}

pub fn initializeTaskStack(
    stackBase: *mut u32,
    stackWords: usize,
    taskEntry: TaskEntry,
    argument: *mut (),
    taskExit: TaskExit,
) -> *mut u32 {
    assert!(stackWords * size_of::<u32>() >= size_of::<InitialTaskContext>());

    let stackTop = unsafe { stackBase.add(stackWords) } as usize;

    let frameAddress = (stackTop - size_of::<InitialTaskContext>()) & !0x7;

    let framePointer = frameAddress as *mut InitialTaskContext;

    unsafe {
        framePointer.write(InitialTaskContext {
            softwareContext: SoftwareContext {
                r4: 0,
                r5: 0,
                r6: 0,
                r7: 0,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
            },
            hardwareFrame: BasicExceptionFrame {
                r0: argument as usize as u32,
                r1: 0,
                r2: 0,
                r3: 0,
                r12: 0,
                lr: taskExit as usize as u32,
                pc: ((taskEntry as usize) & !1) as u32,
                xpsr: 1 << 24,
            },
        });
    }

    framePointer.cast::<u32>()
}
