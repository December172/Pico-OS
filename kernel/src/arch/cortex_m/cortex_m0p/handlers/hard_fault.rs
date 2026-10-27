use pico_abi::syscall::SyscallError;

use crate::arch::cortex_m::cortex_m0p::handlers::general::resume_task_context;
use crate::arch::general::asm::pend_pendsv;

use crate::kernel::tasks::scheduler::SCHEDULER;

/// Do necessary preparations before jump in panic()
fn fatal_fault(use_msp: bool, from_handler: bool, old_psp: *mut u32, error: SyscallError) -> ! {
    panic!(
        "Kernel panicked due to hard_fault!, use_msp: {use_msp}; from_handler: {from_handler}; old_psp: {old_psp:?}; error: {error:?}"
    );
}

unsafe extern "C" fn hard_fault_bridge(exc_return: u32, old_psp: *mut u32) -> *mut u32 {
    let use_msp = if exc_return & 0x4 == 0 { true } else { false };

    let from_handler = if exc_return & 0x8 == 0 { true } else { false };

    if use_msp || from_handler {
        fatal_fault(use_msp, from_handler, old_psp, SyscallError::UnknownError);
    }

    let result = unsafe {
        SCHEDULER.with_exclusive(|scheduler| {
            return scheduler.discard_current_task();
        })
    };
    return result.unwrap_or_else(|error| {
        fatal_fault(use_msp, from_handler, old_psp, error);
    });
}

#[unsafe(naked)]
pub unsafe extern "C" fn hard_fault_handler() -> ! {
    core::arch::naked_asm!(
        "mov  r0, lr",
        "mrs  r1, psp",
        "bl   {bridge}",
        "bl   {resume}",
        "b    .",

        bridge = sym hard_fault_bridge,
        resume = sym resume_task_context
    )
}
