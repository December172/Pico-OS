OUTPUT_FORMAT("elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(idle_task_entry)

/* Layout and section policy are shared; task-entries.ld belongs to this image. */
INCLUDE memory.ld
INCLUDE image-abi.ld
INCLUDE user-sections.ld
