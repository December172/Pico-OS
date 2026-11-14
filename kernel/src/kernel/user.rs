use core::cell::UnsafeCell;
use core::mem::offset_of;

use pico_abi::image::*;

use crate::kernel::memory::layout::*;
use crate::util::crc32;
use crate::util::low_level::read_from_bytes;

const MAX_USER_IMAGES: usize = 3;
const MAX_USER_TASKS: usize = 2;

unsafe extern "C" {
    static __user_image_start: u8;
    static __user_image_end: u8;

    static mut __user_ram_start: u8;
    static mut __user_ram_end: u8;
}

#[derive(Clone, Copy)]
pub struct Image {
    pub data: &'static [u8],
    pub header: UserImageHeader,
    pub segment: UserImageSegment,
    pub mem_map: MemoryMap,
    pub entries: [Option<UserTaskEntry>; MAX_USER_TASKS],
}

pub struct ImageRegistry {
    pub images: [Option<Image>; MAX_USER_IMAGES],
    count: usize,
    sealed: bool,
}

impl Image {
    pub(crate) fn contains_entry(&self, entry: u32) -> bool {
        let Some(offset) = (entry & !1).checked_sub(self.data.as_ptr() as u32) else {
            return false;
        };

        self.contains_entry_offset(offset)
    }

    fn contains_entry_offset(&self, offset: u32) -> bool {
        (offset as usize) >= size_of::<UserImageHeader>()
            && offset & 1 == 0
            && self
                .data
                .get(offset as usize..)
                .is_some_and(|code| code.len() >= 2)
    }
}

impl ImageRegistry {
    pub const fn new() -> Self {
        Self {
            images: [None; MAX_USER_IMAGES],
            count: 0,
            sealed: false,
        }
    }

    /// Iterate over registered images and their entry slices without exposing array lengths.
    pub fn iter_images(&self) -> impl Iterator<Item = (usize, &Image)> + '_ {
        return self.images.iter().enumerate().filter_map(|(index, slot)| {
            return slot.as_ref().map(|image| (index, image));
        });
    }

    fn seal(&mut self) {
        self.sealed = true;
    }

    fn register(&mut self, image: Image) {
        if self.sealed {
            return;
        }

        if self.count >= MAX_USER_IMAGES {
            return;
        }
        self.images[self.count] = Some(image);

        self.count += 1;
    }
}

fn verify_user_image(image: &Image) -> bool {
    let header = image.header;
    // verify crc32 checksum
    {
        const CRC32_START: usize = offset_of!(UserImageHeader, crc32);

        let mut state: u32 = 0;

        state = crc32::update(state, &image.data[..CRC32_START]);
        state = crc32::update(state, &[0; size_of::<u32>()]);
        state = crc32::update(state, &image.data[CRC32_START + size_of::<u32>()..]);

        if state != header.crc32 {
            return false;
        }
    }

    if (image.data.as_ptr() as usize) & 0x1 != 0 {
        // unaligned image
        return false;
    }

    // verify entry table
    let entry_table_start = u32::from_le(header.entry_table_offset) as usize;
    let entry_count = u32::from_le(header.entry_count) as usize;

    if entry_table_start < size_of::<UserImageHeader>() || entry_table_start & 0x1 != 0 {
        return false;
    }

    if entry_count == 0 {
        // empty entry table
        return false;
    }

    let Some(entry_table_end) = entry_count
        .checked_mul(size_of::<UserTaskEntry>())
        .and_then(|size| entry_table_start.checked_add(size))
    else {
        // overflow
        return false;
    };

    if image.data.get(entry_table_start..entry_table_end).is_none() {
        return false;
    }

    // verify segments
    {
        let segment_offset = u32::from_le(header.segment_offset) as usize;
        let Some(segment_end) = segment_offset.checked_add(size_of::<UserImageSegment>()) else {
            return false;
        };

        if segment_offset < size_of::<UserImageHeader>()
            || (entry_table_start < segment_end && segment_offset < entry_table_end)
        {
            return false;
        }

        let Some(segment) = image
            .data
            .get(segment_offset..segment_end)
            .and_then(read_from_bytes::<UserImageSegment>)
        else {
            return false;
        };

        let src_start = u32::from_le(segment.image_offset) as usize;
        let dst_start = u32::from_le(segment.memory_address);
        let file_size = u32::from_le(segment.file_size) as usize;
        let memory_size = u32::from_le(segment.memory_size) as usize;

        let Some(src_end) = src_start.checked_add(file_size) else {
            return false;
        };

        let Some(dst_end) = dst_start.checked_add(memory_size as u32) else {
            return false;
        };

        let mem_map = image.mem_map;

        let user_ram_start = mem_map.ram.base;
        let user_ram_end = mem_map.ram.base + mem_map.ram.size as u32;

        if dst_start < user_ram_start || dst_end > user_ram_end || file_size > memory_size {
            // out of range
            return false;
        }

        if image.data.get(src_start..src_end).is_none() {
            return false;
        }
    }

    true
}

fn load_image(candidate: &'static [u8], map: MemoryMap) -> Option<Image> {
    let header = read_from_bytes::<UserImageHeader>(candidate)?;

    if header.magic != IMAGE_MAGIC {
        return None;
    }

    if u16::from_le(header.format_version) != IMAGE_FORMAT_VERSION
        || u16::from_le(header.abi_version) != KERNEL_ABI_VERSION
    {
        return None;
    }

    let image_size = u32::from_le(header.image_size) as usize;

    let data = candidate.get(..image_size)?;

    let segment = data
        .get(header.segment_offset as usize..)
        .and_then(read_from_bytes::<UserImageSegment>)?;

    let mut image = Image {
        data,
        header,
        segment,
        mem_map: map,
        entries: [None; MAX_USER_TASKS],
    };

    if !verify_user_image(&image) {
        return None;
    }

    let start = u32::from_le(header.entry_table_offset) as usize;
    let count = u32::from_le(header.entry_count) as usize;
    let end = start.checked_add(count.checked_mul(size_of::<UserTaskEntry>())?)?;
    let entry_table = image.data.get(start..end)?;

    for (index, raw_entry) in entry_table
        .chunks_exact(size_of::<UserTaskEntry>())
        .take(MAX_USER_TASKS)
        .enumerate()
    {
        let Some(entry) = read_from_bytes::<UserTaskEntry>(raw_entry) else {
            continue;
        };

        if image.contains_entry_offset(entry.entry_offset) {
            image.entries[index] = Some(entry);
        }
    }
    Some(image)
}

fn collect_user_images() -> ImageRegistry {
    // Only scan this firmware's packed images; unused Flash may retain older images.
    let user_image_start = core::ptr::addr_of!(__user_image_start);
    let user_image_end = core::ptr::addr_of!(__user_image_end);

    let total_user_image_size = user_image_end as usize - user_image_start as usize;
    let flash: &[u8] =
        unsafe { core::slice::from_raw_parts(user_image_start, total_user_image_size) };

    let mut registry = ImageRegistry::new();

    for map in IMAGE_MAPS {
        let offset = map.flash.base as usize - user_image_start as usize;

        let Some(remaining) = flash.get(offset..) else {
            continue;
        };

        let candidate_size = map.flash.size.min(remaining.len());
        let candidate = &remaining[..candidate_size];

        if let Some(image) = load_image(candidate, *map) {
            registry.register(image);
        }
    }
    registry
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

        let user_ram_start = core::ptr::addr_of_mut!(__user_ram_start);
        let user_ram_end = core::ptr::addr_of_mut!(__user_ram_end);

        let user_ram_length = user_ram_end as usize - user_ram_start as usize;

        unsafe {
            core::ptr::write_bytes(user_ram_start, 0, user_ram_length);
        }

        for image in registry.images.iter().flatten() {
            let segment = &image.segment;
            let src_start = unsafe { image.data.as_ptr().add(segment.image_offset as usize) };
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
        method(&registry)
    }
}

pub static IMAGE_REGISTRY: ImageRegistryWrapper = ImageRegistryWrapper::new();
