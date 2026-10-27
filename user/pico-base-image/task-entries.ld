/* Base-image-specific records. Keep idle first; add other task veneers here. */
LONG((idle_task_entry & ~1) - __user_image_start)
LONG(__idle_task_flags)
LONG(0) /* reserved */
