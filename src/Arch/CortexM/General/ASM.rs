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
