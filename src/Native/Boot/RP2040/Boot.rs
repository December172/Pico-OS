use core::panic::PanicInfo;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".boot2")]
pub static boot2: [u8; 256] = *include_bytes!("boot2.bin");

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("bkpt #0");
    }
    loop {}
}

/// ASM entry point
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[unsafe(link_section = ".text._start")]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "bl runtimeInit",
        "bl entry",
        "b ."
    );
}