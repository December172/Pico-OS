//! RP2350 vector / trap tables.
//!
//! The layout of the start of the image differs per core:
//!
//! * **Arm** – the RP2350 bootrom enters the image through the classic
//!   Cortex-M vector table (stack pointer + reset vector) placed in
//!   `.vector_table`.
//! * **RISC-V** – the bootrom has no vector-table convention for RISC-V: it
//!   reads the entry point and stack pointer from the *picobin* `IMAGE_DEF`
//!   block, which must live within the first 4 KiB of flash. That block, the
//!   reset stub and the machine trap (`mtvec`) table are emitted here, at the
//!   start of `.vector_table`, so the whole boot path stays inside this module.

#[cfg(target_arch = "arm")]
type Handler = unsafe extern "C" fn() -> !;

#[cfg(target_arch = "arm")]
#[repr(C)]
pub struct VectorTable {
    pub initialSp: u32,

    pub reset: Handler,
    pub nmi: Handler,
    pub hardFault: Handler,

    pub reserved: [u32; 7],

    pub svcall: Handler,

    pub reserved2: [u32; 2],

    pub pendsv: Handler,
    pub sysTick: Handler,

    pub irq: [Handler; 52],
}

#[cfg(target_arch = "arm")]
unsafe extern "C" {
    fn _start() -> !;
}

#[cfg(target_arch = "arm")]
#[unsafe(link_section = ".vector_table")]
#[unsafe(no_mangle)]
pub static vectorTable: VectorTable = VectorTable {
    initialSp: 0x2008_0000,

    reset: _start,
    nmi: defaultHandler,
    hardFault: defaultHandler,

    reserved: [0; 7],

    svcall: defaultHandler,

    reserved2: [0; 2],

    pendsv: defaultHandler,
    sysTick: defaultHandler,

    irq: [defaultHandler; 52],
};

/// Default exception/trap handler: park the core.
#[unsafe(no_mangle)]
pub extern "C" fn defaultHandler() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

// TODO: refactor this asm code
// ---------------------------------------------------------------------------
// RISC-V boot path
// ---------------------------------------------------------------------------
//
// Layout placed at the start of `.vector_table` (image base, 256-byte aligned):
//
//   +0x00  picobin IMAGE_DEF block:
//            IMAGE_TYPE = EXE | CPU=RISCV | CHIP=RP2350
//            ENTRY_POINT = _start, sp = 0x2008_0000
//          (the RP2350 bootrom scans the first 4 KiB of flash for this block,
//           switches the chip to RISC-V if required, installs sp and jumps to
//           the entry point)
//   +0x24  _start reset stub
//   ...    machine trap table (referenced by mtvec)
//
// No checksum is involved on RP2350; `boot2.bin` (256 bytes at the start of
// flash) is still kept as the optional XIP setup image.
#[cfg(target_arch = "riscv32")]
core::arch::global_asm!(
    r#"
    .section .vector_table, "ax"
    .p2align 8

    // ---- picobin IMAGE_DEF block ------------------------------------------
    .word 0xffffded3                 // PICOBIN_BLOCK_MARKER_START
    // IMAGE_TYPE: EXE (0x1) | CPU=RISCV (0x100) | CHIP=RP2350 (0x1000)
    .byte 0x42, 0x1
    .hword 0x1101
    // ENTRY_POINT: pc = _start, sp = 0x20080000 (matches the Arm table)
    .byte 0x44, 0x3, 0x0, 0x0
    .word _start
    .word 0x20080000
    // LAST item: 4 preceding item words, single self-referencing block loop
    .byte 0xff
    .hword 4
    .byte 0x0
    .word 0
    .word 0xab123579                 // PICOBIN_BLOCK_MARKER_END
    .p2align 3

    // ---- reset / entry stub ----------------------------------------------
    .global _start
    .type _start, %function
_start:
    // The bootrom already installed sp from the ENTRY_POINT item, but set it
    // again so boot2/debugger entry paths behave the same.
    li sp, 0x20080000

    // Enable gp-relative addressing for the Rust runtime.
    .option push
    .option norelax
    .global __global_pointer$
__global_pointer$ = .
    la gp, __global_pointer$
    .option pop

    // Use the trap table below in vectored mode.
    la t0, __riscv_mtvec
    ori t0, t0, 1
    csrw mtvec, t0

    call entry

1:
    wfi
    j 1b
    .size _start, . - _start

    // ---- machine trap vector table (mtvec) --------------------------------
    // Vectored mode: entry `n` is fetched from base + 4*n.
    .p2align 6
    .global __riscv_mtvec
__riscv_mtvec:
    j defaultHandler                 // 0  machine exception
    .word 0
    .word 0
    j defaultHandler                 // 3  machine software interrupt
    .word 0
    .word 0
    .word 0
    j defaultHandler                 // 7  machine timer interrupt
    .word 0
    .word 0
    .word 0
    j defaultHandler                 // 11 machine external interrupt
    .word 0
    .word 0
    .word 0
    .word 0
    "#
);
