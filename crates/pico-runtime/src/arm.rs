#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn __aeabi_uread4(_address: *const u8) -> u32 {
    core::arch::naked_asm!(
        "mov  r1, r0",
        "ldrb r0, [r1, #0]",
        "ldrb r2, [r1, #1]",
        "lsls r2, r2, #8",
        "orrs r0, r2",
        "ldrb r2, [r1, #2]",
        "lsls r2, r2, #16",
        "orrs r0, r2",
        "ldrb r2, [r1, #3]",
        "lsls r2, r2, #24",
        "orrs r0, r2",
        "bx   lr"
    )
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn __aeabi_uread8(_address: *const u8) -> u64 {
    core::arch::naked_asm!(
        "push {{r4, r5, r6, lr}}",

        "mov  r4, r0",
        "bl   {uread4}",
        "mov  r5, r0",

        "adds r0, r4, #4",
        "bl   {uread4}",

        "mov  r1, r0",
        "mov  r0, r5",

       "pop {{r4, r5, r6, pc}}",

       uread4 = sym __aeabi_uread4
    )
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn __aeabi_uwrite4(_value: u32, _address: *mut u8) -> u32 {
    core::arch::naked_asm!(
        "mov  r2, r0",
        "strb r2, [r1, #0]", // store r2[7:0] to <addr + 0x0>
        "lsrs r2, r2, #8",
        "strb r2, [r1, #1]", // store r2[15:8] to <addr + 0x1>
        "lsrs r2, r2, #8",
        "strb r2, [r1, #2]", // store r2[23:16] to <addr + 0x2>
        "lsrs r2, r2, #8",
        "strb r2, [r1, #3]", // store r2[31:24] to <addr + 0x3>
        "bx   lr",
    )
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn __aeabi_uwrite8(_value: u64, _address: *mut u8) -> u64 {
    core::arch::naked_asm!(
        "push {{r4, r5, r6, lr}}",

        "mov  r4, r0",
        "mov  r5, r1",
        "mov  r6, r2",

        "mov  r1, r6",
        "bl   {uwrite4}",

        "mov  r0, r5",
        "adds r1, r6, #4",
        "bl   {uwrite4}",

        "mov  r0, r4",
        "mov  r1, r5",

        "pop  {{r4, r5, r6, pc}}",

       uwrite4 = sym __aeabi_uwrite4
    )
}
