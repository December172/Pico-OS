use core::panic::PanicInfo;

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".boot2")]
pub static boot2: [u8; 256] = *include_bytes!("boot2_arm.bin");

#[cfg(target_arch = "riscv32")]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".boot2")]
pub static boot2: [u8; 256] = *include_bytes!("boot2_riscv.bin");

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

/// ARM entry point
#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[unsafe(link_section = ".text._start")]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "bl entry"
    );
}
