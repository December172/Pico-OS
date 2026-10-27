use core::cell::UnsafeCell;
use core::mem::offset_of;

use pico_abi::image::*;

use crate::util::crc32;
use crate::util::low_level::read_from_bytes;

const MAX_USER_IMAGES: usize = 4;
const MAX_USER_TASKS: usize = 2;

unsafe extern "C" {
    static __user_image_start: u8;
    static __user_image_end: u8;

    static mut __user_ram_start: u8;
    static mut __user_ram_end: u8;
}

#[derive(PartialEq, Eq)]
enum ImageError {
    CRC32,
    General,
}

pub struct ImageRegistry {
    pub images: [Option<&'static [u8]>; MAX_USER_IMAGES],
    pub entries: [[Option<UserTaskEntry>; MAX_USER_TASKS]; MAX_USER_IMAGES],
    count: usize,
    sealed: bool,
}

impl ImageRegistry {
    pub const fn new() -> Self {
        Self {
            images: [None; MAX_USER_IMAGES],
            entries: [[None; MAX_USER_TASKS]; MAX_USER_IMAGES],
            count: 0,
            sealed: false,
        }
    }

    /// Iterate over registered images and their entry slices without exposing array lengths.
    pub fn iter_images(&self) -> impl Iterator<Item = (&'static [u8], &[Option<UserTaskEntry>])> {
        return self
            .images
            .iter()
            .zip(&self.entries)
            .filter_map(|(image, entries)| image.map(|image| (image, entries.as_slice())));
    }

    pub fn seal(&mut self) {
        self.sealed = true;
    }

    pub fn register(&mut self, image: &'static [u8]) {
        if self.sealed {
            return;
        }

        if self.count >= MAX_USER_IMAGES {
            return;
        }

        let header = read_from_bytes::<UserImageHeader>(image).unwrap();

        let entry_table_start = u32::from_le(header.entry_table_offset) as usize;
        let entry_count = u32::from_le(header.entry_count) as usize;
        let entry_table_end = entry_table_start + entry_count * size_of::<UserTaskEntry>();

        let entry_table = image.get(entry_table_start..entry_table_end).unwrap();

        for (index, raw_entry) in entry_table
            .chunks_exact(size_of::<UserTaskEntry>())
            .take(MAX_USER_TASKS)
            .enumerate()
        {
            let Some(entry) = read_from_bytes::<UserTaskEntry>(raw_entry) else {
                continue;
            };

            if !verify_task_entry(image, &entry) {
                continue;
            }

            self.entries[self.count][index] = Some(entry);
        }

        self.images[self.count] = Some(image);

        self.count += 1;
    }
}

fn verify_user_image(image: &[u8], registry: &ImageRegistry) -> Result<(), ImageError> {
    let header = read_from_bytes::<UserImageHeader>(image).unwrap();
    // verify crc32 checksum
    {
        const CRC32_START: usize = offset_of!(UserImageHeader, crc32);

        let mut state: u32 = 0;

        state = crc32::update(state, &image[..CRC32_START]);
        state = crc32::update(state, &[0; size_of::<u32>()]);
        state = crc32::update(state, &image[CRC32_START + size_of::<u32>()..]);

        if state != header.crc32 {
            return Err(ImageError::CRC32);
        }
    }

    if (image.as_ptr() as usize) & 0x1 != 0 {
        // unaligned image
        return Err(ImageError::General);
    }

    // verify entry table
    let entry_table_start = u32::from_le(header.entry_table_offset) as usize;
    let entry_count = u32::from_le(header.entry_count) as usize;

    if entry_table_start < size_of::<UserImageHeader>() || entry_table_start & 0x1 != 0 {
        return Err(ImageError::General);
    }

    if entry_count == 0 {
        // empty entry table
        return Err(ImageError::General);
    }

    let Some(entry_table_end) = entry_count
        .checked_mul(size_of::<UserTaskEntry>())
        .and_then(|size| entry_table_start.checked_add(size))
    else {
        // overflow
        return Err(ImageError::General);
    };

    if image.get(entry_table_start..entry_table_end).is_none() {
        return Err(ImageError::General);
    }

    // verify segments
    {
        let segment_offset = u32::from_le(header.segment_offset) as usize;
        let Some(segment_end) = segment_offset.checked_add(size_of::<UserImageSegment>()) else {
            return Err(ImageError::General);
        };

        if segment_offset < size_of::<UserImageHeader>()
            || (entry_table_start < segment_end && segment_offset < entry_table_end)
        {
            return Err(ImageError::General);
        }

        let Some(segment) = image
            .get(segment_offset..segment_end)
            .and_then(read_from_bytes::<UserImageSegment>)
        else {
            return Err(ImageError::General);
        };

        let src_start = u32::from_le(segment.image_offset) as usize;
        let dst_start = u32::from_le(segment.memory_address) as usize;
        let file_size = u32::from_le(segment.file_size) as usize;
        let memory_size = u32::from_le(segment.memory_size) as usize;

        let Some(src_end) = src_start.checked_add(file_size) else {
            return Err(ImageError::General);
        };

        let Some(dst_end) = dst_start.checked_add(memory_size) else {
            return Err(ImageError::General);
        };

        let user_ram_start = unsafe { core::ptr::addr_of_mut!(__user_ram_start) } as usize;
        let user_ram_end = unsafe { core::ptr::addr_of_mut!(__user_ram_end) } as usize;

        if dst_start < user_ram_start || dst_end > user_ram_end || file_size > memory_size {
            // out of range
            return Err(ImageError::General);
        }

        if image.get(src_start..src_end).is_none() {
            return Err(ImageError::General);
        }

        if memory_size != 0 {
            // Registered images have already passed the same range checks.
            for registered_image in registry.images[..registry.count].iter().flatten() {
                let registered_header =
                    read_from_bytes::<UserImageHeader>(registered_image).unwrap();
                let registered_segment = registered_image
                    .get(u32::from_le(registered_header.segment_offset) as usize..)
                    .and_then(read_from_bytes::<UserImageSegment>)
                    .unwrap();

                let registered_size = u32::from_le(registered_segment.memory_size) as usize;
                if registered_size == 0 {
                    continue;
                }

                let registered_start = u32::from_le(registered_segment.memory_address) as usize;
                let registered_end = registered_start + registered_size;
                // p: dst_end <= registered_start || registered_end <= dst_start
                // not p: dst_start < registered_end && registered_start < dst_end
                if dst_start < registered_end && registered_start < dst_end {
                    return Err(ImageError::General);
                }
            }
        }
    }

    return Ok(());
}

fn verify_task_entry(image: &[u8], entry: &UserTaskEntry) -> bool {
    let task_entry_offset = u32::from_le(entry.entry_offset) as usize;
    if task_entry_offset < size_of::<UserImageHeader>() {
        return false;
    }

    let Some(code) = image.get(task_entry_offset..) else {
        return false;
    };

    if code.len() < size_of::<u16>() {
        return false;
    }

    if task_entry_offset & 1 != 0 {
        return false;
    }

    return true;
}

fn collect_user_images() -> ImageRegistry {
    // Only scan this firmware's packed images; unused Flash may retain older images.
    let user_image_start = unsafe { core::ptr::addr_of!(__user_image_start) };
    let user_image_end = unsafe { core::ptr::addr_of!(__user_image_end) };

    let total_user_image_size = user_image_end as usize - user_image_start as usize;
    let flash: &[u8] =
        unsafe { core::slice::from_raw_parts(user_image_start, total_user_image_size) };

    let mut registry = ImageRegistry::new();

    let mut offset = 0;

    while flash.len() - offset >= size_of::<UserImageHeader>() {
        let start = offset;
        let candidate = &flash[start..];

        offset += 1;

        if !candidate.starts_with(&IMAGE_MAGIC) {
            continue;
        }

        let Some(header) = read_from_bytes::<UserImageHeader>(candidate) else {
            // invaild image header
            continue;
        };

        if u16::from_le(header.format_version) != IMAGE_FORMAT_VERSION
            || u16::from_le(header.abi_version) != KERNEL_ABI_VERSION
        {
            continue;
        }

        let image_size = u32::from_le(header.image_size) as usize;

        if image_size < size_of::<UserImageHeader>() || image_size > candidate.len() {
            continue;
        }

        let image = &candidate[..image_size];

        match verify_user_image(image, &registry) {
            Ok(()) => registry.register(image),
            Err(ImageError::CRC32) => {
                continue;
            }
            Err(ImageError::General) => {
                offset = start + image_size;
                continue;
            }
        }

        offset = start + image_size;
    }

    return registry;
}

pub struct ImageRegistryWrapper {
    registry: UnsafeCell<ImageRegistry>,
}

unsafe impl Sync for ImageRegistryWrapper {}

impl ImageRegistryWrapper {
    pub const fn new() -> Self {
        Self {
            registry: UnsafeCell::new(ImageRegistry::new()),
        }
    }

    pub fn init(&self) {
        let registry = unsafe { &mut *self.registry.get() };

        if registry.sealed {
            return;
        }

        *registry = collect_user_images();

        let user_ram_start = unsafe { core::ptr::addr_of_mut!(__user_ram_start) };
        let user_ram_end = unsafe { core::ptr::addr_of_mut!(__user_ram_end) };

        let user_ram_length = user_ram_end as usize - user_ram_start as usize;

        unsafe {
            core::ptr::write_bytes(user_ram_start, 0, user_ram_length);
        }

        for image in registry.images.iter().flatten() {
            let header = read_from_bytes::<UserImageHeader>(image).unwrap();

            let segment = image
                .get(header.segment_offset as usize..)
                .and_then(read_from_bytes::<UserImageSegment>)
                .unwrap();

            let src_start = unsafe { image.as_ptr().add(segment.image_offset as usize) };
            let dst_start = segment.memory_address as *mut u8;
            let file_size = segment.file_size as usize;

            unsafe {
                core::ptr::copy_nonoverlapping(src_start, dst_start, file_size);
            }
        }

        registry.seal();
    }

    pub fn with<Output>(&self, method: impl FnOnce(&ImageRegistry) -> Output) -> Output {
        let registry = unsafe { &*self.registry.get() };
        assert!(registry.sealed, "Image registry is not initialized");
        return method(&registry);
    }
}

pub static IMAGE_REGISTRY: ImageRegistryWrapper = ImageRegistryWrapper::new();
