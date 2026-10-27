SECTIONS
{
    .user.header ORIGIN(USER_FLASH) :
    {
        __user_image_start = .;

        /* UserImageHeader, in exactly the same order as pico-abi. */
        LONG(__image_magic)
        SHORT(__image_format_version)
        SHORT(__kernel_abi_version)
        LONG(__user_image_end - __user_image_start)
        __user_image_checksum = .;
        LONG(0) /* Patched by tools/pack_user_image.py after linking. */
        LONG(__user_entry_table_start - __user_image_start)
        LONG((__user_entry_table_end - __user_entry_table_start) / __task_entry_size)
        LONG(__user_segment_start - __user_image_start)
        LONG(0) /* image flags */
    } > USER_FLASH

    ASSERT(SIZEOF(.user.header) == __image_header_size,
           "UserImageHeader and linker layout disagree")
    ASSERT(__user_image_checksum - __user_image_start == __image_checksum_offset,
           "checksum offset and UserImageHeader disagree")

    .user.entries : ALIGN(4)
    {
        __user_entry_table_start = .;
        INCLUDE task-entries.ld
        __user_entry_table_end = .;
    } > USER_FLASH

    ASSERT(SIZEOF(.user.entries) > 0,
           "user image needs at least one task entry")
    ASSERT(SIZEOF(.user.entries) % __task_entry_size == 0,
           "invalid UserTaskEntry table size")

    .user.segment : ALIGN(4)
    {
        __user_segment_start = .;

        /* One RW, non-executable segment combines .data and zero-filled .bss. */
        LONG(__user_data_load_start - __user_image_start)
        LONG(__user_data_start)
        LONG(__user_data_end - __user_data_start)
        LONG(__user_bss_end - __user_data_start)
        LONG(__user_ram_flags)
    } > USER_FLASH

    ASSERT(SIZEOF(.user.segment) == __image_segment_size,
           "UserImageSegment and linker layout disagree")

    .user.text : ALIGN(4)
    {
        __user_text_start = .;
        KEEP(*(.text.user_entry.*))
        KEEP(*(.user.entry.*))
        *(.text .text.* .user.text .user.text.*)
        . = ALIGN(4);
        __user_text_end = .;
    } > USER_FLASH

    .user.rodata : ALIGN(4)
    {
        *(.rodata .rodata.* .user.rodata .user.rodata.*)
        . = ALIGN(4);
        __user_ro_end = .;
    } > USER_FLASH

    /* VMA is RAM; the initial bytes (LMA) follow the read-only flash content. */
    .user.data ORIGIN(USER_RAM) : ALIGN(8)
    {
        __user_data_start = .;
        *(.data .data.* .sdata .sdata.* .user.data .user.data.*)
        . = ALIGN(8);
        __user_data_end = .;
    } > USER_RAM AT> USER_FLASH

    __user_data_load_start = LOADADDR(.user.data);
    __user_data_load_end = LOADADDR(.user.data) + SIZEOF(.user.data);

    .user.bss (NOLOAD) : ALIGN(8)
    {
        __user_bss_start = .;
        *(.bss .bss.* .sbss .sbss.* .user.bss .user.bss.*)
        *(COMMON)
        . = ALIGN(8);
        __user_bss_end = .;
    } > USER_RAM

    __user_ram_used_end = __user_bss_end;

    /*
     * After .bss, '.' is a RAM VMA. Explicitly return to the flash LMA.
     * Materialize a final zero word so objcopy includes the entire declared
     * image even when .data is empty and its LMA introduced alignment padding.
     */
    __user_payload_end = MAX(__user_ro_end, __user_data_load_end);
    .user.image_tail __user_payload_end :
    {
        LONG(0)
        __user_image_end = .;
    } > USER_FLASH

    ASSERT(__user_image_end <= __user_flash_end,
           "user image exceeds USER_FLASH")
    ASSERT(__user_ram_used_end <= __user_ram_end,
           "user data/bss exceeds USER_RAM")

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.* .ARM.extab .ARM.extab.*)
        *(.eh_frame .eh_frame.* .eh_frame_hdr)
        *(.comment .note .note.*)
    }
}
