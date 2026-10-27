#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resume_task_context(_saved_sp: *mut u32) -> ! {
    core::arch::naked_asm!(
        "ldmia r0!, {{r4-r7}}",
        "mov   r8,  r4",
        "mov   r9,  r5",
        "mov   r10, r6",
        "mov   r11, r7",
        "ldmia r0!, {{r4-r7}}",
        "msr   psp, r0",
        "movs r0, #1",
        "msr  CONTROL, r0",
        "isb",
        "movs  r0,  #2",
        "mvns  r0,  r0",
        "mov   lr,  r0",
        "bx    lr",
    )
}
