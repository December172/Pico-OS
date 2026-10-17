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
    pub sys_tick: Handler,

    pub irq: [Handler; 32],
}

unsafe extern "C" {
    fn _start() -> !;
    #[link_name = "svcHandler"]
    fn svc_handler() -> !;
    #[link_name = "pendSvHandler"]
    fn pend_sv_handler() -> !;
}

#[unsafe(link_section = ".vector_table")]
#[unsafe(export_name = "vectorTable")]
pub static VECTOR_TABLE: VectorTable = VectorTable {
    initial_sp: 0x2004_2000,

    reset: _start,
    nmi: default_handler,
    hard_fault: default_handler,

    reserved: [0; 7],

    svcall: svc_handler,

    reserved2: [0; 2],

    pendsv: pend_sv_handler,
    sys_tick: default_handler,

    irq: [default_handler; 32],
};

#[unsafe(export_name = "defaultHandler")]
pub extern "C" fn default_handler() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
