// TODO: SMP support (requires hardware spinlock)
#[inline(always)]
pub fn withInterruptsDisabled<Output>(method: impl FnOnce() -> Output) -> Output {
    let savedPrimask: u32;

    unsafe {
        core::arch::asm!(
            "mrs {savedPrimask}, primask",
            "cpsid i",
            savedPrimask = out(reg) savedPrimask,
            options(nostack, preserves_flags),
        );
    }

    let output = method();

    unsafe {
        core::arch::asm!(
            "msr primask, {savedPrimask}",
            savedPrimask = in(reg) savedPrimask,
            options(nostack, preserves_flags),
        );
    }

    return output;
}


const SCB_ICSR: *mut u32 = 0xE000_ED04 as *mut u32;
const SCB_ICSR_PENDSVSET: u32 = 1 << 28;

#[inline(always)]
pub fn pendPendSv() {
    unsafe {
        core::ptr::write_volatile(
            SCB_ICSR,
            SCB_ICSR_PENDSVSET,
        );

        core::arch::asm!(
            "dsb",
            "isb",
            options(nostack, preserves_flags),
        );
    }
}