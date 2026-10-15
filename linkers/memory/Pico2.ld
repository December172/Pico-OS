/* Raspberry Pi Pico 2: 4 MiB flash and the RP2350 main SRAM banks. */
MEMORY
{
    FLASH(rx) : ORIGIN = 0x10000000, LENGTH = 4096K
    RAM(rwx)  : ORIGIN = 0x20000000, LENGTH = 512K
}
