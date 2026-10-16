unsafe extern "C" {
    static __data_load_start: u8;

    static mut __data_start: u8;
    static mut __data_end: u8;

    static mut __bss_start: u8;
    static mut __bss_end: u8;
}

/// Initialize Rust's writable static memory.
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn runtimeInit() {
    // Copy the initial .data image from flash into SRAM.
    let dataSource = unsafe { core::ptr::addr_of!(__data_load_start) };
    let dataStart = unsafe { core::ptr::addr_of_mut!(__data_start) };
    let dataEnd = unsafe { core::ptr::addr_of_mut!(__data_end) };

    let dataLength = dataEnd as usize - dataStart as usize;

    let mut offset = 0;

    while offset < dataLength {
        let value = unsafe { dataSource.wrapping_add(offset).read_volatile() };

        unsafe {
            dataStart.wrapping_add(offset).write_volatile(value);
        }

        offset += 1;
    }

    // Zero-initialize .bss.
    let bssStart = unsafe { core::ptr::addr_of_mut!(__bss_start) };
    let bssEnd = unsafe { core::ptr::addr_of_mut!(__bss_end) };

    let bssLength = bssEnd as usize - bssStart as usize;

    offset = 0;

    while offset < bssLength {
        unsafe {
            bssStart.wrapping_add(offset).write_volatile(0);
        }

        offset += 1;
    }
}
