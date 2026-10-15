ENTRY(_start)

PHDRS
{
    flash PT_LOAD;
}

SECTIONS
{
    /* RP2350 boot image layout.
     *
     * Unlike the RP2040 there is no 256-byte boot2 stage at offset 0: the
     * RP2350 bootrom performs the initial XIP setup itself and then enters the
     * image through the classic Cortex-M vector table, which therefore has to
     * be the very first thing in flash. On top of that the bootrom requires a
     * picobin `IMAGE_DEF` block somewhere within the first 4 KiB of the image
     * to recognise it as a bootable RP2350 (ARM, secure) image.
     */

    .vector_table :
    {
        KEEP(*(.vector_table))
    } > FLASH :flash


    /* picobin IMAGE_DEF: EXE | Secure | Arm | RP2350 (0x1021).
     * No ENTRY_POINT item is emitted, so the bootrom takes the reset vector
     * from the vector table above. The trailing zero word is the "next block"
     * self-reference of this single-block loop. */
    .image_def :
    {
        . = ALIGN(4);
        LONG(0xffffded3)   /* PICOBIN_BLOCK_MARKER_START */
        BYTE(0x42)         /* PICOBIN_BLOCK_ITEM_1BS_IMAGE_TYPE */
        BYTE(0x01)         /* item size: 1 word */
        SHORT(0x1021)      /* EXE(1) | SECURE(2<<4) | ARM(0<<8) | RP2350(1<<12) */
        BYTE(0xff)         /* PICOBIN_BLOCK_ITEM_2BS_LAST */
        SHORT(0x0001)      /* item size: 1 word */
        BYTE(0x00)         /* pad */
        LONG(0x00000000)   /* next block: 0 -> single self-referencing block */
        LONG(0xab123579)   /* PICOBIN_BLOCK_MARKER_END */
    } > FLASH :flash


    /* Optional XIP setup image. On RP2350 this is *not* executed by the
     * bootrom, so it is only kept here for reference/debug builds. */
    .boot2 :
    {
        KEEP(*(.boot2))
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
