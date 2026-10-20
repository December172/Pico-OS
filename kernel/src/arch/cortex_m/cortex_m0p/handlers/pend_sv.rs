use crate::arch::current::context::SoftwareFrame;
use crate::arch::general::exception_frame::HardwareFrame;
use crate::kernel::tasks::scheduler::SCHEDULER;

pub unsafe extern "C" fn switch_bridge(current_stack_pointer: *mut u32) -> *mut u32 {
    unsafe {
        return SCHEDULER.with_exclusive(|scheduler| {
            return scheduler.schedule_next_task(current_stack_pointer);
        });
    }
}

pub unsafe extern "C" fn fresh_start_bridge() -> *mut u32 {
    unsafe {
        return SCHEDULER.with_exclusive(|scheduler| {
            return scheduler.start();
        });
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn pend_sv_handler() -> ! {
    core::arch::naked_asm!(
        "mov   r0,  lr",
        "movs  r1,  #4",  // r1 = 0b100
        "tst   r0,  r1",  // CPSR.eq = r0 & r1
        "bne   .Lnormal_switch",

        ".Lfresh_start:",
        "bl    {fresh_start_bridge}", // now r0 = current task stack pointer
        "mrs   r1,  msp",
        "adds  r1,  #{hardware_frame_size}",
        "msr   msp, r1",
        "b     .Lrestore",

        ".Lnormal_switch:",
        "mrs   r0,  psp",
        "subs  r0,  #{software_frame_size}",
        "mov   r1,  r0",
        "mov   r2,  r8",
        "mov   r3,  r9",
        "stmia r1!, {{r2-r3}}",
        "mov   r2,  r10",
        "mov   r3,  r11",
        "stmia r1!, {{r2-r3}}",
        "stmia r1!, {{r4-r7}}",
        "bl    {switch_bridge}",

        ".Lrestore:",
        "ldmia r0!, {{r4-r7}}",
        "mov   r8,  r4",
        "mov   r9,  r5",
        "mov   r10, r6",
        "mov   r11, r7",
        "ldmia r0!, {{r4-r7}}",
        "msr   psp, r0",

        "movs  r0,  #2",
        "mvns  r0,  r0",
        "mov   lr,  r0",
        "bx    lr",

        fresh_start_bridge = sym fresh_start_bridge,
        switch_bridge = sym switch_bridge,

        hardware_frame_size = const(size_of::<HardwareFrame>()),
        software_frame_size = const(size_of::<SoftwareFrame>()),

    )
}
