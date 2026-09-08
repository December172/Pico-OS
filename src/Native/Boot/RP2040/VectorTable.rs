type Handler = unsafe extern "C" fn() -> !;

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

    pub irq: [Handler; 32],
}

unsafe extern "C" {
    fn _start() -> !;
}

#[unsafe(link_section = ".vector_table")]
#[unsafe(no_mangle)]
pub static vectorTable: VectorTable = VectorTable {
    initialSp: 0x20042000,

    reset: _start,
    nmi: defaultHandler,
    hardFault: defaultHandler,

    reserved: [0; 7],

    svcall: defaultHandler,

    reserved2: [0; 2],

    pendsv: defaultHandler,
    sysTick: defaultHandler,

    irq: [defaultHandler; 32],
};

#[unsafe(no_mangle)]
pub extern "C" fn defaultHandler() -> ! {
    loop {
        unsafe {
           core::arch::asm!("wfi");
        }
    }
}