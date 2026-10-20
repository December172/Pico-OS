use crate::arch::cortex_m::cortex_m0p::handlers::pendsv_handler;
use crate::arch::cortex_m::cortex_m0p::handlers::svc_handler;
use crate::native::boot::rp2040::boot::_start;

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

#[unsafe(link_section = ".vector_table")]
pub static VECTOR_TABLE: VectorTable = VectorTable {
    initial_sp: 0x2004_2000,

    reset: _start,
    nmi: default_handler,
    hard_fault: default_handler,

    reserved: [0; 7],

    svcall: svc_handler,

    reserved2: [0; 2],

    pendsv: pendsv_handler,
    sys_tick: default_handler,

    irq: [default_handler; 32],
};

pub extern "C" fn default_handler() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
