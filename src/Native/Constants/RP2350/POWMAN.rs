#![allow(dead_code)]
// POWMAN

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const POWMAN_BASE:                              u32 = 0x4010_0000;

pub const POWMAN_BADPASSWD:                         u32 = POWMAN_BASE + 0x0;
pub const POWMAN_VREG_CTRL:                         u32 = POWMAN_BASE + 0x4;
pub const POWMAN_VREG_STS:                          u32 = POWMAN_BASE + 0x8;
pub const POWMAN_VREG:                              u32 = POWMAN_BASE + 0xC;
pub const POWMAN_VREG_LP_ENTRY:                     u32 = POWMAN_BASE + 0x10;
pub const POWMAN_VREG_LP_EXIT:                      u32 = POWMAN_BASE + 0x14;
pub const POWMAN_BOD_CTRL:                          u32 = POWMAN_BASE + 0x18;
pub const POWMAN_BOD:                               u32 = POWMAN_BASE + 0x1C;
pub const POWMAN_BOD_LP_ENTRY:                      u32 = POWMAN_BASE + 0x20;
pub const POWMAN_BOD_LP_EXIT:                       u32 = POWMAN_BASE + 0x24;
pub const POWMAN_LPOSC:                             u32 = POWMAN_BASE + 0x28;
pub const POWMAN_CHIP_RESET:                        u32 = POWMAN_BASE + 0x2C;
pub const POWMAN_WDSEL:                             u32 = POWMAN_BASE + 0x30;
pub const POWMAN_SEQ_CFG:                           u32 = POWMAN_BASE + 0x34;
pub const POWMAN_STATE:                             u32 = POWMAN_BASE + 0x38;
pub const POWMAN_POW_FASTDIV:                       u32 = POWMAN_BASE + 0x3C;
pub const POWMAN_POW_DELAY:                         u32 = POWMAN_BASE + 0x40;
// EXT_CTRL0..EXT_CTRL1
pub fn POWMAN_EXT_CTRL(n: u32) -> u32 {
    return POWMAN_BASE + 0x44 + n * 0x4
}

pub const POWMAN_EXT_TIME_REF:                      u32 = POWMAN_BASE + 0x4C;
pub const POWMAN_LPOSC_FREQ_KHZ_INT:                u32 = POWMAN_BASE + 0x50;
pub const POWMAN_LPOSC_FREQ_KHZ_FRAC:               u32 = POWMAN_BASE + 0x54;
pub const POWMAN_XOSC_FREQ_KHZ_INT:                 u32 = POWMAN_BASE + 0x58;
pub const POWMAN_XOSC_FREQ_KHZ_FRAC:                u32 = POWMAN_BASE + 0x5C;
pub const POWMAN_SET_TIME_63TO48:                   u32 = POWMAN_BASE + 0x60;
pub const POWMAN_SET_TIME_47TO32:                   u32 = POWMAN_BASE + 0x64;
pub const POWMAN_SET_TIME_31TO16:                   u32 = POWMAN_BASE + 0x68;
pub const POWMAN_SET_TIME_15TO0:                    u32 = POWMAN_BASE + 0x6C;
pub const POWMAN_READ_TIME_UPPER:                   u32 = POWMAN_BASE + 0x70;
pub const POWMAN_READ_TIME_LOWER:                   u32 = POWMAN_BASE + 0x74;
pub const POWMAN_ALARM_TIME_63TO48:                 u32 = POWMAN_BASE + 0x78;
pub const POWMAN_ALARM_TIME_47TO32:                 u32 = POWMAN_BASE + 0x7C;
pub const POWMAN_ALARM_TIME_31TO16:                 u32 = POWMAN_BASE + 0x80;
pub const POWMAN_ALARM_TIME_15TO0:                  u32 = POWMAN_BASE + 0x84;
pub const POWMAN_TIMER:                             u32 = POWMAN_BASE + 0x88;
// PWRUP0..PWRUP3
pub fn POWMAN_PWRUP(n: u32) -> u32 {
    return POWMAN_BASE + 0x8C + n * 0x4
}

pub const POWMAN_CURRENT_PWRUP_REQ:                 u32 = POWMAN_BASE + 0x9C;
pub const POWMAN_LAST_SWCORE_PWRUP:                 u32 = POWMAN_BASE + 0xA0;
pub const POWMAN_DBG_PWRCFG:                        u32 = POWMAN_BASE + 0xA4;
pub const POWMAN_BOOTDIS:                           u32 = POWMAN_BASE + 0xA8;
pub const POWMAN_DBGCONFIG:                         u32 = POWMAN_BASE + 0xAC;
// SCRATCH0..SCRATCH7
pub fn POWMAN_SCRATCH(n: u32) -> u32 {
    return POWMAN_BASE + 0xB0 + n * 0x4
}

// BOOT0..BOOT3
pub fn POWMAN_BOOT(n: u32) -> u32 {
    return POWMAN_BASE + 0xD0 + n * 0x4
}

pub const POWMAN_INTR:                              u32 = POWMAN_BASE + 0xE0;
pub const POWMAN_INTE:                              u32 = POWMAN_BASE + 0xE4;
pub const POWMAN_INTF:                              u32 = POWMAN_BASE + 0xE8;
pub const POWMAN_INTS:                              u32 = POWMAN_BASE + 0xEC;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// BADPASSWD
pub const POWMAN_BADPASSWD_BIT:                     u32 = 0;

// VREG_CTRL
pub const POWMAN_VREG_CTRL_RST_N_BIT:               u32 = 15;
pub const POWMAN_VREG_CTRL_UNLOCK_BIT:              u32 = 13;
pub const POWMAN_VREG_CTRL_ISOLATE_BIT:             u32 = 12;
pub const POWMAN_VREG_CTRL_DISABLE_VOLTAGE_LIMIT_BIT:u32 = 8;
pub const POWMAN_VREG_CTRL_HT_TH_LOW:               u32 = 4;
pub const POWMAN_VREG_CTRL_HT_TH_HIGH:              u32 = 6;

// VREG_STS
pub const POWMAN_VREG_STS_VOUT_OK_BIT:              u32 = 4;
pub const POWMAN_VREG_STS_STARTUP_BIT:              u32 = 0;

// VREG
pub const POWMAN_VREG_UPDATE_IN_PROGRESS_BIT:       u32 = 15;
pub const POWMAN_VREG_VSEL_LOW:                     u32 = 4;
pub const POWMAN_VREG_VSEL_HIGH:                    u32 = 8;
pub const POWMAN_VREG_HIZ_BIT:                      u32 = 1;

// VREG_LP_ENTRY, VREG_LP_EXIT
pub const POWMAN_VREG_LP_VSEL_LOW:                  u32 = 4;
pub const POWMAN_VREG_LP_VSEL_HIGH:                 u32 = 8;
pub const POWMAN_VREG_LP_MODE_BIT:                  u32 = 2;
pub const POWMAN_VREG_LP_HIZ_BIT:                   u32 = 1;

// BOD_CTRL
pub const POWMAN_BOD_CTRL_ISOLATE_BIT:              u32 = 12;

// BOD, BOD_LP_ENTRY, BOD_LP_EXIT
pub const POWMAN_BOD_VSEL_LOW:                      u32 = 4;
pub const POWMAN_BOD_VSEL_HIGH:                     u32 = 8;
pub const POWMAN_BOD_EN_BIT:                        u32 = 0;

// LPOSC
pub const POWMAN_LPOSC_TRIM_LOW:                    u32 = 4;
pub const POWMAN_LPOSC_TRIM_HIGH:                   u32 = 9;
pub const POWMAN_LPOSC_MODE_LOW:                    u32 = 0;
pub const POWMAN_LPOSC_MODE_HIGH:                   u32 = 1;

// CHIP_RESET
pub const POWMAN_CHIP_RESET_HAD_WATCHDOG_RESET_RSM_BIT:u32 = 28;
pub const POWMAN_CHIP_RESET_HAD_HZD_SYS_RESET_REQ_BIT:u32 = 27;
pub const POWMAN_CHIP_RESET_HAD_GLITCH_DETECT_BIT:  u32 = 26;
pub const POWMAN_CHIP_RESET_HAD_SWCORE_PD_BIT:      u32 = 25;
pub const POWMAN_CHIP_RESET_HAD_WATCHDOG_RESET_SWCORE_BIT:u32 = 24;
pub const POWMAN_CHIP_RESET_HAD_WATCHDOG_RESET_POWMAN_BIT:u32 = 23;
pub const POWMAN_CHIP_RESET_HAD_WATCHDOG_RESET_POWMAN_ASYNC_BIT:u32 = 22;
pub const POWMAN_CHIP_RESET_HAD_RESCUE_BIT:         u32 = 21;
pub const POWMAN_CHIP_RESET_HAD_DP_RESET_REQ_BIT:   u32 = 19;
pub const POWMAN_CHIP_RESET_HAD_RUN_LOW_BIT:        u32 = 18;
pub const POWMAN_CHIP_RESET_HAD_BOR_BIT:            u32 = 17;
pub const POWMAN_CHIP_RESET_HAD_POR_BIT:            u32 = 16;
pub const POWMAN_CHIP_RESET_RESCUE_FLAG_BIT:        u32 = 4;
pub const POWMAN_CHIP_RESET_DOUBLE_TAP_BIT:         u32 = 0;

// WDSEL
pub const POWMAN_WDSEL_RESET_RSM_BIT:               u32 = 12;
pub const POWMAN_WDSEL_RESET_SWCORE_BIT:            u32 = 8;
pub const POWMAN_WDSEL_RESET_POWMAN_BIT:            u32 = 4;
pub const POWMAN_WDSEL_RESET_POWMAN_ASYNC_BIT:      u32 = 0;

// SEQ_CFG
pub const POWMAN_SEQ_CFG_USING_FAST_POWCK_BIT:      u32 = 20;
pub const POWMAN_SEQ_CFG_USING_BOD_LP_BIT:          u32 = 17;
pub const POWMAN_SEQ_CFG_USING_VREG_LP_BIT:         u32 = 16;
pub const POWMAN_SEQ_CFG_USE_FAST_POWCK_BIT:        u32 = 12;
pub const POWMAN_SEQ_CFG_RUN_LPOSC_IN_LP_BIT:       u32 = 8;
pub const POWMAN_SEQ_CFG_USE_BOD_HP_BIT:            u32 = 7;
pub const POWMAN_SEQ_CFG_USE_BOD_LP_BIT:            u32 = 6;
pub const POWMAN_SEQ_CFG_USE_VREG_HP_BIT:           u32 = 5;
pub const POWMAN_SEQ_CFG_USE_VREG_LP_BIT:           u32 = 4;
pub const POWMAN_SEQ_CFG_HW_PWRUP_SRAM0_BIT:        u32 = 1;
pub const POWMAN_SEQ_CFG_HW_PWRUP_SRAM1_BIT:        u32 = 0;

// STATE
pub const POWMAN_STATE_CHANGING_BIT:                u32 = 13;
pub const POWMAN_STATE_WAITING_BIT:                 u32 = 12;
pub const POWMAN_STATE_BAD_HW_REQ_BIT:              u32 = 11;
pub const POWMAN_STATE_BAD_SW_REQ_BIT:              u32 = 10;
pub const POWMAN_STATE_PWRUP_WHILE_WAITING_BIT:     u32 = 9;
pub const POWMAN_STATE_REQ_IGNORED_BIT:             u32 = 8;
pub const POWMAN_STATE_REQ_LOW:                     u32 = 4;
pub const POWMAN_STATE_REQ_HIGH:                    u32 = 7;
pub const POWMAN_STATE_CURRENT_LOW:                 u32 = 0;
pub const POWMAN_STATE_CURRENT_HIGH:                u32 = 3;

// POW_FASTDIV
pub const POWMAN_POW_FASTDIV_LOW:                   u32 = 0;
pub const POWMAN_POW_FASTDIV_HIGH:                  u32 = 10;

// POW_DELAY
pub const POWMAN_POW_DELAY_SRAM_STEP_LOW:           u32 = 8;
pub const POWMAN_POW_DELAY_SRAM_STEP_HIGH:          u32 = 15;
pub const POWMAN_POW_DELAY_XIP_STEP_LOW:            u32 = 4;
pub const POWMAN_POW_DELAY_XIP_STEP_HIGH:           u32 = 7;
pub const POWMAN_POW_DELAY_SWCORE_STEP_LOW:         u32 = 0;
pub const POWMAN_POW_DELAY_SWCORE_STEP_HIGH:        u32 = 3;

// EXT_CTRL0, EXT_CTRL1
pub const POWMAN_EXT_CTRL_LP_EXIT_STATE_BIT:        u32 = 14;
pub const POWMAN_EXT_CTRL_LP_ENTRY_STATE_BIT:       u32 = 13;
pub const POWMAN_EXT_CTRL_INIT_STATE_BIT:           u32 = 12;
pub const POWMAN_EXT_CTRL_INIT_BIT:                 u32 = 8;
pub const POWMAN_EXT_CTRL_GPIO_SELECT_LOW:          u32 = 0;
pub const POWMAN_EXT_CTRL_GPIO_SELECT_HIGH:         u32 = 5;

// EXT_TIME_REF
pub const POWMAN_EXT_TIME_REF_DRIVE_LPCK_BIT:       u32 = 4;
pub const POWMAN_EXT_TIME_REF_SOURCE_SEL_LOW:       u32 = 0;
pub const POWMAN_EXT_TIME_REF_SOURCE_SEL_HIGH:      u32 = 1;

// LPOSC_FREQ_KHZ_INT
pub const POWMAN_LPOSC_FREQ_KHZ_INT_LOW:            u32 = 0;
pub const POWMAN_LPOSC_FREQ_KHZ_INT_HIGH:           u32 = 5;

// LPOSC_FREQ_KHZ_FRAC
pub const POWMAN_LPOSC_FREQ_KHZ_FRAC_LOW:           u32 = 0;
pub const POWMAN_LPOSC_FREQ_KHZ_FRAC_HIGH:          u32 = 15;

// XOSC_FREQ_KHZ_INT
pub const POWMAN_XOSC_FREQ_KHZ_INT_LOW:             u32 = 0;
pub const POWMAN_XOSC_FREQ_KHZ_INT_HIGH:            u32 = 15;

// XOSC_FREQ_KHZ_FRAC
pub const POWMAN_XOSC_FREQ_KHZ_FRAC_LOW:            u32 = 0;
pub const POWMAN_XOSC_FREQ_KHZ_FRAC_HIGH:           u32 = 15;

// SET_TIME_63TO48, SET_TIME_47TO32, SET_TIME_31TO16, SET_TIME_15TO0
pub const POWMAN_SET_TIME_TO_LOW:                   u32 = 0;
pub const POWMAN_SET_TIME_TO_HIGH:                  u32 = 15;

// ALARM_TIME_63TO48, ALARM_TIME_47TO32, ALARM_TIME_31TO16, ALARM_TIME_15TO0
pub const POWMAN_ALARM_TIME_TO_LOW:                 u32 = 0;
pub const POWMAN_ALARM_TIME_TO_HIGH:                u32 = 15;

// TIMER
pub const POWMAN_TIMER_USING_GPIO_1HZ_BIT:          u32 = 19;
pub const POWMAN_TIMER_USING_GPIO_1KHZ_BIT:         u32 = 18;
pub const POWMAN_TIMER_USING_LPOSC_BIT:             u32 = 17;
pub const POWMAN_TIMER_USING_XOSC_BIT:              u32 = 16;
pub const POWMAN_TIMER_USE_GPIO_1HZ_BIT:            u32 = 13;
pub const POWMAN_TIMER_USE_GPIO_1KHZ_BIT:           u32 = 10;
pub const POWMAN_TIMER_USE_XOSC_BIT:                u32 = 9;
pub const POWMAN_TIMER_USE_LPOSC_BIT:               u32 = 8;
pub const POWMAN_TIMER_ALARM_BIT:                   u32 = 6;
pub const POWMAN_TIMER_PWRUP_ON_ALARM_BIT:          u32 = 5;
pub const POWMAN_TIMER_ALARM_ENAB_BIT:              u32 = 4;
pub const POWMAN_TIMER_CLEAR_BIT:                   u32 = 2;
pub const POWMAN_TIMER_RUN_BIT:                     u32 = 1;
pub const POWMAN_TIMER_NONSEC_WRITE_BIT:            u32 = 0;

// PWRUP0, PWRUP1, PWRUP2, PWRUP3
pub const POWMAN_PWRUP_RAW_STATUS_BIT:              u32 = 10;
pub const POWMAN_PWRUP_STATUS_BIT:                  u32 = 9;
pub const POWMAN_PWRUP_MODE_BIT:                    u32 = 8;
pub const POWMAN_PWRUP_DIRECTION_BIT:               u32 = 7;
pub const POWMAN_PWRUP_ENABLE_BIT:                  u32 = 6;
pub const POWMAN_PWRUP_SOURCE_LOW:                  u32 = 0;
pub const POWMAN_PWRUP_SOURCE_HIGH:                 u32 = 5;

// CURRENT_PWRUP_REQ
pub const POWMAN_CURRENT_PWRUP_REQ_LOW:             u32 = 0;
pub const POWMAN_CURRENT_PWRUP_REQ_HIGH:            u32 = 6;

// LAST_SWCORE_PWRUP
pub const POWMAN_LAST_SWCORE_PWRUP_LOW:             u32 = 0;
pub const POWMAN_LAST_SWCORE_PWRUP_HIGH:            u32 = 6;

// DBG_PWRCFG
pub const POWMAN_DBG_PWRCFG_IGNORE_BIT:             u32 = 0;

// BOOTDIS
pub const POWMAN_BOOTDIS_NEXT_BIT:                  u32 = 1;
pub const POWMAN_BOOTDIS_NOW_BIT:                   u32 = 0;

// DBGCONFIG
pub const POWMAN_DBGCONFIG_DP_INSTID_LOW:           u32 = 0;
pub const POWMAN_DBGCONFIG_DP_INSTID_HIGH:          u32 = 3;

// INTR
pub const POWMAN_INTR_PWRUP_WHILE_WAITING_BIT:      u32 = 3;
pub const POWMAN_INTR_STATE_REQ_IGNORED_BIT:        u32 = 2;
pub const POWMAN_INTR_TIMER_BIT:                    u32 = 1;
pub const POWMAN_INTR_VREG_OUTPUT_LOW_BIT:          u32 = 0;

// INTE
pub const POWMAN_INTE_PWRUP_WHILE_WAITING_BIT:      u32 = 3;
pub const POWMAN_INTE_STATE_REQ_IGNORED_BIT:        u32 = 2;
pub const POWMAN_INTE_TIMER_BIT:                    u32 = 1;
pub const POWMAN_INTE_VREG_OUTPUT_LOW_BIT:          u32 = 0;

// INTF
pub const POWMAN_INTF_PWRUP_WHILE_WAITING_BIT:      u32 = 3;
pub const POWMAN_INTF_STATE_REQ_IGNORED_BIT:        u32 = 2;
pub const POWMAN_INTF_TIMER_BIT:                    u32 = 1;
pub const POWMAN_INTF_VREG_OUTPUT_LOW_BIT:          u32 = 0;

// INTS
pub const POWMAN_INTS_PWRUP_WHILE_WAITING_BIT:      u32 = 3;
pub const POWMAN_INTS_STATE_REQ_IGNORED_BIT:        u32 = 2;
pub const POWMAN_INTS_TIMER_BIT:                    u32 = 1;
pub const POWMAN_INTS_VREG_OUTPUT_LOW_BIT:          u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// PWRUP0..PWRUP3: MODE
pub const POWMAN_PWRUP_MODE_LEVEL:                  u32 = 0x0;
pub const POWMAN_PWRUP_MODE_EDGE:                   u32 = 0x1;

// PWRUP0..PWRUP3: DIRECTION
pub const POWMAN_PWRUP_DIRECTION_LOW_FALLING:       u32 = 0x0;
pub const POWMAN_PWRUP_DIRECTION_HIGH_RISING:       u32 = 0x1;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
