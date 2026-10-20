OUTPUT_FORMAT("elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

SECTIONS
{
    /*
     * RP2040 boot ROM loads these 256 bytes into SRAM, verifies the
     * checksum and executes them. boot2 configures XIP, then transfers
     * control through the vector table at 0x10000100.
     */
    .boot2 ORIGIN(FLASH) :
    {
        __boot2_start = .;
        KEEP(*(.boot2))
        KEEP(*(.boot2.*))
        __boot2_end = .;
    } > FLASH

    ASSERT(ADDR(.boot2) == 0x10000000,
           ".boot2 must start at 0x10000000")

    ASSERT(SIZEOF(.boot2) == 0x100,
           ".boot2 must be exactly 256 bytes")

    /*
     * RP2040 VTOR requires 256-byte alignment for this vector-table size.
     * The standard boot2 expects the application image here.
     */
    .vector_table (ORIGIN(FLASH) + 0x100) :
    {
        __vector_table_start = .;
        KEEP(*(.vector_table))
        KEEP(*(.vector_table.*))
        . = ALIGN(4);
        __vector_table_end = .;
    } > FLASH

    ASSERT((ADDR(.vector_table) & 0xff) == 0,
           ".vector_table must be 256-byte aligned")

    ASSERT(SIZEOF(.vector_table) <= 0x100,
           "RP2040 vector table is unexpectedly large")
    
    .text : ALIGN(4)
    {
        __text_start = .;

        /*
         * Keep the reset handler even when --gc-sections is enabled.
         */
        KEEP(*(.text._start))

        *(.text)
        *(.text.*)

        . = ALIGN(4);
        __text_end = .;
    } > FLASH


    .rodata : ALIGN(4)
    {
        __rodata_start = .;

        *(.rodata)
        *(.rodata.*)

        . = ALIGN(4);
        __rodata_end = .;
    } > FLASH

    /*
     * Prelinked user image.
     *
     * The user ELF is converted to a flat binary and then wrapped in
     * a relocatable object whose input section is named `.user.image`.
     *
     * The user image was already linked for ORIGIN(USER_FLASH), so it
     * must be placed here without further relocation.
     */
    .user_image ORIGIN(USER_FLASH) :
    {
        __user_image_start = .;

        KEEP(*(.user.image))

        . = ALIGN(4);
        __user_image_end = .;
    } > USER_FLASH

    ASSERT(__user_image_start == __user_flash_start,
           "user image must start at USER_FLASH origin")

    ASSERT(SIZEOF(.user_image) <= LENGTH(USER_FLASH),
           "user image exceeds USER_FLASH")

    ASSERT(__user_image_end <= __user_flash_end,
           "user image crosses USER_FLASH boundary")


    /*
     * VMA: RAM
     * LMA: flash, immediately following the preceding flash sections.
     *
     * runtimeInit must copy:
     *   __data_load_start -> __data_start..__data_end
     */
    .data : ALIGN(4)
    {
        __data_start = .;

        *(.data)
        *(.data.*)
        *(.sdata)
        *(.sdata.*)

        /*
         * Functions explicitly requested to run from RAM can use
         * #[link_section = ".ram_func"].
         */
        *(.ram_func)
        *(.ram_func.*)

        . = ALIGN(4);
        __data_end = .;
    } > RAM AT> FLASH

    __data_load_start = LOADADDR(.data);
    __data_load_end = LOADADDR(.data) + SIZEOF(.data);


    /*
     * Zeroed by runtimeInit. NOLOAD keeps it out of the flash image.
     */
    .bss (NOLOAD) : ALIGN(4)
    {
        __bss_start = .;

        *(.bss)
        *(.bss.*)
        *(.sbss)
        *(.sbss.*)
        *(COMMON)

        . = ALIGN(4);
        __bss_end = .;
    } > RAM

    /*
     * Static task stacks. Stack arrays can select this section through
     * #[link_section = ".task_stacks"].
     */
    .task_stacks (NOLOAD) : ALIGN(8)
    {
        __task_stacks_start = .;

        KEEP(*(.task_stacks))
        KEEP(*(.task_stacks.*))

        . = ALIGN(8);
        __task_stacks_end = .;
    } > RAM

    /*
     * This only exposes an unused range. It does not require you to
     * implement a heap now.
     */
    __heap_start = ALIGN(8);
    __heap_end = ORIGIN(RAM) + LENGTH(RAM);

    ASSERT(__heap_start <= __heap_end,
           "main SRAM overflow")

    /*
     * Core 0 MSP/kernel stack. It grows downward from
     * __kernel_stack_top = 0x20042000.
     */
    .kernel_stack (NOLOAD) :
    {
        __kernel_stack_bottom = .;
        . += LENGTH(SCRATCH_Y);
        __kernel_stack_top = .;
    } > SCRATCH_Y

    /*
     * Keep Scratch X reserved for a future second-core stack.
     */
    .core1_stack (NOLOAD) :
    {
        __core1_stack_bottom = .;
        . += LENGTH(SCRATCH_X);
        __core1_stack_top = .;
    } > SCRATCH_X

    __flash_image_end = __data_load_end;

    ASSERT(__flash_image_end <= ORIGIN(FLASH) + LENGTH(FLASH),
           "flash overflow")

    ASSERT(__kernel_stack_top == 0x20042000,
           "vector-table initial MSP no longer matches kernel stack")

    ASSERT((__kernel_stack_top & 7) == 0,
           "kernel stack must be 8-byte aligned")

    /DISCARD/ :
    {
        *(.comment)
        *(.note)
        *(.note.*)
    }
}
