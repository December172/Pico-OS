unsafe extern "C" {
    static __data_load_start: u8;

    static mut __data_start: u8;
    static mut __data_end: u8;

    static mut __bss_start: u8;
    static mut __bss_end: u8;
}

/// Initialize Rust's writable static memory.
#[inline(never)]
#[unsafe(export_name = "runtimeInit")]
pub unsafe extern "C" fn runtime_init() {
    // Copy the initial .data image from flash into SRAM.
    let data_source = unsafe { core::ptr::addr_of!(__data_load_start) };
    let data_start = unsafe { core::ptr::addr_of_mut!(__data_start) };
    let data_end = unsafe { core::ptr::addr_of_mut!(__data_end) };

    let data_length = data_end as usize - data_start as usize;

    let mut offset = 0;

    while offset < data_length {
        let value = unsafe { data_source.wrapping_add(offset).read_volatile() };

        unsafe {
            data_start.wrapping_add(offset).write_volatile(value);
        }

        offset += 1;
    }

    // Zero-initialize .bss.
    let bss_start = unsafe { core::ptr::addr_of_mut!(__bss_start) };
    let bss_end = unsafe { core::ptr::addr_of_mut!(__bss_end) };

    let bss_length = bss_end as usize - bss_start as usize;

    offset = 0;

    while offset < bss_length {
        unsafe {
            bss_start.wrapping_add(offset).write_volatile(0);
        }

        offset += 1;
    }
}
