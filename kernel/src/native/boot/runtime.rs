unsafe extern "C" {
    static __data_load_start: u8;

    static mut __data_start: u8;
    static mut __data_end: u8;

    static mut __bss_start: u8;
    static mut __bss_end: u8;
}

/// Initialize Rust's writable static memory.
#[inline(never)]
pub unsafe extern "C" fn runtime_init() {
    // Copy the initial .data image from flash into SRAM.
    let data_source = unsafe { core::ptr::addr_of!(__data_load_start) };
    let data_start = unsafe { core::ptr::addr_of_mut!(__data_start) };
    let data_end = unsafe { core::ptr::addr_of_mut!(__data_end) };

    let data_length = data_end as usize - data_start as usize;

    unsafe {
        core::ptr::copy_nonoverlapping(data_source, data_start, data_length);
    }

    // Zero-initialize .bss.
    let bss_start = unsafe { core::ptr::addr_of_mut!(__bss_start) };
    let bss_end = unsafe { core::ptr::addr_of_mut!(__bss_end) };

    let bss_length = bss_end as usize - bss_start as usize;

    unsafe {
        core::ptr::write_bytes(bss_start, 0, bss_length);
    }
}
