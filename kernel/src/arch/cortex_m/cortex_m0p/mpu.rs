use crate::arch::general::asm;
use crate::native::constants::current::ppb::*;
use crate::util::register::Register;

pub const MAX_MPU_REGIONS: usize = 8;

const MPU_RASR_XN_BIT: u32 = 28;
const MPU_RASR_AP_HIGH: u32 = 26;
const MPU_RASR_AP_LOW: u32 = 24;
const MPU_RASR_SCB_HIGH: u32 = 18;
const MPU_RASR_SCB_LOW: u32 = 16;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AccessPermission {
    NoAccess = 0x0,                        // 0b000
    PrivilegedReadWrite = 0x1,             // 0b001
    PrivilegedReadWriteUserReadOnly = 0x2, // 0b010
    ReadWrite = 0x3,                       // 0b011

    PrivilegedReadOnly = 0x5, // 0b101
    ReadOnly = 0x6,           // 0b110
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemoryAttributes {
    StronglyOrdered = 0x0,
    Device = 0x1,

    NonShareableWriteThrough = 0x2,
    ShareableWriteThrough = 0x6,

    NonShareableWriteBack = 0x3,
    ShareableWriteBack = 0x7,
}

#[derive(Clone, Copy)]
pub struct RegionConfig {
    pub base_address: u32,
    pub size: usize,
    pub permission: AccessPermission,
    pub attributes: MemoryAttributes,
    pub execute_never: bool, // the XN bit - maybe using an `executable` field is more natural-looking

    pub subregion_enable: u8, // to avoid using awkward "disable-mask" in kernel code
}

impl RegionConfig {
    pub fn new(
        base_address: u32,
        size: usize,
        permission: AccessPermission,
        attributes: MemoryAttributes,
        execute_never: bool,
    ) -> Self {
        // though the mpu of cortex-m0+ supports up regions to 4GB,
        // we still use `usize` type to contain size value
        // (note that using a single `usize` can simplify the strcture of mpu management)
        // that's to said, a 4GB mpu region is unsupported in our kernel
        // so `size > 0x100000000` check is not needed
        if size < 0x100 {
            panic!("Incorrect MPU region size: {}", size);
        }
        assert!(size.is_power_of_two(), "Unaligned region size");

        assert_eq!(
            base_address & ((size - 1) as u32),
            0,
            "MPU region base address is not size-aligned"
        );
        return Self {
            base_address,
            size,
            permission,
            attributes,
            execute_never,

            subregion_enable: 0xFF, // u8 size
        };
    }
}

pub struct MpuConfig {
    regions: [Option<RegionConfig>; MAX_MPU_REGIONS],
}

impl MpuConfig {
    pub const fn new() -> Self {
        Self {
            regions: [None; MAX_MPU_REGIONS],
        }
    }

    pub const fn from_regions(regions: [Option<RegionConfig>; MAX_MPU_REGIONS]) -> Self {
        Self { regions }
    }

    pub fn apply(&self) {
        let rnr = Register::new(PPB_MPU_RNR);
        let rbar = Register::new(PPB_MPU_RBAR);
        let rasr = Register::new(PPB_MPU_RASR);

        for (index, slot) in self.regions.iter().enumerate() {
            rnr.field_set(
                PPB_MPU_RNR_REGION_HIGH,
                PPB_MPU_RNR_REGION_LOW,
                index as u32,
            );
            rasr.write(0);
            if let Some(region) = slot {
                rbar.field_set(
                    PPB_MPU_RBAR_ADDR_HIGH,
                    PPB_MPU_RBAR_ADDR_LOW,
                    region.base_address >> PPB_MPU_RBAR_ADDR_LOW,
                );
                rasr.bit_set(MPU_RASR_XN_BIT, region.execute_never);
                rasr.field_set(MPU_RASR_AP_HIGH, MPU_RASR_AP_LOW, region.permission as u32);
                rasr.field_set(
                    MPU_RASR_SCB_HIGH,
                    MPU_RASR_SCB_LOW,
                    region.attributes as u32,
                );

                rasr.field_set(
                    PPB_MPU_RASR_SRD_HIGH,
                    PPB_MPU_RASR_SRD_LOW,
                    (!region.subregion_enable) as u32,
                );
                rasr.field_set(
                    PPB_MPU_RASR_SIZE_HIGH,
                    PPB_MPU_RASR_SIZE_LOW,
                    (region.size.ilog2() - 1) as u32,
                );

                rasr.bit_set(PPB_MPU_RASR_ENABLE_BIT, true);
            }
        }
    }
}

pub unsafe fn init_mpu() {
    asm::with_interrupts_disabled(|| {
        let ctrl = Register::new(PPB_MPU_CTRL);
        unsafe {
            core::arch::asm!("dsb", options(nostack, preserves_flags),);
        }

        // Disable all bits at once
        ctrl.write(0);

        // Clear all mpu region configs
        MpuConfig::new().apply();

        ctrl.bit_set(PPB_MPU_CTRL_PRIVDEFENA_BIT, true);
        ctrl.bit_set(PPB_MPU_CTRL_ENABLE_BIT, true);

        unsafe {
            core::arch::asm!("dsb", "isb", options(nostack, preserves_flags),);
        }
    });
}
