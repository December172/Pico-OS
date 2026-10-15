/* Raspberry Pi Pico: 2 MiB flash and the RP2040 SRAM banks. */
MEMORY
{
    FLASH(rx)      : ORIGIN = 0x10000000, LENGTH = 2048K
    RAM(rwx)       : ORIGIN = 0x20000000, LENGTH = 256K
    SCRATCH_X(rwx) : ORIGIN = 0x20040000, LENGTH = 4K
    SCRATCH_Y(rwx) : ORIGIN = 0x20041000, LENGTH = 4K
}
