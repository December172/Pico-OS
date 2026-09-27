#![allow(dead_code)]
// OTP

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const OTP_BASE:                                 u32 = 0x4012_0000;

// SW_LOCK0..SW_LOCK63
pub fn OTP_SW_LOCK(n: u32) -> u32 {
    return OTP_BASE + 0x0 + n * 0x4
}

pub const OTP_SBPI_INSTR:                           u32 = OTP_BASE + 0x100;
// SBPI_WDATA_0..SBPI_WDATA_3
pub fn OTP_SBPI_WDATA(n: u32) -> u32 {
    return OTP_BASE + 0x104 + n * 0x4
}

// SBPI_RDATA_0..SBPI_RDATA_3
pub fn OTP_SBPI_RDATA(n: u32) -> u32 {
    return OTP_BASE + 0x114 + n * 0x4
}

pub const OTP_SBPI_STATUS:                          u32 = OTP_BASE + 0x124;
pub const OTP_USR:                                  u32 = OTP_BASE + 0x128;
pub const OTP_DBG:                                  u32 = OTP_BASE + 0x12C;
pub const OTP_BIST:                                 u32 = OTP_BASE + 0x134;
// CRT_KEY_W0..CRT_KEY_W3
pub fn OTP_CRT_KEY_W(n: u32) -> u32 {
    return OTP_BASE + 0x138 + n * 0x4
}

pub const OTP_CRITICAL:                             u32 = OTP_BASE + 0x148;
pub const OTP_KEY_VALID:                            u32 = OTP_BASE + 0x14C;
pub const OTP_DEBUGEN:                              u32 = OTP_BASE + 0x150;
pub const OTP_DEBUGEN_LOCK:                         u32 = OTP_BASE + 0x154;
pub const OTP_ARCHSEL:                              u32 = OTP_BASE + 0x158;
pub const OTP_ARCHSEL_STATUS:                       u32 = OTP_BASE + 0x15C;
pub const OTP_BOOTDIS:                              u32 = OTP_BASE + 0x160;
pub const OTP_INTR:                                 u32 = OTP_BASE + 0x164;
pub const OTP_INTE:                                 u32 = OTP_BASE + 0x168;
pub const OTP_INTF:                                 u32 = OTP_BASE + 0x16C;
pub const OTP_INTS:                                 u32 = OTP_BASE + 0x170;
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// SW_LOCK0..SW_LOCK63
pub const OTP_SW_LOCK_NSEC_LOW:                     u32 = 2;
pub const OTP_SW_LOCK_NSEC_HIGH:                    u32 = 3;
pub const OTP_SW_LOCK_SEC_LOW:                      u32 = 0;
pub const OTP_SW_LOCK_SEC_HIGH:                     u32 = 1;

// SBPI_INSTR
pub const OTP_SBPI_INSTR_EXEC_BIT:                  u32 = 30;
pub const OTP_SBPI_INSTR_IS_WR_BIT:                 u32 = 29;
pub const OTP_SBPI_INSTR_HAS_PAYLOAD_BIT:           u32 = 28;
pub const OTP_SBPI_INSTR_PAYLOAD_SIZE_M1_LOW:       u32 = 24;
pub const OTP_SBPI_INSTR_PAYLOAD_SIZE_M1_HIGH:      u32 = 27;
pub const OTP_SBPI_INSTR_TARGET_LOW:                u32 = 16;
pub const OTP_SBPI_INSTR_TARGET_HIGH:               u32 = 23;
pub const OTP_SBPI_INSTR_CMD_LOW:                   u32 = 8;
pub const OTP_SBPI_INSTR_CMD_HIGH:                  u32 = 15;
pub const OTP_SBPI_INSTR_SHORT_WDATA_LOW:           u32 = 0;
pub const OTP_SBPI_INSTR_SHORT_WDATA_HIGH:          u32 = 7;

// SBPI_STATUS
pub const OTP_SBPI_STATUS_MISO_LOW:                 u32 = 16;
pub const OTP_SBPI_STATUS_MISO_HIGH:                u32 = 23;
pub const OTP_SBPI_STATUS_FLAG_BIT:                 u32 = 12;
pub const OTP_SBPI_STATUS_INSTR_MISS_BIT:           u32 = 8;
pub const OTP_SBPI_STATUS_INSTR_DONE_BIT:           u32 = 4;
pub const OTP_SBPI_STATUS_RDATA_VLD_BIT:            u32 = 0;

// USR
pub const OTP_USR_PD_BIT:                           u32 = 4;
pub const OTP_USR_DCTRL_BIT:                        u32 = 0;

// DBG
pub const OTP_DBG_CUSTOMER_RMA_FLAG_BIT:            u32 = 12;
pub const OTP_DBG_PSM_STATE_LOW:                    u32 = 4;
pub const OTP_DBG_PSM_STATE_HIGH:                   u32 = 7;
pub const OTP_DBG_ROSC_UP_BIT:                      u32 = 3;
pub const OTP_DBG_ROSC_UP_SEEN_BIT:                 u32 = 2;
pub const OTP_DBG_BOOT_DONE_BIT:                    u32 = 1;
pub const OTP_DBG_PSM_DONE_BIT:                     u32 = 0;

// BIST
pub const OTP_BIST_CNT_FAIL_BIT:                    u32 = 30;
pub const OTP_BIST_CNT_CLR_BIT:                     u32 = 29;
pub const OTP_BIST_CNT_ENA_BIT:                     u32 = 28;
pub const OTP_BIST_CNT_MAX_LOW:                     u32 = 16;
pub const OTP_BIST_CNT_MAX_HIGH:                    u32 = 27;
pub const OTP_BIST_CNT_LOW:                         u32 = 0;
pub const OTP_BIST_CNT_HIGH:                        u32 = 12;

// CRITICAL
pub const OTP_CRITICAL_RISCV_DISABLE_BIT:           u32 = 17;
pub const OTP_CRITICAL_ARM_DISABLE_BIT:             u32 = 16;
pub const OTP_CRITICAL_GLITCH_DETECTOR_SENS_LOW:    u32 = 5;
pub const OTP_CRITICAL_GLITCH_DETECTOR_SENS_HIGH:   u32 = 6;
pub const OTP_CRITICAL_GLITCH_DETECTOR_ENABLE_BIT:  u32 = 4;
pub const OTP_CRITICAL_DEFAULT_ARCHSEL_BIT:         u32 = 3;
pub const OTP_CRITICAL_DEBUG_DISABLE_BIT:           u32 = 2;
pub const OTP_CRITICAL_SECURE_DEBUG_DISABLE_BIT:    u32 = 1;
pub const OTP_CRITICAL_SECURE_BOOT_ENABLE_BIT:      u32 = 0;

// KEY_VALID
pub const OTP_KEY_VALID_LOW:                        u32 = 0;
pub const OTP_KEY_VALID_HIGH:                       u32 = 7;

// DEBUGEN, DEBUGEN_LOCK
pub const OTP_DEBUGEN_MISC_BIT:                     u32 = 8;
pub const OTP_DEBUGEN_PROC1_SECURE_BIT:             u32 = 3;
pub const OTP_DEBUGEN_PROC0_SECURE_BIT:             u32 = 1;
pub const OTP_DEBUGEN_PROC1_BIT:                    u32 = 2;
pub const OTP_DEBUGEN_PROC0_BIT:                    u32 = 0;

// ARCHSEL, ARCHSEL_STATUS
pub const OTP_ARCHSEL_CORE1_BIT:                    u32 = 1;
pub const OTP_ARCHSEL_CORE0_BIT:                    u32 = 0;

// BOOTDIS
pub const OTP_BOOTDIS_NEXT_BIT:                     u32 = 1;
pub const OTP_BOOTDIS_NOW_BIT:                      u32 = 0;

// INTR
pub const OTP_INTR_APB_RD_NSEC_FAIL_BIT:            u32 = 4;
pub const OTP_INTR_APB_RD_SEC_FAIL_BIT:             u32 = 3;
pub const OTP_INTR_APB_DCTRL_FAIL_BIT:              u32 = 2;
pub const OTP_INTR_SBPI_WR_FAIL_BIT:                u32 = 1;
pub const OTP_INTR_SBPI_FLAG_N_BIT:                 u32 = 0;

// INTE
pub const OTP_INTE_APB_RD_NSEC_FAIL_BIT:            u32 = 4;
pub const OTP_INTE_APB_RD_SEC_FAIL_BIT:             u32 = 3;
pub const OTP_INTE_APB_DCTRL_FAIL_BIT:              u32 = 2;
pub const OTP_INTE_SBPI_WR_FAIL_BIT:                u32 = 1;
pub const OTP_INTE_SBPI_FLAG_N_BIT:                 u32 = 0;

// INTF
pub const OTP_INTF_APB_RD_NSEC_FAIL_BIT:            u32 = 4;
pub const OTP_INTF_APB_RD_SEC_FAIL_BIT:             u32 = 3;
pub const OTP_INTF_APB_DCTRL_FAIL_BIT:              u32 = 2;
pub const OTP_INTF_SBPI_WR_FAIL_BIT:                u32 = 1;
pub const OTP_INTF_SBPI_FLAG_N_BIT:                 u32 = 0;

// INTS
pub const OTP_INTS_APB_RD_NSEC_FAIL_BIT:            u32 = 4;
pub const OTP_INTS_APB_RD_SEC_FAIL_BIT:             u32 = 3;
pub const OTP_INTS_APB_DCTRL_FAIL_BIT:              u32 = 2;
pub const OTP_INTS_SBPI_WR_FAIL_BIT:                u32 = 1;
pub const OTP_INTS_SBPI_FLAG_N_BIT:                 u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====

// ==== BEGIN AUTO-GENERATED ENUMERATED VALUES (tools/gen_enum_values.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// SW_LOCK0..SW_LOCK63: NSEC
pub const OTP_SW_LOCK_NSEC_READ_WRITE:              u32 = 0x0;
pub const OTP_SW_LOCK_NSEC_READ_ONLY:               u32 = 0x1;
pub const OTP_SW_LOCK_NSEC_INACCESSIBLE:            u32 = 0x3;

// SW_LOCK0..SW_LOCK63: SEC
pub const OTP_SW_LOCK_SEC_READ_WRITE:               u32 = 0x0;
pub const OTP_SW_LOCK_SEC_READ_ONLY:                u32 = 0x1;
pub const OTP_SW_LOCK_SEC_INACCESSIBLE:             u32 = 0x3;

// ARCHSEL..ARCHSEL_STATUS: CORE
pub const OTP_ARCHSEL_CORE_ARM:                     u32 = 0x0;
pub const OTP_ARCHSEL_CORE_RISCV:                   u32 = 0x1;
// ==== END AUTO-GENERATED ENUMERATED VALUES ====
