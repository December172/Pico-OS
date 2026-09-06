ENTRY(_start)

PHDRS
{
    flash PT_LOAD;
}

MEMORY
{
    FLASH(rx) : ORIGIN = 0x10000000, LENGTH = 2048K
    RAM(rwx)  : ORIGIN = 0x20000000, LENGTH = 264K
}


SECTIONS
{
    .boot2 :
    {
        KEEP(*(.boot2))
    } > FLASH :flash


    .vector_table :
    {
        . = ALIGN(256);
        KEEP(*(.vector_table))
    } > FLASH :flash


    .text :
    {
         . = ALIGN(4);
        KEEP(*(.text._start))
        *(.text*)
        *(.rodata*)
    } > FLASH :flash

    /DISCARD/ :
    {
        *(.ARM.exidx*)
        *(.ARM.extab*)
        *(.comment*)
        *(.ARM.attributes*)
    }

}

