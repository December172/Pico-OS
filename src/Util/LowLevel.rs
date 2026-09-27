/// used before clock system successfully initialized.
/// 
/// at this point, we cannot trust hardware tick counter
pub fn _poll(count: u32, func: impl Fn() -> bool) -> bool {
    let mut counter = count;
    while !func() {
        counter -= 1;
        core::hint::spin_loop();
        if counter == 0 {
            return false;
        }
    }
    return true;
}

pub fn _breakpoint() {
    unsafe {
        core::arch::asm!("bkpt #1");
    }
}