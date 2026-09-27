#![allow(dead_code)]
// OTP_DATA

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const OTP_DATA_BASE:                            u32 = 0x4013_0000;

// CHIPID0..CHIPID3
pub fn OTP_DATA_CHIPID(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x0 + n * 0x2
}

// RANDID0..RANDID7
pub fn OTP_DATA_RANDID(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x8 + n * 0x2
}

pub const OTP_DATA_ROSC_CALIB:                      u32 = OTP_DATA_BASE + 0x20;
pub const OTP_DATA_LPOSC_CALIB:                     u32 = OTP_DATA_BASE + 0x22;
pub const OTP_DATA_NUM_GPIOS:                       u32 = OTP_DATA_BASE + 0x30;
// INFO_CRC0..INFO_CRC1
pub fn OTP_DATA_INFO_CRC(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x6C + n * 0x2
}

pub const OTP_DATA_FLASH_DEVINFO:                   u32 = OTP_DATA_BASE + 0xA8;
pub const OTP_DATA_FLASH_PARTITION_SLOT_SIZE:       u32 = OTP_DATA_BASE + 0xAA;
pub const OTP_DATA_BOOTSEL_LED_CFG:                 u32 = OTP_DATA_BASE + 0xAC;
pub const OTP_DATA_BOOTSEL_PLL_CFG:                 u32 = OTP_DATA_BASE + 0xAE;
pub const OTP_DATA_BOOTSEL_XOSC_CFG:                u32 = OTP_DATA_BASE + 0xB0;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR:            u32 = OTP_DATA_BASE + 0xB8;
pub const OTP_DATA_OTPBOOT_SRC:                     u32 = OTP_DATA_BASE + 0xBC;
pub const OTP_DATA_OTPBOOT_LEN:                     u32 = OTP_DATA_BASE + 0xBE;
// OTPBOOT_DST0..OTPBOOT_DST1
pub fn OTP_DATA_OTPBOOT_DST(n: u32) -> u32 {
    return OTP_DATA_BASE + 0xC0 + n * 0x2
}

// BOOTKEY0_0..BOOTKEY0_15
pub fn OTP_DATA_BOOTKEY0(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x100 + n * 0x2
}

// BOOTKEY1_0..BOOTKEY1_15
pub fn OTP_DATA_BOOTKEY1(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x120 + n * 0x2
}

// BOOTKEY2_0..BOOTKEY2_15
pub fn OTP_DATA_BOOTKEY2(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x140 + n * 0x2
}

// BOOTKEY3_0..BOOTKEY3_15
pub fn OTP_DATA_BOOTKEY3(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x160 + n * 0x2
}

// KEY1_0..KEY1_7
pub fn OTP_DATA_KEY1(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x1E90 + n * 0x2
}

// KEY2_0..KEY2_7
pub fn OTP_DATA_KEY2(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x1EA0 + n * 0x2
}

// KEY3_0..KEY3_7
pub fn OTP_DATA_KEY3(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x1EB0 + n * 0x2
}

// KEY4_0..KEY4_7
pub fn OTP_DATA_KEY4(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x1EC0 + n * 0x2
}

// KEY5_0..KEY5_7
pub fn OTP_DATA_KEY5(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x1ED0 + n * 0x2
}

// KEY6_0..KEY6_7
pub fn OTP_DATA_KEY6(n: u32) -> u32 {
    return OTP_DATA_BASE + 0x1EE0 + n * 0x2
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// CHIPID0, CHIPID1, CHIPID2, CHIPID3
pub const OTP_DATA_CHIPID_LOW:                      u32 = 0;
pub const OTP_DATA_CHIPID_HIGH:                     u32 = 15;

// RANDID0, RANDID1, RANDID2, RANDID3, RANDID4, RANDID5, RANDID6, RANDID7
pub const OTP_DATA_RANDID_LOW:                      u32 = 0;
pub const OTP_DATA_RANDID_HIGH:                     u32 = 15;

// ROSC_CALIB
pub const OTP_DATA_ROSC_CALIB_LOW:                  u32 = 0;
pub const OTP_DATA_ROSC_CALIB_HIGH:                 u32 = 15;

// LPOSC_CALIB
pub const OTP_DATA_LPOSC_CALIB_LOW:                 u32 = 0;
pub const OTP_DATA_LPOSC_CALIB_HIGH:                u32 = 15;

// NUM_GPIOS
pub const OTP_DATA_NUM_GPIOS_LOW:                   u32 = 0;
pub const OTP_DATA_NUM_GPIOS_HIGH:                  u32 = 7;

// INFO_CRC0, INFO_CRC1
pub const OTP_DATA_INFO_CRC_LOW:                    u32 = 0;
pub const OTP_DATA_INFO_CRC_HIGH:                   u32 = 15;

// FLASH_DEVINFO
pub const OTP_DATA_FLASH_DEVINFO_CS1_SIZE_LOW:      u32 = 12;
pub const OTP_DATA_FLASH_DEVINFO_CS1_SIZE_HIGH:     u32 = 15;
pub const OTP_DATA_FLASH_DEVINFO_CS0_SIZE_LOW:      u32 = 8;
pub const OTP_DATA_FLASH_DEVINFO_CS0_SIZE_HIGH:     u32 = 11;
pub const OTP_DATA_FLASH_DEVINFO_D8H_ERASE_SUPPORTED_BIT:u32 = 7;
pub const OTP_DATA_FLASH_DEVINFO_CS1_GPIO_LOW:      u32 = 0;
pub const OTP_DATA_FLASH_DEVINFO_CS1_GPIO_HIGH:     u32 = 5;

// FLASH_PARTITION_SLOT_SIZE
pub const OTP_DATA_FLASH_PARTITION_SLOT_SIZE_LOW:   u32 = 0;
pub const OTP_DATA_FLASH_PARTITION_SLOT_SIZE_HIGH:  u32 = 15;

// BOOTSEL_LED_CFG
pub const OTP_DATA_BOOTSEL_LED_CFG_ACTIVELOW_BIT:   u32 = 8;
pub const OTP_DATA_BOOTSEL_LED_CFG_PIN_LOW:         u32 = 0;
pub const OTP_DATA_BOOTSEL_LED_CFG_PIN_HIGH:        u32 = 5;

// BOOTSEL_PLL_CFG
pub const OTP_DATA_BOOTSEL_PLL_CFG_REFDIV_BIT:      u32 = 15;
pub const OTP_DATA_BOOTSEL_PLL_CFG_POSTDIV2_LOW:    u32 = 12;
pub const OTP_DATA_BOOTSEL_PLL_CFG_POSTDIV2_HIGH:   u32 = 14;
pub const OTP_DATA_BOOTSEL_PLL_CFG_POSTDIV1_LOW:    u32 = 9;
pub const OTP_DATA_BOOTSEL_PLL_CFG_POSTDIV1_HIGH:   u32 = 11;
pub const OTP_DATA_BOOTSEL_PLL_CFG_FBDIV_LOW:       u32 = 0;
pub const OTP_DATA_BOOTSEL_PLL_CFG_FBDIV_HIGH:      u32 = 8;

// BOOTSEL_XOSC_CFG
pub const OTP_DATA_BOOTSEL_XOSC_CFG_RANGE_LOW:      u32 = 14;
pub const OTP_DATA_BOOTSEL_XOSC_CFG_RANGE_HIGH:     u32 = 15;
pub const OTP_DATA_BOOTSEL_XOSC_CFG_STARTUP_LOW:    u32 = 0;
pub const OTP_DATA_BOOTSEL_XOSC_CFG_STARTUP_HIGH:   u32 = 13;

// USB_WHITE_LABEL_ADDR
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_LOW:        u32 = 0;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_HIGH:       u32 = 15;

// OTPBOOT_SRC
pub const OTP_DATA_OTPBOOT_SRC_LOW:                 u32 = 0;
pub const OTP_DATA_OTPBOOT_SRC_HIGH:                u32 = 15;

// OTPBOOT_LEN
pub const OTP_DATA_OTPBOOT_LEN_LOW:                 u32 = 0;
pub const OTP_DATA_OTPBOOT_LEN_HIGH:                u32 = 15;

// OTPBOOT_DST0, OTPBOOT_DST1
pub const OTP_DATA_OTPBOOT_DST_LOW:                 u32 = 0;
pub const OTP_DATA_OTPBOOT_DST_HIGH:                u32 = 15;

// BOOTKEY0_0..BOOTKEY3_15
pub const OTP_DATA_BOOTKEY_LOW:                     u32 = 0;
pub const OTP_DATA_BOOTKEY_HIGH:                    u32 = 15;

// KEY1_0..KEY6_7
pub const OTP_DATA_KEY_LOW:                         u32 = 0;
pub const OTP_DATA_KEY_HIGH:                        u32 = 15;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// FLASH_DEVINFO..FLASH_DEVINFO: CS_SIZE
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_NONE:      u32 = 0x0;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_8K:        u32 = 0x1;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_16K:       u32 = 0x2;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_32K:       u32 = 0x3;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_64K:       u32 = 0x4;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_128K:      u32 = 0x5;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_256K:      u32 = 0x6;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_512K:      u32 = 0x7;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_1M:        u32 = 0x8;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_2M:        u32 = 0x9;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_4M:        u32 = 0xA;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_8M:        u32 = 0xB;
pub const OTP_DATA_FLASH_DEVINFO_CS_SIZE_16M:       u32 = 0xC;

// BOOTSEL_XOSC_CFG: RANGE
pub const OTP_DATA_BOOTSEL_XOSC_CFG_RANGE_1_15MHZ:  u32 = 0x0;
pub const OTP_DATA_BOOTSEL_XOSC_CFG_RANGE_10_30MHZ: u32 = 0x1;
pub const OTP_DATA_BOOTSEL_XOSC_CFG_RANGE_25_60MHZ: u32 = 0x2;
pub const OTP_DATA_BOOTSEL_XOSC_CFG_RANGE_40_100MHZ:u32 = 0x3;

// USB_WHITE_LABEL_ADDR: USB_WHITE_LABEL_ADDR
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_VID_VALUE:u32 = 0x0;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_PID_VALUE:u32 = 0x1;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_BCD_DEVICE_VALUE:u32 = 0x2;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_LANG_ID_VALUE:u32 = 0x3;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_MANUFACTURER_STRDEF:u32 = 0x4;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_PRODUCT_STRDEF:u32 = 0x5;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_SERIAL_NUMBER_STRDEF:u32 = 0x6;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES:u32 = 0x7;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_VOLUME_LABEL_STRDEF:u32 = 0x8;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_SCSI_INQUIRY_VENDOR_STRDEF:u32 = 0x9;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_SCSI_INQUIRY_PRODUCT_STRDEF:u32 = 0xA;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_SCSI_INQUIRY_VERSION_STRDEF:u32 = 0xB;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_INDEX_HTM_REDIRECT_URL_STRDEF:u32 = 0xC;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_INDEX_HTM_REDIRECT_NAME_STRDEF:u32 = 0xD;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_INFO_UF2_TXT_MODEL_STRDEF:u32 = 0xE;
pub const OTP_DATA_USB_WHITE_LABEL_ADDR_INDEX_INFO_UF2_TXT_BOARD_ID_STRDEF:u32 = 0xF;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
