pub const IMAGE_MAGIC: [u8; 4] = *b"PIOS";
pub const IMAGE_FORMAT_VERSION: u16 = 1;
pub const KERNEL_ABI_VERSION: u16 = 1;

/// This entry is the scheduler's idle task.
pub const USER_TASK_FLAG_IDLE: u32 = 1 << 0;

pub const IMAGE_SEGMENT_READ: u32 = 1 << 0; // 0b001
pub const IMAGE_SEGMENT_WRITE: u32 = 1 << 1; // 0b010
pub const IMAGE_SEGMENT_EXECUTE: u32 = 1 << 2; // 0b100

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserImageHeader {
    pub magic: [u8; 4],
    pub format_version: u16,
    pub abi_version: u16,

    /// Serialized flash bytes, including padding; excludes zero-filled RAM.
    pub image_size: u32,
    /// CRC-32/ISO-HDLC of image_size bytes with this field treated as zero.
    pub crc32: u32,

    pub entry_table_offset: u32,
    pub entry_count: u32,

    /// Offset to this image's single UserImageSegment record.
    pub segment_offset: u32,

    pub flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserTaskEntry {
    pub entry_offset: u32,
    pub flags: u32,
    pub reserved: u32,
}

/// A fixed-address RAM segment. Copy file_size bytes, then zero the remaining bytes.
#[repr(C)]
pub struct UserImageSegment {
    /// Source offset relative to the first image byte.
    pub image_offset: u32,
    /// Fixed RAM VMA used when the user ELF was linked.
    pub memory_address: u32,
    /// Bytes to copy from image_offset to memory_address.
    pub file_size: u32,
    /// Total RAM span; bytes after file_size must be zeroed.
    pub memory_size: u32,
    pub flags: u32,
}
