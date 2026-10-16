use crate::Arch::General::ExceptionFrame::HardwareFrame;
use crate::Arch::Current::Context::SoftwareFrame;
use crate::Kernel::Tasks::Scheduler::SCHEDULER;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn switchBridge(currentStackPointer: *mut u32) -> *mut u32{
    unsafe {
        return SCHEDULER.withExclusive(|scheduler| {
            return scheduler.scheduleNextTask(currentStackPointer);
        });
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn freshStartBridge() -> *mut u32{
    unsafe {
        return SCHEDULER.withExclusive(|scheduler| {
            return scheduler.start();
        });
    }
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn pendSvHandler() -> ! {
    core::arch::naked_asm!(
        "mov   r0,  lr",
        "movs  r1,  #4",  // r1 = 0b100 
        "tst   r0,  r1",  // CPSR.eq = r0 & r1
        "bne   .Lnormal_switch",

        ".Lfresh_start:",
        "bl    {freshStartBridge}", // now r0 = current task stack pointer
        "mrs   r1,  msp",
        "adds  r1,  #{hardwareFrameSize}",
        "msr   msp, r1",
        "b     .Lrestore",

        ".Lnormal_switch:",
        "mrs   r0,  psp",
        "subs  r0,  #{softwareFrameSize}",
        "mov   r1,  r0",
        "mov   r2,  r8",
        "mov   r3,  r9",
        "stmia r1!, {{r2-r3}}",
        "mov   r2,  r10",
        "mov   r3,  r11",
        "stmia r1!, {{r2-r3}}",
        "stmia r1!, {{r4-r7}}",
        "bl    {switchBridge}",

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

        freshStartBridge  = sym freshStartBridge,
        switchBridge      = sym switchBridge,

        hardwareFrameSize = const(size_of::<HardwareFrame>()),
        softwareFrameSize = const(size_of::<SoftwareFrame>()),

    )
}