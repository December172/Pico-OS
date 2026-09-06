#[link_section = ".boot2"]
#[no_mangle]
pub static BOOT2: [u8; 256] = *include_bytes!("boot2.bin");