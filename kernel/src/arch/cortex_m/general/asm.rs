// TODO: SMP support (requires hardware spinlock)
#[inline(always)]
pub fn with_interrupts_disabled<Output>(method: impl FnOnce() -> Output) -> Output {
    let saved_primask: u32;

    unsafe {
        core::arch::asm!(
            "mrs {saved_primask}, primask",
            "cpsid i",
            saved_primask = out(reg) saved_primask,
            options(nostack, preserves_flags),
        );
    }

    let output = method();

    unsafe {
        core::arch::asm!(
            "msr primask, {saved_primask}",
            saved_primask = in(reg) saved_primask,
            options(nostack, preserves_flags),
        );
    }

    return output;
}

const SCB_ICSR: *mut u32 = 0xE000_ED04 as *mut u32;
const SCB_ICSR_PENDSVSET: u32 = 1 << 28;

#[inline(always)]
pub fn pend_pend_sv() {
    unsafe {
        core::ptr::write_volatile(SCB_ICSR, SCB_ICSR_PENDSVSET);

        core::arch::asm!("dsb", "isb", options(nostack, preserves_flags),);
    }
}
