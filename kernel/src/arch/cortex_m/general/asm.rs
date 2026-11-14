use crate::util::register::Register;

use crate::native::constants::current::ppb::{PPB_ICSR, PPB_ICSR_PENDSVSET_BIT};

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

    output
}

#[inline(always)]
pub fn pend_pendsv() {
    unsafe {
        let register = Register::new(PPB_ICSR);
        register.write(1u32 << PPB_ICSR_PENDSVSET_BIT);

        core::arch::asm!("dsb", "isb", options(nostack, preserves_flags),);
    }
}

#[inline(always)]
pub fn pull_idle_task() {
    unsafe {
        core::arch::asm!(
            "mov r12, r4",

            "svc  #0",

            in("r4") const { 0 },
        );
    }
}

/// CONTROL.nPRIV will take effect after the syscall
#[inline(always)]
pub fn set_privileged_state(user: bool) {
    unsafe {
        core::arch::asm!(
            "msr CONTROL, {control}",
            "isb",
            control = in(reg) u32::from(user),
            options(nostack, preserves_flags),
        );
    }
}
