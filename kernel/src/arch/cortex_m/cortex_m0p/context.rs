use core::mem::size_of;

use crate::arch::cortex_m::general::exception_frame::HardwareFrame;

use crate::kernel::tasks::task::TaskEntry;

const CONTEXT_WORDS: usize = size_of::<InitialTaskContext>() / size_of::<u32>();

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
    pub software_context: SoftwareFrame,
    pub hardware_frame: HardwareFrame,
}

pub fn initialize_task_stack(
    stack: &'static mut [u32],
    task_entry: TaskEntry,
    task_argument: u32,
) -> *mut u32 {
    let stack_top = unsafe {
        // Note that raw pointer's `.add()` / `.sub` shifts the pointed address by size of the pointer's type (word length, for u32)
        stack.as_mut_ptr().add(stack.len())
    };

    let frame_pointer = unsafe { stack_top.sub(CONTEXT_WORDS).cast::<InitialTaskContext>() };

    unsafe {
        frame_pointer.write(InitialTaskContext {
            software_context: SoftwareFrame {
                r4: 0,
                r5: 0,
                r6: 0,
                r7: 0,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
            },
            hardware_frame: HardwareFrame {
                r0: task_argument,
                r1: 0,
                r2: 0,
                r3: 0,
                r12: 0,
                lr: 0,
                pc: task_entry & !1, // `& !1` to ensure pc[0] = 0; to ensure thumb mode
                xpsr: 1 << 24,
            },
        });
    }

    return frame_pointer.cast::<u32>();
}
