use crate::arch::current::mpu::*;

#[derive(Clone, Copy)]
pub struct MemoryRange {
    pub base: u32,
    pub size: usize,
}

#[derive(Clone, Copy)]
pub struct MemoryMap {
    pub flash: MemoryRange,
    pub ram: MemoryRange,
}

pub struct MemoryConfig {
    mpu_config: MpuConfig,
}

impl MemoryConfig {
    pub const fn kernel() -> Self {
        Self {
            mpu_config: MpuConfig::new(),
        }
    }

    pub fn user(map: MemoryMap, stack: MemoryRange) -> Self {
        let mappings = [
            (map.flash, AccessPermission::ReadOnly, false),
            (map.ram, AccessPermission::ReadWrite, true),
            (stack, AccessPermission::ReadWrite, true),
        ];

        let mut regions: [Option<RegionConfig>; MAX_MPU_REGIONS] = [None; MAX_MPU_REGIONS];

        for (index, (region, permission, execute_never)) in mappings.into_iter().enumerate() {
            regions[index] = Some(RegionConfig::new(
                region.base,
                region.size,
                permission,
                MemoryAttributes::NonShareableWriteThrough,
                execute_never,
            ));
        }

        return Self {
            mpu_config: MpuConfig::from_regions(regions),
        };
    }

    pub fn apply(&self) {
        self.mpu_config.apply();
    }
}

include!(concat!(env!("OUT_DIR"), "/image_layouts.rs"));
