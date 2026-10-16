use crate::Arch::CortexM::General::ASM::pendPendSv;
use crate::Arch::CortexM::General::ExceptionFrame::HardwareFrame;

use crate::Kernel::Syscalls::Syscall::*;
use crate::Kernel::Syscalls::SyscallRegistry::SYSCALL_REGISTRY;
use crate::Kernel::Tasks::Scheduler::SCHEDULER;

#[unsafe(no_mangle)]
unsafe extern "C" fn svcBridge(framePtr: *mut HardwareFrame, tableId: u32, fromKernel: u32) {
    let frame = unsafe { &mut *framePtr };

    if tableId == 0 {
        if fromKernel != 0x1 {
            frame.r0 = SyscallError::PermissionDenied as u32;
            frame.r1 = 0;
        } else {
            // `svc #0`: pull up idle task
            pendPendSv();
        }
        return;
    }

    let currentTaskId =
        unsafe { SCHEDULER.withExclusive(|scheduler| scheduler.getCurrentTaskId()) };

    let mut context = SyscallContext {
        currentTaskId: currentTaskId
            .unwrap_or_else(|| panic!("ordinary SVC entered without a running task")),
    };

    let result = SYSCALL_REGISTRY.dispatch(
        tableId as u8,
        frame.r12,
        context,
        [frame.r0, frame.r1, frame.r2, frame.r3],
    );
    frame.r0 = result.status as u32;
    frame.r1 = result.value;

    if result.action == SyscallAction::Reschedule {
        pendPendSv();
    }
}

/// The main entry of syscalls
///
/// asm: `svc #n`. the number `n` should be registered at kernel init
/// # Internals
/// 1. Hardware frames have been saved to stack (user / kernel stack) when calling this (<SP> = MSP; handler uses kernel stack)
///
/// 2. Do some works
///
/// 3. Normal return (non-task syscalls) / tail-chain by pendPendSv() (for rescheduling)

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn svcHandler() -> ! {
    // the bridge between kernel space and user space;
    // privileged code entry, isn't it exciting?

    // current reg structure:
    // r0-r3: arguments of syscall
    // r4-r11: caller's current reg state - only used for reschdeuling in pendSvHandler().dont touch them at here
    // r12: syscall id
    // r13 / sp: msp (kernel's stack pointer)
    // r14 / lr: EXC_RETURN
    // r15 / pc: <entry address of svcHandler> (current line)
    // stacks structure: (for normal userspace syscall)
    // psp top: basicExceptionFrame.r0
    // msp top: <current kernel stack top>
    // where:
    // basicExceptionFrame.lr: caller's lr
    // basicExceptionFrame.pc: caller's pc (syscall returns to this normally)
    // basicExceptionFrame.r0-r3, r12: caller's current reg state
    core::arch::naked_asm!(
        // EXC_RETURN[2]:
        // 0 : msp = <exception frame>
        // 1 : psp = <exception frame>
                                  // 1. Get *basicException
        "mov  r2, lr",            // r2 = lr = EXC_RETURN
        "movs r3, #4",            // r3 = 0b100
        "tst  r2, r3",            // CPSR.eq = r2 & r3
        "beq  .Lsvc_from_msp",    // CPSR.eq == 0: jump to label svc_from_msp (EXC_RETURN[2]: 0)

                                  // (EXC_RETURN[2]: 1): from user space syscall
        "mrs  r0, psp",           // r0 = psp
        "movs r2, #0",            // r2 = 0x0 (not from kernel)
        "b    .Lsvc_frame_ready", // jump to label svc_frame_ready (normal routine)

        ".Lsvc_from_msp:",        // (EXC_RETURN[2]: 0): from kernel space syscall
        "mrs  r0, msp",           // r0 = msp
        "movs r2, #1",            // r2 = 0x1 (from kernel)
                                  // continue to label 3 (normal routine)

        ".Lsvc_frame_ready:",     // normal routine; r0 = *basicExceptionFrame

                                  // 2. Get the svc immediate #n
        "ldr  r1, [r0, #24]",     // r1 = *basicException + 0x24: caller's original pc (stacked pc)
        "subs r1, #2",            // r1 -= 0x2: get the address of caller's `svc #n` line (opcode length: 0x2)
                                  // where: caller's original pc points to the next opcode of the svc opcode
        "ldrb r1, [r1]",          // r1 = <svc immediate #n> (load first byte of the opcode address)

                                  // 3. Save EXC_RETURN
        "push {{r3, lr}}",        // rust calling conversion - keep 8-byte alignment (save r3)

        "bl   {rust_entry}",      // lr will be overwriten

        "pop  {{r3, pc}}",        // pc = stored lr; exception return / PendSV tail-chain

        rust_entry = sym svcBridge,
    )
}
