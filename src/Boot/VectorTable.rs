type Handler = unsafe extern "C" fn() -> !;

#[repr(C)]
pub struct VectorTable {
    pub initial_sp: u32,

    pub reset: Handler,
    pub nmi: Handler,
    pub hard_fault: Handler,

    pub reserved: [u32; 7],

    pub svcall: Handler,

    pub reserved2: [u32; 2],

    pub pendsv: Handler,
    pub systick: Handler,

    pub irq: [Handler; 32],
}

extern "C" {
    fn _start() -> !;
}

#[unsafe(link_section = ".vector_table")]
#[unsafe(no_mangle)]
pub static VECTOR_TABLE: VectorTable = VectorTable {
    initial_sp: 0x20042000,

    reset: _start,
    nmi: default_handler,
    hard_fault: default_handler,

    reserved: [0; 7],

    svcall: default_handler,

    reserved2: [0; 2],

    pendsv: default_handler,
    systick: default_handler,

    irq: [default_handler; 32],
};

#[unsafe(no_mangle)]
pub extern "C" fn default_handler() -> ! {
    loop {
        unsafe {
           core::arch::asm!("wfi");
        }
    }
}