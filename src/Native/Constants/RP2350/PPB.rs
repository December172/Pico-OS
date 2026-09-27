#![allow(dead_code)]
// PPB

// ==== BEGIN AUTO-GENERATED REGISTER OFFSETS (tools/gen_register_offsets.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

pub const PPB_BASE:                                 u32 = 0xE000_0000;
pub const PPB_NS_BASE:                              u32 = 0xE002_0000;

// ITM_STIM0..ITM_STIM31
pub fn PPB_ITM_STIM_OFFSET(n: u32) -> u32 {
    return 0x0 + n * 0x4
}

pub const PPB_ITM_TER0_OFFSET:                      u32 = 0xE00;
pub const PPB_ITM_TPR_OFFSET:                       u32 = 0xE40;
pub const PPB_ITM_TCR_OFFSET:                       u32 = 0xE80;
pub const PPB_INT_ATREADY_OFFSET:                   u32 = 0xEF0;
pub const PPB_INT_ATVALID_OFFSET:                   u32 = 0xEF8;
pub const PPB_ITM_ITCTRL_OFFSET:                    u32 = 0xF00;
pub const PPB_ITM_DEVARCH_OFFSET:                   u32 = 0xFBC;
pub const PPB_ITM_DEVTYPE_OFFSET:                   u32 = 0xFCC;
pub const PPB_ITM_PIDR4_OFFSET:                     u32 = 0xFD0;
pub const PPB_ITM_PIDR5_OFFSET:                     u32 = 0xFD4;
pub const PPB_ITM_PIDR6_OFFSET:                     u32 = 0xFD8;
pub const PPB_ITM_PIDR7_OFFSET:                     u32 = 0xFDC;
pub const PPB_ITM_PIDR0_OFFSET:                     u32 = 0xFE0;
pub const PPB_ITM_PIDR1_OFFSET:                     u32 = 0xFE4;
pub const PPB_ITM_PIDR2_OFFSET:                     u32 = 0xFE8;
pub const PPB_ITM_PIDR3_OFFSET:                     u32 = 0xFEC;
// ITM_CIDR0..ITM_CIDR3
pub fn PPB_ITM_CIDR_OFFSET(n: u32) -> u32 {
    return 0xFF0 + n * 0x4
}

pub const PPB_DWT_CTRL_OFFSET:                      u32 = 0x1000;
pub const PPB_DWT_CYCCNT_OFFSET:                    u32 = 0x1004;
pub const PPB_DWT_EXCCNT_OFFSET:                    u32 = 0x100C;
pub const PPB_DWT_LSUCNT_OFFSET:                    u32 = 0x1014;
pub const PPB_DWT_FOLDCNT_OFFSET:                   u32 = 0x1018;
// DWT_COMP0..DWT_COMP3
pub fn PPB_DWT_COMP_OFFSET(n: u32) -> u32 {
    return 0x1020 + n * 0x10
}

// DWT_FUNCTION0..DWT_FUNCTION3
pub fn PPB_DWT_FUNCTION_OFFSET(n: u32) -> u32 {
    return 0x1028 + n * 0x10
}

pub const PPB_DWT_DEVARCH_OFFSET:                   u32 = 0x1FBC;
pub const PPB_DWT_DEVTYPE_OFFSET:                   u32 = 0x1FCC;
pub const PPB_DWT_PIDR4_OFFSET:                     u32 = 0x1FD0;
pub const PPB_DWT_PIDR5_OFFSET:                     u32 = 0x1FD4;
pub const PPB_DWT_PIDR6_OFFSET:                     u32 = 0x1FD8;
pub const PPB_DWT_PIDR7_OFFSET:                     u32 = 0x1FDC;
pub const PPB_DWT_PIDR0_OFFSET:                     u32 = 0x1FE0;
pub const PPB_DWT_PIDR1_OFFSET:                     u32 = 0x1FE4;
pub const PPB_DWT_PIDR2_OFFSET:                     u32 = 0x1FE8;
pub const PPB_DWT_PIDR3_OFFSET:                     u32 = 0x1FEC;
// DWT_CIDR0..DWT_CIDR3
pub fn PPB_DWT_CIDR_OFFSET(n: u32) -> u32 {
    return 0x1FF0 + n * 0x4
}

pub const PPB_FP_CTRL_OFFSET:                       u32 = 0x2000;
pub const PPB_FP_REMAP_OFFSET:                      u32 = 0x2004;
// FP_COMP0..FP_COMP7
pub fn PPB_FP_COMP_OFFSET(n: u32) -> u32 {
    return 0x2008 + n * 0x4
}

pub const PPB_FP_DEVARCH_OFFSET:                    u32 = 0x2FBC;
pub const PPB_FP_DEVTYPE_OFFSET:                    u32 = 0x2FCC;
pub const PPB_FP_PIDR4_OFFSET:                      u32 = 0x2FD0;
pub const PPB_FP_PIDR5_OFFSET:                      u32 = 0x2FD4;
pub const PPB_FP_PIDR6_OFFSET:                      u32 = 0x2FD8;
pub const PPB_FP_PIDR7_OFFSET:                      u32 = 0x2FDC;
pub const PPB_FP_PIDR0_OFFSET:                      u32 = 0x2FE0;
pub const PPB_FP_PIDR1_OFFSET:                      u32 = 0x2FE4;
pub const PPB_FP_PIDR2_OFFSET:                      u32 = 0x2FE8;
pub const PPB_FP_PIDR3_OFFSET:                      u32 = 0x2FEC;
// FP_CIDR0..FP_CIDR3
pub fn PPB_FP_CIDR_OFFSET(n: u32) -> u32 {
    return 0x2FF0 + n * 0x4
}

pub const PPB_ICTR_OFFSET:                          u32 = 0xE004;
pub const PPB_ACTLR_OFFSET:                         u32 = 0xE008;
pub const PPB_SYST_CSR_OFFSET:                      u32 = 0xE010;
pub const PPB_SYST_RVR_OFFSET:                      u32 = 0xE014;
pub const PPB_SYST_CVR_OFFSET:                      u32 = 0xE018;
pub const PPB_SYST_CALIB_OFFSET:                    u32 = 0xE01C;
// NVIC_ISER0..NVIC_ISER1
pub fn PPB_NVIC_ISER_OFFSET(n: u32) -> u32 {
    return 0xE100 + n * 0x4
}

// NVIC_ICER0..NVIC_ICER1
pub fn PPB_NVIC_ICER_OFFSET(n: u32) -> u32 {
    return 0xE180 + n * 0x4
}

// NVIC_ISPR0..NVIC_ISPR1
pub fn PPB_NVIC_ISPR_OFFSET(n: u32) -> u32 {
    return 0xE200 + n * 0x4
}

// NVIC_ICPR0..NVIC_ICPR1
pub fn PPB_NVIC_ICPR_OFFSET(n: u32) -> u32 {
    return 0xE280 + n * 0x4
}

// NVIC_IABR0..NVIC_IABR1
pub fn PPB_NVIC_IABR_OFFSET(n: u32) -> u32 {
    return 0xE300 + n * 0x4
}

// NVIC_ITNS0..NVIC_ITNS1
pub fn PPB_NVIC_ITNS_OFFSET(n: u32) -> u32 {
    return 0xE380 + n * 0x4
}

// NVIC_IPR0..NVIC_IPR15
pub fn PPB_NVIC_IPR_OFFSET(n: u32) -> u32 {
    return 0xE400 + n * 0x4
}

pub const PPB_CPUID_OFFSET:                         u32 = 0xED00;
pub const PPB_ICSR_OFFSET:                          u32 = 0xED04;
pub const PPB_VTOR_OFFSET:                          u32 = 0xED08;
pub const PPB_AIRCR_OFFSET:                         u32 = 0xED0C;
pub const PPB_SCR_OFFSET:                           u32 = 0xED10;
pub const PPB_CCR_OFFSET:                           u32 = 0xED14;
pub const PPB_SHPR1_OFFSET:                         u32 = 0xED18;
pub const PPB_SHPR2_OFFSET:                         u32 = 0xED1C;
pub const PPB_SHPR3_OFFSET:                         u32 = 0xED20;
pub const PPB_SHCSR_OFFSET:                         u32 = 0xED24;
pub const PPB_CFSR_OFFSET:                          u32 = 0xED28;
pub const PPB_HFSR_OFFSET:                          u32 = 0xED2C;
pub const PPB_DFSR_OFFSET:                          u32 = 0xED30;
pub const PPB_MMFAR_OFFSET:                         u32 = 0xED34;
pub const PPB_BFAR_OFFSET:                          u32 = 0xED38;
// ID_PFR0..ID_PFR1
pub fn PPB_ID_PFR_OFFSET(n: u32) -> u32 {
    return 0xED40 + n * 0x4
}

pub const PPB_ID_DFR0_OFFSET:                       u32 = 0xED48;
pub const PPB_ID_AFR0_OFFSET:                       u32 = 0xED4C;
// ID_MMFR0..ID_MMFR3
pub fn PPB_ID_MMFR_OFFSET(n: u32) -> u32 {
    return 0xED50 + n * 0x4
}

// ID_ISAR0..ID_ISAR5
pub fn PPB_ID_ISAR_OFFSET(n: u32) -> u32 {
    return 0xED60 + n * 0x4
}

pub const PPB_CTR_OFFSET:                           u32 = 0xED7C;
pub const PPB_CPACR_OFFSET:                         u32 = 0xED88;
pub const PPB_NSACR_OFFSET:                         u32 = 0xED8C;
pub const PPB_MPU_TYPE_OFFSET:                      u32 = 0xED90;
pub const PPB_MPU_CTRL_OFFSET:                      u32 = 0xED94;
pub const PPB_MPU_RNR_OFFSET:                       u32 = 0xED98;
pub const PPB_MPU_RBAR_OFFSET:                      u32 = 0xED9C;
pub const PPB_MPU_RLAR_OFFSET:                      u32 = 0xEDA0;
pub const PPB_MPU_RBAR_A1_OFFSET:                   u32 = 0xEDA4;
pub const PPB_MPU_RBAR_A2_OFFSET:                   u32 = 0xEDAC;
pub const PPB_MPU_RBAR_A3_OFFSET:                   u32 = 0xEDB4;
pub const PPB_MPU_RLAR_A1_OFFSET:                   u32 = 0xEDA8;
pub const PPB_MPU_RLAR_A2_OFFSET:                   u32 = 0xEDB0;
pub const PPB_MPU_RLAR_A3_OFFSET:                   u32 = 0xEDB8;
// MPU_MAIR0..MPU_MAIR1
pub fn PPB_MPU_MAIR_OFFSET(n: u32) -> u32 {
    return 0xEDC0 + n * 0x4
}

pub const PPB_SAU_CTRL_OFFSET:                      u32 = 0xEDD0;
pub const PPB_SAU_TYPE_OFFSET:                      u32 = 0xEDD4;
pub const PPB_SAU_RNR_OFFSET:                       u32 = 0xEDD8;
pub const PPB_SAU_RBAR_OFFSET:                      u32 = 0xEDDC;
pub const PPB_SAU_RLAR_OFFSET:                      u32 = 0xEDE0;
pub const PPB_SFSR_OFFSET:                          u32 = 0xEDE4;
pub const PPB_SFAR_OFFSET:                          u32 = 0xEDE8;
pub const PPB_DHCSR_OFFSET:                         u32 = 0xEDF0;
pub const PPB_DCRSR_OFFSET:                         u32 = 0xEDF4;
pub const PPB_DCRDR_OFFSET:                         u32 = 0xEDF8;
pub const PPB_DEMCR_OFFSET:                         u32 = 0xEDFC;
pub const PPB_DSCSR_OFFSET:                         u32 = 0xEE08;
pub const PPB_STIR_OFFSET:                          u32 = 0xEF00;
pub const PPB_FPCCR_OFFSET:                         u32 = 0xEF34;
pub const PPB_FPCAR_OFFSET:                         u32 = 0xEF38;
pub const PPB_FPDSCR_OFFSET:                        u32 = 0xEF3C;
// MVFR0..MVFR2
pub fn PPB_MVFR_OFFSET(n: u32) -> u32 {
    return 0xEF40 + n * 0x4
}

pub const PPB_DDEVARCH_OFFSET:                      u32 = 0xEFBC;
pub const PPB_DDEVTYPE_OFFSET:                      u32 = 0xEFCC;
pub const PPB_DPIDR4_OFFSET:                        u32 = 0xEFD0;
pub const PPB_DPIDR5_OFFSET:                        u32 = 0xEFD4;
pub const PPB_DPIDR6_OFFSET:                        u32 = 0xEFD8;
pub const PPB_DPIDR7_OFFSET:                        u32 = 0xEFDC;
pub const PPB_DPIDR0_OFFSET:                        u32 = 0xEFE0;
pub const PPB_DPIDR1_OFFSET:                        u32 = 0xEFE4;
pub const PPB_DPIDR2_OFFSET:                        u32 = 0xEFE8;
pub const PPB_DPIDR3_OFFSET:                        u32 = 0xEFEC;
// DCIDR0..DCIDR3
pub fn PPB_DCIDR_OFFSET(n: u32) -> u32 {
    return 0xEFF0 + n * 0x4
}

pub const PPB_TRCPRGCTLR_OFFSET:                    u32 = 0x41004;
pub const PPB_TRCSTATR_OFFSET:                      u32 = 0x4100C;
pub const PPB_TRCCONFIGR_OFFSET:                    u32 = 0x41010;
// TRCEVENTCTL0R..TRCEVENTCTL1R
pub fn PPB_TRCEVENTCTLR_OFFSET(n: u32) -> u32 {
    return 0x41020 + n * 0x4
}

pub const PPB_TRCSTALLCTLR_OFFSET:                  u32 = 0x4102C;
pub const PPB_TRCTSCTLR_OFFSET:                     u32 = 0x41030;
pub const PPB_TRCSYNCPR_OFFSET:                     u32 = 0x41034;
pub const PPB_TRCCCCTLR_OFFSET:                     u32 = 0x41038;
pub const PPB_TRCVICTLR_OFFSET:                     u32 = 0x41080;
pub const PPB_TRCCNTRLDVR0_OFFSET:                  u32 = 0x41140;
pub const PPB_TRCIDR8_OFFSET:                       u32 = 0x41180;
pub const PPB_TRCIDR9_OFFSET:                       u32 = 0x41184;
pub const PPB_TRCIDR10_OFFSET:                      u32 = 0x41188;
pub const PPB_TRCIDR11_OFFSET:                      u32 = 0x4118C;
pub const PPB_TRCIDR12_OFFSET:                      u32 = 0x41190;
pub const PPB_TRCIDR13_OFFSET:                      u32 = 0x41194;
// TRCIDR0..TRCIDR7
pub fn PPB_TRCIDR_OFFSET(n: u32) -> u32 {
    return 0x411E0 + n * 0x4
}

pub const PPB_TRCIMSPEC_OFFSET:                     u32 = 0x411C0;
pub const PPB_TRCRSCTLR2_OFFSET:                    u32 = 0x41208;
pub const PPB_TRCRSCTLR3_OFFSET:                    u32 = 0x4120C;
pub const PPB_TRCSSCSR_OFFSET:                      u32 = 0x412A0;
pub const PPB_TRCSSPCICR_OFFSET:                    u32 = 0x412C0;
pub const PPB_TRCPDCR_OFFSET:                       u32 = 0x41310;
pub const PPB_TRCPDSR_OFFSET:                       u32 = 0x41314;
pub const PPB_TRCITATBIDR_OFFSET:                   u32 = 0x41EE4;
pub const PPB_TRCITIATBINR_OFFSET:                  u32 = 0x41EF4;
pub const PPB_TRCITIATBOUTR_OFFSET:                 u32 = 0x41EFC;
pub const PPB_TRCCLAIMSET_OFFSET:                   u32 = 0x41FA0;
pub const PPB_TRCCLAIMCLR_OFFSET:                   u32 = 0x41FA4;
pub const PPB_TRCAUTHSTATUS_OFFSET:                 u32 = 0x41FB8;
pub const PPB_TRCDEVARCH_OFFSET:                    u32 = 0x41FBC;
pub const PPB_TRCDEVID_OFFSET:                      u32 = 0x41FC8;
pub const PPB_TRCDEVTYPE_OFFSET:                    u32 = 0x41FCC;
pub const PPB_TRCPIDR4_OFFSET:                      u32 = 0x41FD0;
pub const PPB_TRCPIDR5_OFFSET:                      u32 = 0x41FD4;
pub const PPB_TRCPIDR6_OFFSET:                      u32 = 0x41FD8;
pub const PPB_TRCPIDR7_OFFSET:                      u32 = 0x41FDC;
pub const PPB_TRCPIDR0_OFFSET:                      u32 = 0x41FE0;
pub const PPB_TRCPIDR1_OFFSET:                      u32 = 0x41FE4;
pub const PPB_TRCPIDR2_OFFSET:                      u32 = 0x41FE8;
pub const PPB_TRCPIDR3_OFFSET:                      u32 = 0x41FEC;
// TRCCIDR0..TRCCIDR3
pub fn PPB_TRCCIDR_OFFSET(n: u32) -> u32 {
    return 0x41FF0 + n * 0x4
}

pub const PPB_CTICONTROL_OFFSET:                    u32 = 0x42000;
pub const PPB_CTIINTACK_OFFSET:                     u32 = 0x42010;
pub const PPB_CTIAPPSET_OFFSET:                     u32 = 0x42014;
pub const PPB_CTIAPPCLEAR_OFFSET:                   u32 = 0x42018;
pub const PPB_CTIAPPPULSE_OFFSET:                   u32 = 0x4201C;
// CTIINEN0..CTIINEN7
pub fn PPB_CTIINEN_OFFSET(n: u32) -> u32 {
    return 0x42020 + n * 0x4
}

// CTIOUTEN0..CTIOUTEN7
pub fn PPB_CTIOUTEN_OFFSET(n: u32) -> u32 {
    return 0x420A0 + n * 0x4
}

pub const PPB_CTITRIGINSTATUS_OFFSET:               u32 = 0x42130;
pub const PPB_CTITRIGOUTSTATUS_OFFSET:              u32 = 0x42134;
pub const PPB_CTICHINSTATUS_OFFSET:                 u32 = 0x42138;
pub const PPB_CTIGATE_OFFSET:                       u32 = 0x42140;
pub const PPB_ASICCTL_OFFSET:                       u32 = 0x42144;
pub const PPB_ITCHOUT_OFFSET:                       u32 = 0x42EE4;
pub const PPB_ITTRIGOUT_OFFSET:                     u32 = 0x42EE8;
pub const PPB_ITCHIN_OFFSET:                        u32 = 0x42EF4;
pub const PPB_ITCTRL_OFFSET:                        u32 = 0x42F00;
pub const PPB_DEVARCH_OFFSET:                       u32 = 0x42FBC;
pub const PPB_DEVID_OFFSET:                         u32 = 0x42FC8;
pub const PPB_DEVTYPE_OFFSET:                       u32 = 0x42FCC;
pub const PPB_PIDR4_OFFSET:                         u32 = 0x42FD0;
pub const PPB_PIDR5_OFFSET:                         u32 = 0x42FD4;
pub const PPB_PIDR6_OFFSET:                         u32 = 0x42FD8;
pub const PPB_PIDR7_OFFSET:                         u32 = 0x42FDC;
pub const PPB_PIDR0_OFFSET:                         u32 = 0x42FE0;
pub const PPB_PIDR1_OFFSET:                         u32 = 0x42FE4;
pub const PPB_PIDR2_OFFSET:                         u32 = 0x42FE8;
pub const PPB_PIDR3_OFFSET:                         u32 = 0x42FEC;
// CIDR0..CIDR3
pub fn PPB_CIDR_OFFSET(n: u32) -> u32 {
    return 0x42FF0 + n * 0x4
}
// ==== END AUTO-GENERATED REGISTER OFFSETS ====

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2350.svd -- do not edit by hand.

// ITM_TPR
pub const PPB_ITM_TPR_PRIVMASK_LOW:                 u32 = 0;
pub const PPB_ITM_TPR_PRIVMASK_HIGH:                u32 = 3;

// ITM_TCR
pub const PPB_ITM_TCR_BUSY_BIT:                     u32 = 23;
pub const PPB_ITM_TCR_TRACEBUSID_LOW:               u32 = 16;
pub const PPB_ITM_TCR_TRACEBUSID_HIGH:              u32 = 22;
pub const PPB_ITM_TCR_GTSFREQ_LOW:                  u32 = 10;
pub const PPB_ITM_TCR_GTSFREQ_HIGH:                 u32 = 11;
pub const PPB_ITM_TCR_TSPRESCALE_LOW:               u32 = 8;
pub const PPB_ITM_TCR_TSPRESCALE_HIGH:              u32 = 9;
pub const PPB_ITM_TCR_STALLENA_BIT:                 u32 = 5;
pub const PPB_ITM_TCR_SWOENA_BIT:                   u32 = 4;
pub const PPB_ITM_TCR_TXENA_BIT:                    u32 = 3;
pub const PPB_ITM_TCR_SYNCENA_BIT:                  u32 = 2;
pub const PPB_ITM_TCR_TSENA_BIT:                    u32 = 1;
pub const PPB_ITM_TCR_ITMENA_BIT:                   u32 = 0;

// INT_ATREADY
pub const PPB_INT_ATREADY_AFVALID_BIT:              u32 = 1;
pub const PPB_INT_ATREADY_ATREADY_BIT:              u32 = 0;

// INT_ATVALID
pub const PPB_INT_ATVALID_AFREADY_BIT:              u32 = 1;
pub const PPB_INT_ATVALID_ATREADY_BIT:              u32 = 0;

// ITM_ITCTRL, ITCTRL
pub const PPB_ITCTRL_IME_BIT:                       u32 = 0;

// ITM_DEVARCH
pub const PPB_ITM_DEVARCH_ARCHITECT_LOW:            u32 = 21;
pub const PPB_ITM_DEVARCH_ARCHITECT_HIGH:           u32 = 31;
pub const PPB_ITM_DEVARCH_PRESENT_BIT:              u32 = 20;
pub const PPB_ITM_DEVARCH_REVISION_LOW:             u32 = 16;
pub const PPB_ITM_DEVARCH_REVISION_HIGH:            u32 = 19;
pub const PPB_ITM_DEVARCH_ARCHVER_LOW:              u32 = 12;
pub const PPB_ITM_DEVARCH_ARCHVER_HIGH:             u32 = 15;
pub const PPB_ITM_DEVARCH_ARCHPART_LOW:             u32 = 0;
pub const PPB_ITM_DEVARCH_ARCHPART_HIGH:            u32 = 11;

// DWT_DEVARCH
pub const PPB_DWT_DEVARCH_ARCHITECT_LOW:            u32 = 21;
pub const PPB_DWT_DEVARCH_ARCHITECT_HIGH:           u32 = 31;
pub const PPB_DWT_DEVARCH_PRESENT_BIT:              u32 = 20;
pub const PPB_DWT_DEVARCH_REVISION_LOW:             u32 = 16;
pub const PPB_DWT_DEVARCH_REVISION_HIGH:            u32 = 19;
pub const PPB_DWT_DEVARCH_ARCHVER_LOW:              u32 = 12;
pub const PPB_DWT_DEVARCH_ARCHVER_HIGH:             u32 = 15;
pub const PPB_DWT_DEVARCH_ARCHPART_LOW:             u32 = 0;
pub const PPB_DWT_DEVARCH_ARCHPART_HIGH:            u32 = 11;

// FP_DEVARCH
pub const PPB_FP_DEVARCH_ARCHITECT_LOW:             u32 = 21;
pub const PPB_FP_DEVARCH_ARCHITECT_HIGH:            u32 = 31;
pub const PPB_FP_DEVARCH_PRESENT_BIT:               u32 = 20;
pub const PPB_FP_DEVARCH_REVISION_LOW:              u32 = 16;
pub const PPB_FP_DEVARCH_REVISION_HIGH:             u32 = 19;
pub const PPB_FP_DEVARCH_ARCHVER_LOW:               u32 = 12;
pub const PPB_FP_DEVARCH_ARCHVER_HIGH:              u32 = 15;
pub const PPB_FP_DEVARCH_ARCHPART_LOW:              u32 = 0;
pub const PPB_FP_DEVARCH_ARCHPART_HIGH:             u32 = 11;

// DDEVARCH
pub const PPB_DDEVARCH_ARCHITECT_LOW:               u32 = 21;
pub const PPB_DDEVARCH_ARCHITECT_HIGH:              u32 = 31;
pub const PPB_DDEVARCH_PRESENT_BIT:                 u32 = 20;
pub const PPB_DDEVARCH_REVISION_LOW:                u32 = 16;
pub const PPB_DDEVARCH_REVISION_HIGH:               u32 = 19;
pub const PPB_DDEVARCH_ARCHVER_LOW:                 u32 = 12;
pub const PPB_DDEVARCH_ARCHVER_HIGH:                u32 = 15;
pub const PPB_DDEVARCH_ARCHPART_LOW:                u32 = 0;
pub const PPB_DDEVARCH_ARCHPART_HIGH:               u32 = 11;

// ITM_DEVTYPE
pub const PPB_ITM_DEVTYPE_SUB_LOW:                  u32 = 4;
pub const PPB_ITM_DEVTYPE_SUB_HIGH:                 u32 = 7;
pub const PPB_ITM_DEVTYPE_MAJOR_LOW:                u32 = 0;
pub const PPB_ITM_DEVTYPE_MAJOR_HIGH:               u32 = 3;

// DWT_DEVTYPE
pub const PPB_DWT_DEVTYPE_SUB_LOW:                  u32 = 4;
pub const PPB_DWT_DEVTYPE_SUB_HIGH:                 u32 = 7;
pub const PPB_DWT_DEVTYPE_MAJOR_LOW:                u32 = 0;
pub const PPB_DWT_DEVTYPE_MAJOR_HIGH:               u32 = 3;

// FP_DEVTYPE
pub const PPB_FP_DEVTYPE_SUB_LOW:                   u32 = 4;
pub const PPB_FP_DEVTYPE_SUB_HIGH:                  u32 = 7;
pub const PPB_FP_DEVTYPE_MAJOR_LOW:                 u32 = 0;
pub const PPB_FP_DEVTYPE_MAJOR_HIGH:                u32 = 3;

// DDEVTYPE
pub const PPB_DDEVTYPE_SUB_LOW:                     u32 = 4;
pub const PPB_DDEVTYPE_SUB_HIGH:                    u32 = 7;
pub const PPB_DDEVTYPE_MAJOR_LOW:                   u32 = 0;
pub const PPB_DDEVTYPE_MAJOR_HIGH:                  u32 = 3;

// TRCDEVTYPE
pub const PPB_TRCDEVTYPE_SUB_LOW:                   u32 = 4;
pub const PPB_TRCDEVTYPE_SUB_HIGH:                  u32 = 7;
pub const PPB_TRCDEVTYPE_MAJOR_LOW:                 u32 = 0;
pub const PPB_TRCDEVTYPE_MAJOR_HIGH:                u32 = 3;

// DEVTYPE
pub const PPB_DEVTYPE_SUB_LOW:                      u32 = 4;
pub const PPB_DEVTYPE_SUB_HIGH:                     u32 = 7;
pub const PPB_DEVTYPE_MAJOR_LOW:                    u32 = 0;
pub const PPB_DEVTYPE_MAJOR_HIGH:                   u32 = 3;

// ITM_PIDR4
pub const PPB_ITM_PIDR4_SIZE_LOW:                   u32 = 4;
pub const PPB_ITM_PIDR4_SIZE_HIGH:                  u32 = 7;
pub const PPB_ITM_PIDR4_DES_2_LOW:                  u32 = 0;
pub const PPB_ITM_PIDR4_DES_2_HIGH:                 u32 = 3;

// DWT_PIDR4
pub const PPB_DWT_PIDR4_SIZE_LOW:                   u32 = 4;
pub const PPB_DWT_PIDR4_SIZE_HIGH:                  u32 = 7;
pub const PPB_DWT_PIDR4_DES_2_LOW:                  u32 = 0;
pub const PPB_DWT_PIDR4_DES_2_HIGH:                 u32 = 3;

// FP_PIDR4
pub const PPB_FP_PIDR4_SIZE_LOW:                    u32 = 4;
pub const PPB_FP_PIDR4_SIZE_HIGH:                   u32 = 7;
pub const PPB_FP_PIDR4_DES_2_LOW:                   u32 = 0;
pub const PPB_FP_PIDR4_DES_2_HIGH:                  u32 = 3;

// DPIDR4
pub const PPB_DPIDR4_SIZE_LOW:                      u32 = 4;
pub const PPB_DPIDR4_SIZE_HIGH:                     u32 = 7;
pub const PPB_DPIDR4_DES_2_LOW:                     u32 = 0;
pub const PPB_DPIDR4_DES_2_HIGH:                    u32 = 3;

// TRCPIDR4
pub const PPB_TRCPIDR4_SIZE_LOW:                    u32 = 4;
pub const PPB_TRCPIDR4_SIZE_HIGH:                   u32 = 7;
pub const PPB_TRCPIDR4_DES_2_LOW:                   u32 = 0;
pub const PPB_TRCPIDR4_DES_2_HIGH:                  u32 = 3;

// PIDR4
pub const PPB_PIDR4_SIZE_LOW:                       u32 = 4;
pub const PPB_PIDR4_SIZE_HIGH:                      u32 = 7;
pub const PPB_PIDR4_DES_2_LOW:                      u32 = 0;
pub const PPB_PIDR4_DES_2_HIGH:                     u32 = 3;

// ITM_PIDR0
pub const PPB_ITM_PIDR0_PART_0_LOW:                 u32 = 0;
pub const PPB_ITM_PIDR0_PART_0_HIGH:                u32 = 7;

// DWT_PIDR0
pub const PPB_DWT_PIDR0_PART_0_LOW:                 u32 = 0;
pub const PPB_DWT_PIDR0_PART_0_HIGH:                u32 = 7;

// FP_PIDR0
pub const PPB_FP_PIDR0_PART_0_LOW:                  u32 = 0;
pub const PPB_FP_PIDR0_PART_0_HIGH:                 u32 = 7;

// DPIDR0
pub const PPB_DPIDR0_PART_0_LOW:                    u32 = 0;
pub const PPB_DPIDR0_PART_0_HIGH:                   u32 = 7;

// TRCPIDR0
pub const PPB_TRCPIDR0_PART_0_LOW:                  u32 = 0;
pub const PPB_TRCPIDR0_PART_0_HIGH:                 u32 = 7;

// PIDR0
pub const PPB_PIDR0_PART_0_LOW:                     u32 = 0;
pub const PPB_PIDR0_PART_0_HIGH:                    u32 = 7;

// ITM_PIDR1
pub const PPB_ITM_PIDR1_DES_0_LOW:                  u32 = 4;
pub const PPB_ITM_PIDR1_DES_0_HIGH:                 u32 = 7;
pub const PPB_ITM_PIDR1_PART_1_LOW:                 u32 = 0;
pub const PPB_ITM_PIDR1_PART_1_HIGH:                u32 = 3;

// DWT_PIDR1
pub const PPB_DWT_PIDR1_DES_0_LOW:                  u32 = 4;
pub const PPB_DWT_PIDR1_DES_0_HIGH:                 u32 = 7;
pub const PPB_DWT_PIDR1_PART_1_LOW:                 u32 = 0;
pub const PPB_DWT_PIDR1_PART_1_HIGH:                u32 = 3;

// FP_PIDR1
pub const PPB_FP_PIDR1_DES_0_LOW:                   u32 = 4;
pub const PPB_FP_PIDR1_DES_0_HIGH:                  u32 = 7;
pub const PPB_FP_PIDR1_PART_1_LOW:                  u32 = 0;
pub const PPB_FP_PIDR1_PART_1_HIGH:                 u32 = 3;

// DPIDR1
pub const PPB_DPIDR1_DES_0_LOW:                     u32 = 4;
pub const PPB_DPIDR1_DES_0_HIGH:                    u32 = 7;
pub const PPB_DPIDR1_PART_1_LOW:                    u32 = 0;
pub const PPB_DPIDR1_PART_1_HIGH:                   u32 = 3;

// TRCPIDR1
pub const PPB_TRCPIDR1_DES_0_LOW:                   u32 = 4;
pub const PPB_TRCPIDR1_DES_0_HIGH:                  u32 = 7;
pub const PPB_TRCPIDR1_PART_0_LOW:                  u32 = 0;
pub const PPB_TRCPIDR1_PART_0_HIGH:                 u32 = 3;

// PIDR1
pub const PPB_PIDR1_DES_0_LOW:                      u32 = 4;
pub const PPB_PIDR1_DES_0_HIGH:                     u32 = 7;
pub const PPB_PIDR1_PART_1_LOW:                     u32 = 0;
pub const PPB_PIDR1_PART_1_HIGH:                    u32 = 3;

// ITM_PIDR2
pub const PPB_ITM_PIDR2_REVISION_LOW:               u32 = 4;
pub const PPB_ITM_PIDR2_REVISION_HIGH:              u32 = 7;
pub const PPB_ITM_PIDR2_JEDEC_BIT:                  u32 = 3;
pub const PPB_ITM_PIDR2_DES_1_LOW:                  u32 = 0;
pub const PPB_ITM_PIDR2_DES_1_HIGH:                 u32 = 2;

// DWT_PIDR2
pub const PPB_DWT_PIDR2_REVISION_LOW:               u32 = 4;
pub const PPB_DWT_PIDR2_REVISION_HIGH:              u32 = 7;
pub const PPB_DWT_PIDR2_JEDEC_BIT:                  u32 = 3;
pub const PPB_DWT_PIDR2_DES_1_LOW:                  u32 = 0;
pub const PPB_DWT_PIDR2_DES_1_HIGH:                 u32 = 2;

// FP_PIDR2
pub const PPB_FP_PIDR2_REVISION_LOW:                u32 = 4;
pub const PPB_FP_PIDR2_REVISION_HIGH:               u32 = 7;
pub const PPB_FP_PIDR2_JEDEC_BIT:                   u32 = 3;
pub const PPB_FP_PIDR2_DES_1_LOW:                   u32 = 0;
pub const PPB_FP_PIDR2_DES_1_HIGH:                  u32 = 2;

// DPIDR2
pub const PPB_DPIDR2_REVISION_LOW:                  u32 = 4;
pub const PPB_DPIDR2_REVISION_HIGH:                 u32 = 7;
pub const PPB_DPIDR2_JEDEC_BIT:                     u32 = 3;
pub const PPB_DPIDR2_DES_1_LOW:                     u32 = 0;
pub const PPB_DPIDR2_DES_1_HIGH:                    u32 = 2;

// TRCPIDR2
pub const PPB_TRCPIDR2_REVISION_LOW:                u32 = 4;
pub const PPB_TRCPIDR2_REVISION_HIGH:               u32 = 7;
pub const PPB_TRCPIDR2_JEDEC_BIT:                   u32 = 3;
pub const PPB_TRCPIDR2_DES_0_LOW:                   u32 = 0;
pub const PPB_TRCPIDR2_DES_0_HIGH:                  u32 = 2;

// PIDR2
pub const PPB_PIDR2_REVISION_LOW:                   u32 = 4;
pub const PPB_PIDR2_REVISION_HIGH:                  u32 = 7;
pub const PPB_PIDR2_JEDEC_BIT:                      u32 = 3;
pub const PPB_PIDR2_DES_1_LOW:                      u32 = 0;
pub const PPB_PIDR2_DES_1_HIGH:                     u32 = 2;

// ITM_PIDR3
pub const PPB_ITM_PIDR3_REVAND_LOW:                 u32 = 4;
pub const PPB_ITM_PIDR3_REVAND_HIGH:                u32 = 7;
pub const PPB_ITM_PIDR3_CMOD_LOW:                   u32 = 0;
pub const PPB_ITM_PIDR3_CMOD_HIGH:                  u32 = 3;

// DWT_PIDR3
pub const PPB_DWT_PIDR3_REVAND_LOW:                 u32 = 4;
pub const PPB_DWT_PIDR3_REVAND_HIGH:                u32 = 7;
pub const PPB_DWT_PIDR3_CMOD_LOW:                   u32 = 0;
pub const PPB_DWT_PIDR3_CMOD_HIGH:                  u32 = 3;

// FP_PIDR3
pub const PPB_FP_PIDR3_REVAND_LOW:                  u32 = 4;
pub const PPB_FP_PIDR3_REVAND_HIGH:                 u32 = 7;
pub const PPB_FP_PIDR3_CMOD_LOW:                    u32 = 0;
pub const PPB_FP_PIDR3_CMOD_HIGH:                   u32 = 3;

// DPIDR3
pub const PPB_DPIDR3_REVAND_LOW:                    u32 = 4;
pub const PPB_DPIDR3_REVAND_HIGH:                   u32 = 7;
pub const PPB_DPIDR3_CMOD_LOW:                      u32 = 0;
pub const PPB_DPIDR3_CMOD_HIGH:                     u32 = 3;

// TRCPIDR3
pub const PPB_TRCPIDR3_REVAND_LOW:                  u32 = 4;
pub const PPB_TRCPIDR3_REVAND_HIGH:                 u32 = 7;
pub const PPB_TRCPIDR3_CMOD_LOW:                    u32 = 0;
pub const PPB_TRCPIDR3_CMOD_HIGH:                   u32 = 3;

// PIDR3
pub const PPB_PIDR3_REVAND_LOW:                     u32 = 4;
pub const PPB_PIDR3_REVAND_HIGH:                    u32 = 7;
pub const PPB_PIDR3_CMOD_LOW:                       u32 = 0;
pub const PPB_PIDR3_CMOD_HIGH:                      u32 = 3;

// ITM_CIDR0, ITM_CIDR2, ITM_CIDR3
pub const PPB_ITM_CIDR_PRMBL_LOW:                   u32 = 0;
pub const PPB_ITM_CIDR_PRMBL_HIGH:                  u32 = 7;

// DWT_CIDR0, DWT_CIDR2, DWT_CIDR3
pub const PPB_DWT_CIDR_PRMBL_LOW:                   u32 = 0;
pub const PPB_DWT_CIDR_PRMBL_HIGH:                  u32 = 7;

// FP_CIDR0, FP_CIDR2, FP_CIDR3
pub const PPB_FP_CIDR_PRMBL_LOW:                    u32 = 0;
pub const PPB_FP_CIDR_PRMBL_HIGH:                   u32 = 7;

// DCIDR0, DCIDR2, DCIDR3
pub const PPB_DCIDR_PRMBL_LOW:                      u32 = 0;
pub const PPB_DCIDR_PRMBL_HIGH:                     u32 = 7;

// TRCCIDR0, TRCCIDR2, TRCCIDR3
pub const PPB_TRCCIDR_PRMBL_LOW:                    u32 = 0;
pub const PPB_TRCCIDR_PRMBL_HIGH:                   u32 = 7;

// CIDR0, CIDR2, CIDR3
pub const PPB_CIDR_PRMBL_LOW:                       u32 = 0;
pub const PPB_CIDR_PRMBL_HIGH:                      u32 = 7;

// ITM_CIDR1
pub const PPB_ITM_CIDR1_CLASS_LOW:                  u32 = 4;
pub const PPB_ITM_CIDR1_CLASS_HIGH:                 u32 = 7;
pub const PPB_ITM_CIDR1_PRMBL_1_LOW:                u32 = 0;
pub const PPB_ITM_CIDR1_PRMBL_1_HIGH:               u32 = 3;

// DWT_CIDR1
pub const PPB_DWT_CIDR1_CLASS_LOW:                  u32 = 4;
pub const PPB_DWT_CIDR1_CLASS_HIGH:                 u32 = 7;
pub const PPB_DWT_CIDR1_PRMBL_1_LOW:                u32 = 0;
pub const PPB_DWT_CIDR1_PRMBL_1_HIGH:               u32 = 3;

// FP_CIDR1
pub const PPB_FP_CIDR1_CLASS_LOW:                   u32 = 4;
pub const PPB_FP_CIDR1_CLASS_HIGH:                  u32 = 7;
pub const PPB_FP_CIDR1_PRMBL_1_LOW:                 u32 = 0;
pub const PPB_FP_CIDR1_PRMBL_1_HIGH:                u32 = 3;

// DCIDR1
pub const PPB_DCIDR1_CLASS_LOW:                     u32 = 4;
pub const PPB_DCIDR1_CLASS_HIGH:                    u32 = 7;
pub const PPB_DCIDR1_PRMBL_1_LOW:                   u32 = 0;
pub const PPB_DCIDR1_PRMBL_1_HIGH:                  u32 = 3;

// TRCCIDR1
pub const PPB_TRCCIDR1_CLASS_LOW:                   u32 = 4;
pub const PPB_TRCCIDR1_CLASS_HIGH:                  u32 = 7;
pub const PPB_TRCCIDR1_PRMBL_1_LOW:                 u32 = 0;
pub const PPB_TRCCIDR1_PRMBL_1_HIGH:                u32 = 3;

// CIDR1
pub const PPB_CIDR1_CLASS_LOW:                      u32 = 4;
pub const PPB_CIDR1_CLASS_HIGH:                     u32 = 7;
pub const PPB_CIDR1_PRMBL_1_LOW:                    u32 = 0;
pub const PPB_CIDR1_PRMBL_1_HIGH:                   u32 = 3;

// DWT_CTRL
pub const PPB_DWT_CTRL_NUMCOMP_LOW:                 u32 = 28;
pub const PPB_DWT_CTRL_NUMCOMP_HIGH:                u32 = 31;
pub const PPB_DWT_CTRL_NOTRCPKT_BIT:                u32 = 27;
pub const PPB_DWT_CTRL_NOEXTTRIG_BIT:               u32 = 26;
pub const PPB_DWT_CTRL_NOCYCCNT_BIT:                u32 = 25;
pub const PPB_DWT_CTRL_NOPRFCNT_BIT:                u32 = 24;
pub const PPB_DWT_CTRL_CYCDISS_BIT:                 u32 = 23;
pub const PPB_DWT_CTRL_CYCEVTENA_BIT:               u32 = 22;
pub const PPB_DWT_CTRL_FOLDEVTENA_BIT:              u32 = 21;
pub const PPB_DWT_CTRL_LSUEVTENA_BIT:               u32 = 20;
pub const PPB_DWT_CTRL_SLEEPEVTENA_BIT:             u32 = 19;
pub const PPB_DWT_CTRL_EXCEVTENA_BIT:               u32 = 18;
pub const PPB_DWT_CTRL_CPIEVTENA_BIT:               u32 = 17;
pub const PPB_DWT_CTRL_EXTTRCENA_BIT:               u32 = 16;
pub const PPB_DWT_CTRL_PCSAMPLENA_BIT:              u32 = 12;
pub const PPB_DWT_CTRL_SYNCTAP_LOW:                 u32 = 10;
pub const PPB_DWT_CTRL_SYNCTAP_HIGH:                u32 = 11;
pub const PPB_DWT_CTRL_CYCTAP_BIT:                  u32 = 9;
pub const PPB_DWT_CTRL_POSTINIT_LOW:                u32 = 5;
pub const PPB_DWT_CTRL_POSTINIT_HIGH:               u32 = 8;
pub const PPB_DWT_CTRL_POSTPRESET_LOW:              u32 = 1;
pub const PPB_DWT_CTRL_POSTPRESET_HIGH:             u32 = 4;
pub const PPB_DWT_CTRL_CYCCNTENA_BIT:               u32 = 0;

// DWT_EXCCNT
pub const PPB_DWT_EXCCNT_EXCCNT_LOW:                u32 = 0;
pub const PPB_DWT_EXCCNT_EXCCNT_HIGH:               u32 = 7;

// DWT_LSUCNT
pub const PPB_DWT_LSUCNT_LSUCNT_LOW:                u32 = 0;
pub const PPB_DWT_LSUCNT_LSUCNT_HIGH:               u32 = 7;

// DWT_FOLDCNT
pub const PPB_DWT_FOLDCNT_FOLDCNT_LOW:              u32 = 0;
pub const PPB_DWT_FOLDCNT_FOLDCNT_HIGH:             u32 = 7;

// DWT_FUNCTION0, DWT_FUNCTION1, DWT_FUNCTION2, DWT_FUNCTION3
pub const PPB_DWT_FUNCTION_ID_LOW:                  u32 = 27;
pub const PPB_DWT_FUNCTION_ID_HIGH:                 u32 = 31;
pub const PPB_DWT_FUNCTION_MATCHED_BIT:             u32 = 24;
pub const PPB_DWT_FUNCTION_DATAVSIZE_LOW:           u32 = 10;
pub const PPB_DWT_FUNCTION_DATAVSIZE_HIGH:          u32 = 11;
pub const PPB_DWT_FUNCTION_ACTION_LOW:              u32 = 4;
pub const PPB_DWT_FUNCTION_ACTION_HIGH:             u32 = 5;
pub const PPB_DWT_FUNCTION_MATCH_LOW:               u32 = 0;
pub const PPB_DWT_FUNCTION_MATCH_HIGH:              u32 = 3;

// FP_CTRL
pub const PPB_FP_CTRL_REV_LOW:                      u32 = 28;
pub const PPB_FP_CTRL_REV_HIGH:                     u32 = 31;
pub const PPB_FP_CTRL_NUM_CODE_14_12_LOW:           u32 = 12;
pub const PPB_FP_CTRL_NUM_CODE_14_12_HIGH:          u32 = 14;
pub const PPB_FP_CTRL_NUM_CODE_7_4_LOW:             u32 = 4;
pub const PPB_FP_CTRL_NUM_CODE_7_4_HIGH:            u32 = 7;
pub const PPB_FP_CTRL_NUM_LIT_LOW:                  u32 = 8;
pub const PPB_FP_CTRL_NUM_LIT_HIGH:                 u32 = 11;
pub const PPB_FP_CTRL_KEY_BIT:                      u32 = 1;
pub const PPB_FP_CTRL_ENABLE_BIT:                   u32 = 0;

// FP_REMAP
pub const PPB_FP_REMAP_RMPSPT_BIT:                  u32 = 29;
pub const PPB_FP_REMAP_REMAP_LOW:                   u32 = 5;
pub const PPB_FP_REMAP_REMAP_HIGH:                  u32 = 28;

// FP_COMP0, FP_COMP1, FP_COMP2, FP_COMP3, FP_COMP4, FP_COMP5, FP_COMP6, FP_COMP7
pub const PPB_FP_COMP_BE_BIT:                       u32 = 0;

// ICTR
pub const PPB_ICTR_INTLINESNUM_LOW:                 u32 = 0;
pub const PPB_ICTR_INTLINESNUM_HIGH:                u32 = 3;

// ACTLR
pub const PPB_ACTLR_EXTEXCLALL_BIT:                 u32 = 29;
pub const PPB_ACTLR_DISITMATBFLUSH_BIT:             u32 = 12;
pub const PPB_ACTLR_FPEXCODIS_BIT:                  u32 = 10;
pub const PPB_ACTLR_DISOOFP_BIT:                    u32 = 9;
pub const PPB_ACTLR_DISFOLD_BIT:                    u32 = 2;
pub const PPB_ACTLR_DISMCYCINT_BIT:                 u32 = 0;

// SYST_CSR
pub const PPB_SYST_CSR_COUNTFLAG_BIT:               u32 = 16;
pub const PPB_SYST_CSR_CLKSOURCE_BIT:               u32 = 2;
pub const PPB_SYST_CSR_TICKINT_BIT:                 u32 = 1;
pub const PPB_SYST_CSR_ENABLE_BIT:                  u32 = 0;

// SYST_RVR
pub const PPB_SYST_RVR_RELOAD_LOW:                  u32 = 0;
pub const PPB_SYST_RVR_RELOAD_HIGH:                 u32 = 23;

// SYST_CVR
pub const PPB_SYST_CVR_CURRENT_LOW:                 u32 = 0;
pub const PPB_SYST_CVR_CURRENT_HIGH:                u32 = 23;

// SYST_CALIB
pub const PPB_SYST_CALIB_NOREF_BIT:                 u32 = 31;
pub const PPB_SYST_CALIB_SKEW_BIT:                  u32 = 30;
pub const PPB_SYST_CALIB_TENMS_LOW:                 u32 = 0;
pub const PPB_SYST_CALIB_TENMS_HIGH:                u32 = 23;

// NVIC_IPR0..NVIC_IPR15
pub const PPB_NVIC_IPR_PRI_N3_LOW:                  u32 = 28;
pub const PPB_NVIC_IPR_PRI_N3_HIGH:                 u32 = 31;
pub const PPB_NVIC_IPR_PRI_N2_LOW:                  u32 = 20;
pub const PPB_NVIC_IPR_PRI_N2_HIGH:                 u32 = 23;
pub const PPB_NVIC_IPR_PRI_N1_LOW:                  u32 = 12;
pub const PPB_NVIC_IPR_PRI_N1_HIGH:                 u32 = 15;
pub const PPB_NVIC_IPR_PRI_N0_LOW:                  u32 = 4;
pub const PPB_NVIC_IPR_PRI_N0_HIGH:                 u32 = 7;

// CPUID
pub const PPB_CPUID_IMPLEMENTER_LOW:                u32 = 24;
pub const PPB_CPUID_IMPLEMENTER_HIGH:               u32 = 31;
pub const PPB_CPUID_VARIANT_LOW:                    u32 = 20;
pub const PPB_CPUID_VARIANT_HIGH:                   u32 = 23;
pub const PPB_CPUID_ARCHITECTURE_LOW:               u32 = 16;
pub const PPB_CPUID_ARCHITECTURE_HIGH:              u32 = 19;
pub const PPB_CPUID_PARTNO_LOW:                     u32 = 4;
pub const PPB_CPUID_PARTNO_HIGH:                    u32 = 15;
pub const PPB_CPUID_REVISION_LOW:                   u32 = 0;
pub const PPB_CPUID_REVISION_HIGH:                  u32 = 3;

// ICSR
pub const PPB_ICSR_PENDNMISET_BIT:                  u32 = 31;
pub const PPB_ICSR_PENDNMICLR_BIT:                  u32 = 30;
pub const PPB_ICSR_PENDSVSET_BIT:                   u32 = 28;
pub const PPB_ICSR_PENDSVCLR_BIT:                   u32 = 27;
pub const PPB_ICSR_PENDSTSET_BIT:                   u32 = 26;
pub const PPB_ICSR_PENDSTCLR_BIT:                   u32 = 25;
pub const PPB_ICSR_STTNS_BIT:                       u32 = 24;
pub const PPB_ICSR_ISRPREEMPT_BIT:                  u32 = 23;
pub const PPB_ICSR_ISRPENDING_BIT:                  u32 = 22;
pub const PPB_ICSR_VECTPENDING_LOW:                 u32 = 12;
pub const PPB_ICSR_VECTPENDING_HIGH:                u32 = 20;
pub const PPB_ICSR_RETTOBASE_BIT:                   u32 = 11;
pub const PPB_ICSR_VECTACTIVE_LOW:                  u32 = 0;
pub const PPB_ICSR_VECTACTIVE_HIGH:                 u32 = 8;

// VTOR
pub const PPB_VTOR_TBLOFF_LOW:                      u32 = 7;
pub const PPB_VTOR_TBLOFF_HIGH:                     u32 = 31;

// AIRCR
pub const PPB_AIRCR_VECTKEY_LOW:                    u32 = 16;
pub const PPB_AIRCR_VECTKEY_HIGH:                   u32 = 31;
pub const PPB_AIRCR_ENDIANESS_BIT:                  u32 = 15;
pub const PPB_AIRCR_PRIS_BIT:                       u32 = 14;
pub const PPB_AIRCR_BFHFNMINS_BIT:                  u32 = 13;
pub const PPB_AIRCR_PRIGROUP_LOW:                   u32 = 8;
pub const PPB_AIRCR_PRIGROUP_HIGH:                  u32 = 10;
pub const PPB_AIRCR_SYSRESETREQS_BIT:               u32 = 3;
pub const PPB_AIRCR_SYSRESETREQ_BIT:                u32 = 2;
pub const PPB_AIRCR_VECTCLRACTIVE_BIT:              u32 = 1;

// SCR
pub const PPB_SCR_SEVONPEND_BIT:                    u32 = 4;
pub const PPB_SCR_SLEEPDEEPS_BIT:                   u32 = 3;
pub const PPB_SCR_SLEEPDEEP_BIT:                    u32 = 2;
pub const PPB_SCR_SLEEPONEXIT_BIT:                  u32 = 1;

// CCR
pub const PPB_CCR_BP_BIT:                           u32 = 18;
pub const PPB_CCR_IC_BIT:                           u32 = 17;
pub const PPB_CCR_DC_BIT:                           u32 = 16;
pub const PPB_CCR_STKOFHFNMIGN_BIT:                 u32 = 10;
pub const PPB_CCR_RES1_BIT:                         u32 = 9;
pub const PPB_CCR_RES1_1_BIT:                       u32 = 0;
pub const PPB_CCR_BFHFNMIGN_BIT:                    u32 = 8;
pub const PPB_CCR_DIV_0_TRP_BIT:                    u32 = 4;
pub const PPB_CCR_UNALIGN_TRP_BIT:                  u32 = 3;
pub const PPB_CCR_USERSETMPEND_BIT:                 u32 = 1;

// SHPR1
pub const PPB_SHPR1_PRI_7_3_LOW:                    u32 = 29;
pub const PPB_SHPR1_PRI_7_3_HIGH:                   u32 = 31;
pub const PPB_SHPR1_PRI_6_3_LOW:                    u32 = 21;
pub const PPB_SHPR1_PRI_6_3_HIGH:                   u32 = 23;
pub const PPB_SHPR1_PRI_5_3_LOW:                    u32 = 13;
pub const PPB_SHPR1_PRI_5_3_HIGH:                   u32 = 15;
pub const PPB_SHPR1_PRI_4_3_LOW:                    u32 = 5;
pub const PPB_SHPR1_PRI_4_3_HIGH:                   u32 = 7;

// SHPR2
pub const PPB_SHPR2_PRI_11_3_LOW:                   u32 = 29;
pub const PPB_SHPR2_PRI_11_3_HIGH:                  u32 = 31;
pub const PPB_SHPR2_PRI_10_LOW:                     u32 = 16;
pub const PPB_SHPR2_PRI_10_HIGH:                    u32 = 23;
pub const PPB_SHPR2_PRI_9_LOW:                      u32 = 8;
pub const PPB_SHPR2_PRI_9_HIGH:                     u32 = 15;
pub const PPB_SHPR2_PRI_8_LOW:                      u32 = 0;
pub const PPB_SHPR2_PRI_8_HIGH:                     u32 = 7;

// SHPR3
pub const PPB_SHPR3_PRI_15_3_LOW:                   u32 = 29;
pub const PPB_SHPR3_PRI_15_3_HIGH:                  u32 = 31;
pub const PPB_SHPR3_PRI_14_3_LOW:                   u32 = 21;
pub const PPB_SHPR3_PRI_14_3_HIGH:                  u32 = 23;
pub const PPB_SHPR3_PRI_13_LOW:                     u32 = 8;
pub const PPB_SHPR3_PRI_13_HIGH:                    u32 = 15;
pub const PPB_SHPR3_PRI_12_3_LOW:                   u32 = 5;
pub const PPB_SHPR3_PRI_12_3_HIGH:                  u32 = 7;

// SHCSR
pub const PPB_SHCSR_HARDFAULTPENDED_BIT:            u32 = 21;
pub const PPB_SHCSR_SECUREFAULTPENDED_BIT:          u32 = 20;
pub const PPB_SHCSR_SECUREFAULTENA_BIT:             u32 = 19;
pub const PPB_SHCSR_USGFAULTENA_BIT:                u32 = 18;
pub const PPB_SHCSR_BUSFAULTENA_BIT:                u32 = 17;
pub const PPB_SHCSR_MEMFAULTENA_BIT:                u32 = 16;
pub const PPB_SHCSR_SVCALLPENDED_BIT:               u32 = 15;
pub const PPB_SHCSR_BUSFAULTPENDED_BIT:             u32 = 14;
pub const PPB_SHCSR_MEMFAULTPENDED_BIT:             u32 = 13;
pub const PPB_SHCSR_USGFAULTPENDED_BIT:             u32 = 12;
pub const PPB_SHCSR_SYSTICKACT_BIT:                 u32 = 11;
pub const PPB_SHCSR_PENDSVACT_BIT:                  u32 = 10;
pub const PPB_SHCSR_MONITORACT_BIT:                 u32 = 8;
pub const PPB_SHCSR_SVCALLACT_BIT:                  u32 = 7;
pub const PPB_SHCSR_NMIACT_BIT:                     u32 = 5;
pub const PPB_SHCSR_SECUREFAULTACT_BIT:             u32 = 4;
pub const PPB_SHCSR_USGFAULTACT_BIT:                u32 = 3;
pub const PPB_SHCSR_HARDFAULTACT_BIT:               u32 = 2;
pub const PPB_SHCSR_BUSFAULTACT_BIT:                u32 = 1;
pub const PPB_SHCSR_MEMFAULTACT_BIT:                u32 = 0;

// CFSR
pub const PPB_CFSR_UFSR_DIVBYZERO_BIT:              u32 = 25;
pub const PPB_CFSR_UFSR_UNALIGNED_BIT:              u32 = 24;
pub const PPB_CFSR_UFSR_STKOF_BIT:                  u32 = 20;
pub const PPB_CFSR_UFSR_NOCP_BIT:                   u32 = 19;
pub const PPB_CFSR_UFSR_INVPC_BIT:                  u32 = 18;
pub const PPB_CFSR_UFSR_INVSTATE_BIT:               u32 = 17;
pub const PPB_CFSR_UFSR_UNDEFINSTR_BIT:             u32 = 16;
pub const PPB_CFSR_BFSR_BFARVALID_BIT:              u32 = 15;
pub const PPB_CFSR_BFSR_LSPERR_BIT:                 u32 = 13;
pub const PPB_CFSR_BFSR_STKERR_BIT:                 u32 = 12;
pub const PPB_CFSR_BFSR_UNSTKERR_BIT:               u32 = 11;
pub const PPB_CFSR_BFSR_IMPRECISERR_BIT:            u32 = 10;
pub const PPB_CFSR_BFSR_PRECISERR_BIT:              u32 = 9;
pub const PPB_CFSR_BFSR_IBUSERR_BIT:                u32 = 8;
pub const PPB_CFSR_MMFSR_LOW:                       u32 = 0;
pub const PPB_CFSR_MMFSR_HIGH:                      u32 = 7;

// HFSR
pub const PPB_HFSR_DEBUGEVT_BIT:                    u32 = 31;
pub const PPB_HFSR_FORCED_BIT:                      u32 = 30;
pub const PPB_HFSR_VECTTBL_BIT:                     u32 = 1;

// DFSR
pub const PPB_DFSR_EXTERNAL_BIT:                    u32 = 4;
pub const PPB_DFSR_VCATCH_BIT:                      u32 = 3;
pub const PPB_DFSR_DWTTRAP_BIT:                     u32 = 2;
pub const PPB_DFSR_BKPT_BIT:                        u32 = 1;
pub const PPB_DFSR_HALTED_BIT:                      u32 = 0;

// ID_PFR0
pub const PPB_ID_PFR0_STATE1_LOW:                   u32 = 4;
pub const PPB_ID_PFR0_STATE1_HIGH:                  u32 = 7;
pub const PPB_ID_PFR0_STATE0_LOW:                   u32 = 0;
pub const PPB_ID_PFR0_STATE0_HIGH:                  u32 = 3;

// ID_PFR1
pub const PPB_ID_PFR1_MPROGMOD_LOW:                 u32 = 8;
pub const PPB_ID_PFR1_MPROGMOD_HIGH:                u32 = 11;
pub const PPB_ID_PFR1_SECURITY_LOW:                 u32 = 4;
pub const PPB_ID_PFR1_SECURITY_HIGH:                u32 = 7;

// ID_DFR0
pub const PPB_ID_DFR0_MPROFDBG_LOW:                 u32 = 20;
pub const PPB_ID_DFR0_MPROFDBG_HIGH:                u32 = 23;

// ID_AFR0
pub const PPB_ID_AFR0_IMPDEF3_LOW:                  u32 = 12;
pub const PPB_ID_AFR0_IMPDEF3_HIGH:                 u32 = 15;
pub const PPB_ID_AFR0_IMPDEF2_LOW:                  u32 = 8;
pub const PPB_ID_AFR0_IMPDEF2_HIGH:                 u32 = 11;
pub const PPB_ID_AFR0_IMPDEF1_LOW:                  u32 = 4;
pub const PPB_ID_AFR0_IMPDEF1_HIGH:                 u32 = 7;
pub const PPB_ID_AFR0_IMPDEF0_LOW:                  u32 = 0;
pub const PPB_ID_AFR0_IMPDEF0_HIGH:                 u32 = 3;

// ID_MMFR0
pub const PPB_ID_MMFR0_AUXREG_LOW:                  u32 = 20;
pub const PPB_ID_MMFR0_AUXREG_HIGH:                 u32 = 23;
pub const PPB_ID_MMFR0_TCM_LOW:                     u32 = 16;
pub const PPB_ID_MMFR0_TCM_HIGH:                    u32 = 19;
pub const PPB_ID_MMFR0_SHARELVL_LOW:                u32 = 12;
pub const PPB_ID_MMFR0_SHARELVL_HIGH:               u32 = 15;
pub const PPB_ID_MMFR0_OUTERSHR_LOW:                u32 = 8;
pub const PPB_ID_MMFR0_OUTERSHR_HIGH:               u32 = 11;
pub const PPB_ID_MMFR0_PMSA_LOW:                    u32 = 4;
pub const PPB_ID_MMFR0_PMSA_HIGH:                   u32 = 7;

// ID_MMFR2
pub const PPB_ID_MMFR2_WFISTALL_LOW:                u32 = 24;
pub const PPB_ID_MMFR2_WFISTALL_HIGH:               u32 = 27;

// ID_MMFR3
pub const PPB_ID_MMFR3_BPMAINT_LOW:                 u32 = 8;
pub const PPB_ID_MMFR3_BPMAINT_HIGH:                u32 = 11;
pub const PPB_ID_MMFR3_CMAINTSW_LOW:                u32 = 4;
pub const PPB_ID_MMFR3_CMAINTSW_HIGH:               u32 = 7;
pub const PPB_ID_MMFR3_CMAINTVA_LOW:                u32 = 0;
pub const PPB_ID_MMFR3_CMAINTVA_HIGH:               u32 = 3;

// ID_ISAR0
pub const PPB_ID_ISAR0_DIVIDE_LOW:                  u32 = 24;
pub const PPB_ID_ISAR0_DIVIDE_HIGH:                 u32 = 27;
pub const PPB_ID_ISAR0_DEBUG_LOW:                   u32 = 20;
pub const PPB_ID_ISAR0_DEBUG_HIGH:                  u32 = 23;
pub const PPB_ID_ISAR0_COPROC_LOW:                  u32 = 16;
pub const PPB_ID_ISAR0_COPROC_HIGH:                 u32 = 19;
pub const PPB_ID_ISAR0_CMPBRANCH_LOW:               u32 = 12;
pub const PPB_ID_ISAR0_CMPBRANCH_HIGH:              u32 = 15;
pub const PPB_ID_ISAR0_BITFIELD_LOW:                u32 = 8;
pub const PPB_ID_ISAR0_BITFIELD_HIGH:               u32 = 11;
pub const PPB_ID_ISAR0_BITCOUNT_LOW:                u32 = 4;
pub const PPB_ID_ISAR0_BITCOUNT_HIGH:               u32 = 7;

// ID_ISAR1
pub const PPB_ID_ISAR1_INTERWORK_LOW:               u32 = 24;
pub const PPB_ID_ISAR1_INTERWORK_HIGH:              u32 = 27;
pub const PPB_ID_ISAR1_IMMEDIATE_LOW:               u32 = 20;
pub const PPB_ID_ISAR1_IMMEDIATE_HIGH:              u32 = 23;
pub const PPB_ID_ISAR1_IFTHEN_LOW:                  u32 = 16;
pub const PPB_ID_ISAR1_IFTHEN_HIGH:                 u32 = 19;
pub const PPB_ID_ISAR1_EXTEND_LOW:                  u32 = 12;
pub const PPB_ID_ISAR1_EXTEND_HIGH:                 u32 = 15;

// ID_ISAR2
pub const PPB_ID_ISAR2_REVERSAL_LOW:                u32 = 28;
pub const PPB_ID_ISAR2_REVERSAL_HIGH:               u32 = 31;
pub const PPB_ID_ISAR2_MULTU_LOW:                   u32 = 20;
pub const PPB_ID_ISAR2_MULTU_HIGH:                  u32 = 23;
pub const PPB_ID_ISAR2_MULTS_LOW:                   u32 = 16;
pub const PPB_ID_ISAR2_MULTS_HIGH:                  u32 = 19;
pub const PPB_ID_ISAR2_MULT_LOW:                    u32 = 12;
pub const PPB_ID_ISAR2_MULT_HIGH:                   u32 = 15;
pub const PPB_ID_ISAR2_MULTIACCESSINT_LOW:          u32 = 8;
pub const PPB_ID_ISAR2_MULTIACCESSINT_HIGH:         u32 = 11;
pub const PPB_ID_ISAR2_MEMHINT_LOW:                 u32 = 4;
pub const PPB_ID_ISAR2_MEMHINT_HIGH:                u32 = 7;
pub const PPB_ID_ISAR2_LOADSTORE_LOW:               u32 = 0;
pub const PPB_ID_ISAR2_LOADSTORE_HIGH:              u32 = 3;

// ID_ISAR3
pub const PPB_ID_ISAR3_TRUENOP_LOW:                 u32 = 24;
pub const PPB_ID_ISAR3_TRUENOP_HIGH:                u32 = 27;
pub const PPB_ID_ISAR3_T32COPY_LOW:                 u32 = 20;
pub const PPB_ID_ISAR3_T32COPY_HIGH:                u32 = 23;
pub const PPB_ID_ISAR3_TABBRANCH_LOW:               u32 = 16;
pub const PPB_ID_ISAR3_TABBRANCH_HIGH:              u32 = 19;
pub const PPB_ID_ISAR3_SYNCHPRIM_LOW:               u32 = 12;
pub const PPB_ID_ISAR3_SYNCHPRIM_HIGH:              u32 = 15;
pub const PPB_ID_ISAR3_SVC_LOW:                     u32 = 8;
pub const PPB_ID_ISAR3_SVC_HIGH:                    u32 = 11;
pub const PPB_ID_ISAR3_SIMD_LOW:                    u32 = 4;
pub const PPB_ID_ISAR3_SIMD_HIGH:                   u32 = 7;
pub const PPB_ID_ISAR3_SATURATE_LOW:                u32 = 0;
pub const PPB_ID_ISAR3_SATURATE_HIGH:               u32 = 3;

// ID_ISAR4
pub const PPB_ID_ISAR4_PSR_M_LOW:                   u32 = 24;
pub const PPB_ID_ISAR4_PSR_M_HIGH:                  u32 = 27;
pub const PPB_ID_ISAR4_SYNCPRIM_FRAC_LOW:           u32 = 20;
pub const PPB_ID_ISAR4_SYNCPRIM_FRAC_HIGH:          u32 = 23;
pub const PPB_ID_ISAR4_BARRIER_LOW:                 u32 = 16;
pub const PPB_ID_ISAR4_BARRIER_HIGH:                u32 = 19;
pub const PPB_ID_ISAR4_WRITEBACK_LOW:               u32 = 8;
pub const PPB_ID_ISAR4_WRITEBACK_HIGH:              u32 = 11;
pub const PPB_ID_ISAR4_WITHSHIFTS_LOW:              u32 = 4;
pub const PPB_ID_ISAR4_WITHSHIFTS_HIGH:             u32 = 7;
pub const PPB_ID_ISAR4_UNPRIV_LOW:                  u32 = 0;
pub const PPB_ID_ISAR4_UNPRIV_HIGH:                 u32 = 3;

// CTR
pub const PPB_CTR_RES1_BIT:                         u32 = 31;
pub const PPB_CTR_RES1_1_LOW:                       u32 = 14;
pub const PPB_CTR_RES1_1_HIGH:                      u32 = 15;
pub const PPB_CTR_CWG_LOW:                          u32 = 24;
pub const PPB_CTR_CWG_HIGH:                         u32 = 27;
pub const PPB_CTR_ERG_LOW:                          u32 = 20;
pub const PPB_CTR_ERG_HIGH:                         u32 = 23;
pub const PPB_CTR_DMINLINE_LOW:                     u32 = 16;
pub const PPB_CTR_DMINLINE_HIGH:                    u32 = 19;
pub const PPB_CTR_IMINLINE_LOW:                     u32 = 0;
pub const PPB_CTR_IMINLINE_HIGH:                    u32 = 3;

// CPACR
pub const PPB_CPACR_CP11_LOW:                       u32 = 22;
pub const PPB_CPACR_CP11_HIGH:                      u32 = 23;
pub const PPB_CPACR_CP10_LOW:                       u32 = 20;
pub const PPB_CPACR_CP10_HIGH:                      u32 = 21;
pub const PPB_CPACR_CP7_LOW:                        u32 = 14;
pub const PPB_CPACR_CP7_HIGH:                       u32 = 15;
pub const PPB_CPACR_CP6_LOW:                        u32 = 12;
pub const PPB_CPACR_CP6_HIGH:                       u32 = 13;
pub const PPB_CPACR_CP5_LOW:                        u32 = 10;
pub const PPB_CPACR_CP5_HIGH:                       u32 = 11;
pub const PPB_CPACR_CP4_LOW:                        u32 = 8;
pub const PPB_CPACR_CP4_HIGH:                       u32 = 9;
pub const PPB_CPACR_CP3_LOW:                        u32 = 6;
pub const PPB_CPACR_CP3_HIGH:                       u32 = 7;
pub const PPB_CPACR_CP2_LOW:                        u32 = 4;
pub const PPB_CPACR_CP2_HIGH:                       u32 = 5;
pub const PPB_CPACR_CP1_LOW:                        u32 = 2;
pub const PPB_CPACR_CP1_HIGH:                       u32 = 3;
pub const PPB_CPACR_CP0_LOW:                        u32 = 0;
pub const PPB_CPACR_CP0_HIGH:                       u32 = 1;

// NSACR
pub const PPB_NSACR_CP11_BIT:                       u32 = 11;
pub const PPB_NSACR_CP10_BIT:                       u32 = 10;
pub const PPB_NSACR_CP7_BIT:                        u32 = 7;
pub const PPB_NSACR_CP6_BIT:                        u32 = 6;
pub const PPB_NSACR_CP5_BIT:                        u32 = 5;
pub const PPB_NSACR_CP4_BIT:                        u32 = 4;
pub const PPB_NSACR_CP3_BIT:                        u32 = 3;
pub const PPB_NSACR_CP2_BIT:                        u32 = 2;
pub const PPB_NSACR_CP1_BIT:                        u32 = 1;
pub const PPB_NSACR_CP0_BIT:                        u32 = 0;

// MPU_TYPE
pub const PPB_MPU_TYPE_DREGION_LOW:                 u32 = 8;
pub const PPB_MPU_TYPE_DREGION_HIGH:                u32 = 15;
pub const PPB_MPU_TYPE_SEPARATE_BIT:                u32 = 0;

// MPU_CTRL
pub const PPB_MPU_CTRL_PRIVDEFENA_BIT:              u32 = 2;
pub const PPB_MPU_CTRL_HFNMIENA_BIT:                u32 = 1;
pub const PPB_MPU_CTRL_ENABLE_BIT:                  u32 = 0;

// MPU_RNR
pub const PPB_MPU_RNR_REGION_LOW:                   u32 = 0;
pub const PPB_MPU_RNR_REGION_HIGH:                  u32 = 2;

// MPU_RBAR, MPU_RBAR_A1, MPU_RBAR_A2, MPU_RBAR_A3
pub const PPB_MPU_RBAR_BASE_LOW:                    u32 = 5;
pub const PPB_MPU_RBAR_BASE_HIGH:                   u32 = 31;
pub const PPB_MPU_RBAR_SH_LOW:                      u32 = 3;
pub const PPB_MPU_RBAR_SH_HIGH:                     u32 = 4;
pub const PPB_MPU_RBAR_AP_LOW:                      u32 = 1;
pub const PPB_MPU_RBAR_AP_HIGH:                     u32 = 2;
pub const PPB_MPU_RBAR_XN_BIT:                      u32 = 0;

// MPU_RLAR, MPU_RLAR_A1, MPU_RLAR_A2, MPU_RLAR_A3
pub const PPB_MPU_RLAR_LIMIT_LOW:                   u32 = 5;
pub const PPB_MPU_RLAR_LIMIT_HIGH:                  u32 = 31;
pub const PPB_MPU_RLAR_ATTRINDX_LOW:                u32 = 1;
pub const PPB_MPU_RLAR_ATTRINDX_HIGH:               u32 = 3;
pub const PPB_MPU_RLAR_EN_BIT:                      u32 = 0;

// MPU_MAIR0, MPU_MAIR1
pub const PPB_MPU_MAIR_ATTR3_LOW:                   u32 = 24;
pub const PPB_MPU_MAIR_ATTR3_HIGH:                  u32 = 31;
pub const PPB_MPU_MAIR_ATTR2_LOW:                   u32 = 16;
pub const PPB_MPU_MAIR_ATTR2_HIGH:                  u32 = 23;
pub const PPB_MPU_MAIR_ATTR1_LOW:                   u32 = 8;
pub const PPB_MPU_MAIR_ATTR1_HIGH:                  u32 = 15;
pub const PPB_MPU_MAIR_ATTR0_LOW:                   u32 = 0;
pub const PPB_MPU_MAIR_ATTR0_HIGH:                  u32 = 7;
pub const PPB_MPU_MAIR_ATTR7_LOW:                   u32 = 24;
pub const PPB_MPU_MAIR_ATTR7_HIGH:                  u32 = 31;
pub const PPB_MPU_MAIR_ATTR6_LOW:                   u32 = 16;
pub const PPB_MPU_MAIR_ATTR6_HIGH:                  u32 = 23;
pub const PPB_MPU_MAIR_ATTR5_LOW:                   u32 = 8;
pub const PPB_MPU_MAIR_ATTR5_HIGH:                  u32 = 15;
pub const PPB_MPU_MAIR_ATTR4_LOW:                   u32 = 0;
pub const PPB_MPU_MAIR_ATTR4_HIGH:                  u32 = 7;

// SAU_CTRL
pub const PPB_SAU_CTRL_ALLNS_BIT:                   u32 = 1;
pub const PPB_SAU_CTRL_ENABLE_BIT:                  u32 = 0;

// SAU_TYPE
pub const PPB_SAU_TYPE_SREGION_LOW:                 u32 = 0;
pub const PPB_SAU_TYPE_SREGION_HIGH:                u32 = 7;

// SAU_RNR
pub const PPB_SAU_RNR_REGION_LOW:                   u32 = 0;
pub const PPB_SAU_RNR_REGION_HIGH:                  u32 = 7;

// SAU_RBAR
pub const PPB_SAU_RBAR_BADDR_LOW:                   u32 = 5;
pub const PPB_SAU_RBAR_BADDR_HIGH:                  u32 = 31;

// SAU_RLAR
pub const PPB_SAU_RLAR_LADDR_LOW:                   u32 = 5;
pub const PPB_SAU_RLAR_LADDR_HIGH:                  u32 = 31;
pub const PPB_SAU_RLAR_NSC_BIT:                     u32 = 1;
pub const PPB_SAU_RLAR_ENABLE_BIT:                  u32 = 0;

// SFSR
pub const PPB_SFSR_LSERR_BIT:                       u32 = 7;
pub const PPB_SFSR_SFARVALID_BIT:                   u32 = 6;
pub const PPB_SFSR_LSPERR_BIT:                      u32 = 5;
pub const PPB_SFSR_INVTRAN_BIT:                     u32 = 4;
pub const PPB_SFSR_AUVIOL_BIT:                      u32 = 3;
pub const PPB_SFSR_INVER_BIT:                       u32 = 2;
pub const PPB_SFSR_INVIS_BIT:                       u32 = 1;
pub const PPB_SFSR_INVEP_BIT:                       u32 = 0;

// DHCSR
pub const PPB_DHCSR_S_RESTART_ST_BIT:               u32 = 26;
pub const PPB_DHCSR_S_RESET_ST_BIT:                 u32 = 25;
pub const PPB_DHCSR_S_RETIRE_ST_BIT:                u32 = 24;
pub const PPB_DHCSR_S_SDE_BIT:                      u32 = 20;
pub const PPB_DHCSR_S_LOCKUP_BIT:                   u32 = 19;
pub const PPB_DHCSR_S_SLEEP_BIT:                    u32 = 18;
pub const PPB_DHCSR_S_HALT_BIT:                     u32 = 17;
pub const PPB_DHCSR_S_REGRDY_BIT:                   u32 = 16;
pub const PPB_DHCSR_C_SNAPSTALL_BIT:                u32 = 5;
pub const PPB_DHCSR_C_MASKINTS_BIT:                 u32 = 3;
pub const PPB_DHCSR_C_STEP_BIT:                     u32 = 2;
pub const PPB_DHCSR_C_HALT_BIT:                     u32 = 1;
pub const PPB_DHCSR_C_DEBUGEN_BIT:                  u32 = 0;

// DCRSR
pub const PPB_DCRSR_REGWNR_BIT:                     u32 = 16;
pub const PPB_DCRSR_REGSEL_LOW:                     u32 = 0;
pub const PPB_DCRSR_REGSEL_HIGH:                    u32 = 6;

// DEMCR
pub const PPB_DEMCR_TRCENA_BIT:                     u32 = 24;
pub const PPB_DEMCR_SDME_BIT:                       u32 = 20;
pub const PPB_DEMCR_MON_REQ_BIT:                    u32 = 19;
pub const PPB_DEMCR_MON_STEP_BIT:                   u32 = 18;
pub const PPB_DEMCR_MON_PEND_BIT:                   u32 = 17;
pub const PPB_DEMCR_MON_EN_BIT:                     u32 = 16;
pub const PPB_DEMCR_VC_SFERR_BIT:                   u32 = 11;
pub const PPB_DEMCR_VC_HARDERR_BIT:                 u32 = 10;
pub const PPB_DEMCR_VC_INTERR_BIT:                  u32 = 9;
pub const PPB_DEMCR_VC_BUSERR_BIT:                  u32 = 8;
pub const PPB_DEMCR_VC_STATERR_BIT:                 u32 = 7;
pub const PPB_DEMCR_VC_CHKERR_BIT:                  u32 = 6;
pub const PPB_DEMCR_VC_NOCPERR_BIT:                 u32 = 5;
pub const PPB_DEMCR_VC_MMERR_BIT:                   u32 = 4;
pub const PPB_DEMCR_VC_CORERESET_BIT:               u32 = 0;

// DSCSR
pub const PPB_DSCSR_CDSKEY_BIT:                     u32 = 17;
pub const PPB_DSCSR_CDS_BIT:                        u32 = 16;
pub const PPB_DSCSR_SBRSEL_BIT:                     u32 = 1;
pub const PPB_DSCSR_SBRSELEN_BIT:                   u32 = 0;

// STIR
pub const PPB_STIR_INTID_LOW:                       u32 = 0;
pub const PPB_STIR_INTID_HIGH:                      u32 = 8;

// FPCCR
pub const PPB_FPCCR_ASPEN_BIT:                      u32 = 31;
pub const PPB_FPCCR_LSPEN_BIT:                      u32 = 30;
pub const PPB_FPCCR_LSPENS_BIT:                     u32 = 29;
pub const PPB_FPCCR_CLRONRET_BIT:                   u32 = 28;
pub const PPB_FPCCR_CLRONRETS_BIT:                  u32 = 27;
pub const PPB_FPCCR_TS_BIT:                         u32 = 26;
pub const PPB_FPCCR_UFRDY_BIT:                      u32 = 10;
pub const PPB_FPCCR_SPLIMVIOL_BIT:                  u32 = 9;
pub const PPB_FPCCR_MONRDY_BIT:                     u32 = 8;
pub const PPB_FPCCR_SFRDY_BIT:                      u32 = 7;
pub const PPB_FPCCR_BFRDY_BIT:                      u32 = 6;
pub const PPB_FPCCR_MMRDY_BIT:                      u32 = 5;
pub const PPB_FPCCR_HFRDY_BIT:                      u32 = 4;
pub const PPB_FPCCR_THREAD_BIT:                     u32 = 3;
pub const PPB_FPCCR_S_BIT:                          u32 = 2;
pub const PPB_FPCCR_USER_BIT:                       u32 = 1;
pub const PPB_FPCCR_LSPACT_BIT:                     u32 = 0;

// FPCAR
pub const PPB_FPCAR_ADDRESS_LOW:                    u32 = 3;
pub const PPB_FPCAR_ADDRESS_HIGH:                   u32 = 31;

// FPDSCR
pub const PPB_FPDSCR_AHP_BIT:                       u32 = 26;
pub const PPB_FPDSCR_DN_BIT:                        u32 = 25;
pub const PPB_FPDSCR_FZ_BIT:                        u32 = 24;
pub const PPB_FPDSCR_RMODE_LOW:                     u32 = 22;
pub const PPB_FPDSCR_RMODE_HIGH:                    u32 = 23;

// MVFR0
pub const PPB_MVFR0_FPROUND_LOW:                    u32 = 28;
pub const PPB_MVFR0_FPROUND_HIGH:                   u32 = 31;
pub const PPB_MVFR0_FPSQRT_LOW:                     u32 = 20;
pub const PPB_MVFR0_FPSQRT_HIGH:                    u32 = 23;
pub const PPB_MVFR0_FPDIVIDE_LOW:                   u32 = 16;
pub const PPB_MVFR0_FPDIVIDE_HIGH:                  u32 = 19;
pub const PPB_MVFR0_FPDP_LOW:                       u32 = 8;
pub const PPB_MVFR0_FPDP_HIGH:                      u32 = 11;
pub const PPB_MVFR0_FPSP_LOW:                       u32 = 4;
pub const PPB_MVFR0_FPSP_HIGH:                      u32 = 7;
pub const PPB_MVFR0_SIMDREG_LOW:                    u32 = 0;
pub const PPB_MVFR0_SIMDREG_HIGH:                   u32 = 3;

// MVFR1
pub const PPB_MVFR1_FMAC_LOW:                       u32 = 28;
pub const PPB_MVFR1_FMAC_HIGH:                      u32 = 31;
pub const PPB_MVFR1_FPHP_LOW:                       u32 = 24;
pub const PPB_MVFR1_FPHP_HIGH:                      u32 = 27;
pub const PPB_MVFR1_FPDNAN_LOW:                     u32 = 4;
pub const PPB_MVFR1_FPDNAN_HIGH:                    u32 = 7;
pub const PPB_MVFR1_FPFTZ_LOW:                      u32 = 0;
pub const PPB_MVFR1_FPFTZ_HIGH:                     u32 = 3;

// MVFR2
pub const PPB_MVFR2_FPMISC_LOW:                     u32 = 4;
pub const PPB_MVFR2_FPMISC_HIGH:                    u32 = 7;

// TRCPRGCTLR
pub const PPB_TRCPRGCTLR_EN_BIT:                    u32 = 0;

// TRCSTATR
pub const PPB_TRCSTATR_PMSTABLE_BIT:                u32 = 1;
pub const PPB_TRCSTATR_IDLE_BIT:                    u32 = 0;

// TRCCONFIGR
pub const PPB_TRCCONFIGR_RS_BIT:                    u32 = 12;
pub const PPB_TRCCONFIGR_TS_BIT:                    u32 = 11;
pub const PPB_TRCCONFIGR_COND_LOW:                  u32 = 5;
pub const PPB_TRCCONFIGR_COND_HIGH:                 u32 = 10;
pub const PPB_TRCCONFIGR_CCI_BIT:                   u32 = 4;
pub const PPB_TRCCONFIGR_BB_BIT:                    u32 = 3;

// TRCEVENTCTL0R
pub const PPB_TRCEVENTCTL0R_TYPE1_BIT:              u32 = 15;
pub const PPB_TRCEVENTCTL0R_TYPE0_BIT:              u32 = 7;
pub const PPB_TRCEVENTCTL0R_SEL1_LOW:               u32 = 8;
pub const PPB_TRCEVENTCTL0R_SEL1_HIGH:              u32 = 10;
pub const PPB_TRCEVENTCTL0R_SEL0_LOW:               u32 = 0;
pub const PPB_TRCEVENTCTL0R_SEL0_HIGH:              u32 = 2;

// TRCEVENTCTL1R
pub const PPB_TRCEVENTCTL1R_LPOVERRIDE_BIT:         u32 = 12;
pub const PPB_TRCEVENTCTL1R_ATB_BIT:                u32 = 11;
pub const PPB_TRCEVENTCTL1R_INSTEN1_BIT:            u32 = 1;
pub const PPB_TRCEVENTCTL1R_INSTEN0_BIT:            u32 = 0;

// TRCSTALLCTLR
pub const PPB_TRCSTALLCTLR_INSTPRIORITY_BIT:        u32 = 10;
pub const PPB_TRCSTALLCTLR_ISTALL_BIT:              u32 = 8;
pub const PPB_TRCSTALLCTLR_LEVEL_LOW:               u32 = 2;
pub const PPB_TRCSTALLCTLR_LEVEL_HIGH:              u32 = 3;

// TRCTSCTLR
pub const PPB_TRCTSCTLR_TYPE0_BIT:                  u32 = 7;
pub const PPB_TRCTSCTLR_SEL0_LOW:                   u32 = 0;
pub const PPB_TRCTSCTLR_SEL0_HIGH:                  u32 = 1;

// TRCSYNCPR
pub const PPB_TRCSYNCPR_PERIOD_LOW:                 u32 = 0;
pub const PPB_TRCSYNCPR_PERIOD_HIGH:                u32 = 4;

// TRCCCCTLR
pub const PPB_TRCCCCTLR_THRESHOLD_LOW:              u32 = 0;
pub const PPB_TRCCCCTLR_THRESHOLD_HIGH:             u32 = 11;

// TRCVICTLR
pub const PPB_TRCVICTLR_EXLEVEL_S3_BIT:             u32 = 19;
pub const PPB_TRCVICTLR_EXLEVEL_S0_BIT:             u32 = 16;
pub const PPB_TRCVICTLR_TRCERR_BIT:                 u32 = 11;
pub const PPB_TRCVICTLR_TRCRESET_BIT:               u32 = 10;
pub const PPB_TRCVICTLR_SSSTATUS_BIT:               u32 = 9;
pub const PPB_TRCVICTLR_TYPE0_BIT:                  u32 = 7;
pub const PPB_TRCVICTLR_SEL0_LOW:                   u32 = 0;
pub const PPB_TRCVICTLR_SEL0_HIGH:                  u32 = 1;

// TRCCNTRLDVR0
pub const PPB_TRCCNTRLDVR0_VALUE_LOW:               u32 = 0;
pub const PPB_TRCCNTRLDVR0_VALUE_HIGH:              u32 = 15;

// TRCIMSPEC
pub const PPB_TRCIMSPEC_SUPPORT_LOW:                u32 = 0;
pub const PPB_TRCIMSPEC_SUPPORT_HIGH:               u32 = 3;

// TRCIDR0
pub const PPB_TRCIDR0_COMMOPT_BIT:                  u32 = 29;
pub const PPB_TRCIDR0_TSSIZE_LOW:                   u32 = 24;
pub const PPB_TRCIDR0_TSSIZE_HIGH:                  u32 = 28;
pub const PPB_TRCIDR0_TRCEXDATA_BIT:                u32 = 17;
pub const PPB_TRCIDR0_QSUPP_LOW:                    u32 = 15;
pub const PPB_TRCIDR0_QSUPP_HIGH:                   u32 = 16;
pub const PPB_TRCIDR0_QFILT_BIT:                    u32 = 14;
pub const PPB_TRCIDR0_CONDTYPE_LOW:                 u32 = 12;
pub const PPB_TRCIDR0_CONDTYPE_HIGH:                u32 = 13;
pub const PPB_TRCIDR0_NUMEVENT_LOW:                 u32 = 10;
pub const PPB_TRCIDR0_NUMEVENT_HIGH:                u32 = 11;
pub const PPB_TRCIDR0_RETSTACK_BIT:                 u32 = 9;
pub const PPB_TRCIDR0_TRCCCI_BIT:                   u32 = 7;
pub const PPB_TRCIDR0_TRCCOND_BIT:                  u32 = 6;
pub const PPB_TRCIDR0_TRCBB_BIT:                    u32 = 5;
pub const PPB_TRCIDR0_TRCDATA_LOW:                  u32 = 3;
pub const PPB_TRCIDR0_TRCDATA_HIGH:                 u32 = 4;
pub const PPB_TRCIDR0_INSTP0_LOW:                   u32 = 1;
pub const PPB_TRCIDR0_INSTP0_HIGH:                  u32 = 2;
pub const PPB_TRCIDR0_RES1_BIT:                     u32 = 0;

// TRCIDR1
pub const PPB_TRCIDR1_DESIGNER_LOW:                 u32 = 24;
pub const PPB_TRCIDR1_DESIGNER_HIGH:                u32 = 31;
pub const PPB_TRCIDR1_RES1_LOW:                     u32 = 12;
pub const PPB_TRCIDR1_RES1_HIGH:                    u32 = 15;
pub const PPB_TRCIDR1_TRCARCHMAJ_LOW:               u32 = 8;
pub const PPB_TRCIDR1_TRCARCHMAJ_HIGH:              u32 = 11;
pub const PPB_TRCIDR1_TRCARCHMIN_LOW:               u32 = 4;
pub const PPB_TRCIDR1_TRCARCHMIN_HIGH:              u32 = 7;
pub const PPB_TRCIDR1_REVISION_LOW:                 u32 = 0;
pub const PPB_TRCIDR1_REVISION_HIGH:                u32 = 3;

// TRCIDR2
pub const PPB_TRCIDR2_CCSIZE_LOW:                   u32 = 25;
pub const PPB_TRCIDR2_CCSIZE_HIGH:                  u32 = 28;
pub const PPB_TRCIDR2_DVSIZE_LOW:                   u32 = 20;
pub const PPB_TRCIDR2_DVSIZE_HIGH:                  u32 = 24;
pub const PPB_TRCIDR2_DASIZE_LOW:                   u32 = 15;
pub const PPB_TRCIDR2_DASIZE_HIGH:                  u32 = 19;
pub const PPB_TRCIDR2_VMIDSIZE_LOW:                 u32 = 10;
pub const PPB_TRCIDR2_VMIDSIZE_HIGH:                u32 = 14;
pub const PPB_TRCIDR2_CIDSIZE_LOW:                  u32 = 5;
pub const PPB_TRCIDR2_CIDSIZE_HIGH:                 u32 = 9;
pub const PPB_TRCIDR2_IASIZE_LOW:                   u32 = 0;
pub const PPB_TRCIDR2_IASIZE_HIGH:                  u32 = 4;

// TRCIDR3
pub const PPB_TRCIDR3_NOOVERFLOW_BIT:               u32 = 31;
pub const PPB_TRCIDR3_NUMPROC_LOW:                  u32 = 28;
pub const PPB_TRCIDR3_NUMPROC_HIGH:                 u32 = 30;
pub const PPB_TRCIDR3_SYSSTALL_BIT:                 u32 = 27;
pub const PPB_TRCIDR3_STALLCTL_BIT:                 u32 = 26;
pub const PPB_TRCIDR3_SYNCPR_BIT:                   u32 = 25;
pub const PPB_TRCIDR3_TRCERR_BIT:                   u32 = 24;
pub const PPB_TRCIDR3_EXLEVEL_NS_LOW:               u32 = 20;
pub const PPB_TRCIDR3_EXLEVEL_NS_HIGH:              u32 = 23;
pub const PPB_TRCIDR3_EXLEVEL_S_LOW:                u32 = 16;
pub const PPB_TRCIDR3_EXLEVEL_S_HIGH:               u32 = 19;
pub const PPB_TRCIDR3_CCITMIN_LOW:                  u32 = 0;
pub const PPB_TRCIDR3_CCITMIN_HIGH:                 u32 = 11;

// TRCIDR4
pub const PPB_TRCIDR4_NUMVMIDC_LOW:                 u32 = 28;
pub const PPB_TRCIDR4_NUMVMIDC_HIGH:                u32 = 31;
pub const PPB_TRCIDR4_NUMCIDC_LOW:                  u32 = 24;
pub const PPB_TRCIDR4_NUMCIDC_HIGH:                 u32 = 27;
pub const PPB_TRCIDR4_NUMSSCC_LOW:                  u32 = 20;
pub const PPB_TRCIDR4_NUMSSCC_HIGH:                 u32 = 23;
pub const PPB_TRCIDR4_NUMRSPAIR_LOW:                u32 = 16;
pub const PPB_TRCIDR4_NUMRSPAIR_HIGH:               u32 = 19;
pub const PPB_TRCIDR4_NUMPC_LOW:                    u32 = 12;
pub const PPB_TRCIDR4_NUMPC_HIGH:                   u32 = 15;
pub const PPB_TRCIDR4_SUPPDAC_BIT:                  u32 = 8;
pub const PPB_TRCIDR4_NUMDVC_LOW:                   u32 = 4;
pub const PPB_TRCIDR4_NUMDVC_HIGH:                  u32 = 7;
pub const PPB_TRCIDR4_NUMACPAIRS_LOW:               u32 = 0;
pub const PPB_TRCIDR4_NUMACPAIRS_HIGH:              u32 = 3;

// TRCIDR5
pub const PPB_TRCIDR5_REDFUNCNTR_BIT:               u32 = 31;
pub const PPB_TRCIDR5_NUMCNTR_LOW:                  u32 = 28;
pub const PPB_TRCIDR5_NUMCNTR_HIGH:                 u32 = 30;
pub const PPB_TRCIDR5_NUMSEQSTATE_LOW:              u32 = 25;
pub const PPB_TRCIDR5_NUMSEQSTATE_HIGH:             u32 = 27;
pub const PPB_TRCIDR5_LPOVERRIDE_BIT:               u32 = 23;
pub const PPB_TRCIDR5_ATBTRIG_BIT:                  u32 = 22;
pub const PPB_TRCIDR5_TRACEIDSIZE_LOW:              u32 = 16;
pub const PPB_TRCIDR5_TRACEIDSIZE_HIGH:             u32 = 21;
pub const PPB_TRCIDR5_NUMEXTINSEL_LOW:              u32 = 9;
pub const PPB_TRCIDR5_NUMEXTINSEL_HIGH:             u32 = 11;
pub const PPB_TRCIDR5_NUMEXTIN_LOW:                 u32 = 0;
pub const PPB_TRCIDR5_NUMEXTIN_HIGH:                u32 = 8;

// TRCRSCTLR2, TRCRSCTLR3
pub const PPB_TRCRSCTLR_PAIRINV_BIT:                u32 = 21;
pub const PPB_TRCRSCTLR_INV_BIT:                    u32 = 20;
pub const PPB_TRCRSCTLR_GROUP_LOW:                  u32 = 16;
pub const PPB_TRCRSCTLR_GROUP_HIGH:                 u32 = 18;
pub const PPB_TRCRSCTLR_SELECT_LOW:                 u32 = 0;
pub const PPB_TRCRSCTLR_SELECT_HIGH:                u32 = 7;

// TRCSSCSR
pub const PPB_TRCSSCSR_STATUS_BIT:                  u32 = 31;
pub const PPB_TRCSSCSR_PC_BIT:                      u32 = 3;
pub const PPB_TRCSSCSR_DV_BIT:                      u32 = 2;
pub const PPB_TRCSSCSR_DA_BIT:                      u32 = 1;
pub const PPB_TRCSSCSR_INST_BIT:                    u32 = 0;

// TRCSSPCICR
pub const PPB_TRCSSPCICR_PC_LOW:                    u32 = 0;
pub const PPB_TRCSSPCICR_PC_HIGH:                   u32 = 3;

// TRCPDCR
pub const PPB_TRCPDCR_PU_BIT:                       u32 = 3;

// TRCPDSR
pub const PPB_TRCPDSR_OSLK_BIT:                     u32 = 5;
pub const PPB_TRCPDSR_STICKYPD_BIT:                 u32 = 1;
pub const PPB_TRCPDSR_POWER_BIT:                    u32 = 0;

// TRCITATBIDR
pub const PPB_TRCITATBIDR_ID_LOW:                   u32 = 0;
pub const PPB_TRCITATBIDR_ID_HIGH:                  u32 = 6;

// TRCITIATBINR
pub const PPB_TRCITIATBINR_AFVALIDM_BIT:            u32 = 1;
pub const PPB_TRCITIATBINR_ATREADYM_BIT:            u32 = 0;

// TRCITIATBOUTR
pub const PPB_TRCITIATBOUTR_AFREADY_BIT:            u32 = 1;
pub const PPB_TRCITIATBOUTR_ATVALID_BIT:            u32 = 0;

// TRCCLAIMSET
pub const PPB_TRCCLAIMSET_SET3_BIT:                 u32 = 3;
pub const PPB_TRCCLAIMSET_SET2_BIT:                 u32 = 2;
pub const PPB_TRCCLAIMSET_SET1_BIT:                 u32 = 1;
pub const PPB_TRCCLAIMSET_SET0_BIT:                 u32 = 0;

// TRCCLAIMCLR
pub const PPB_TRCCLAIMCLR_CLR3_BIT:                 u32 = 3;
pub const PPB_TRCCLAIMCLR_CLR2_BIT:                 u32 = 2;
pub const PPB_TRCCLAIMCLR_CLR1_BIT:                 u32 = 1;
pub const PPB_TRCCLAIMCLR_CLR0_BIT:                 u32 = 0;

// TRCAUTHSTATUS
pub const PPB_TRCAUTHSTATUS_SNID_LOW:               u32 = 6;
pub const PPB_TRCAUTHSTATUS_SNID_HIGH:              u32 = 7;
pub const PPB_TRCAUTHSTATUS_SID_LOW:                u32 = 4;
pub const PPB_TRCAUTHSTATUS_SID_HIGH:               u32 = 5;
pub const PPB_TRCAUTHSTATUS_NSNID_LOW:              u32 = 2;
pub const PPB_TRCAUTHSTATUS_NSNID_HIGH:             u32 = 3;
pub const PPB_TRCAUTHSTATUS_NSID_LOW:               u32 = 0;
pub const PPB_TRCAUTHSTATUS_NSID_HIGH:              u32 = 1;

// TRCDEVARCH
pub const PPB_TRCDEVARCH_ARCHITECT_LOW:             u32 = 21;
pub const PPB_TRCDEVARCH_ARCHITECT_HIGH:            u32 = 31;
pub const PPB_TRCDEVARCH_PRESENT_BIT:               u32 = 20;
pub const PPB_TRCDEVARCH_REVISION_LOW:              u32 = 16;
pub const PPB_TRCDEVARCH_REVISION_HIGH:             u32 = 19;
pub const PPB_TRCDEVARCH_ARCHID_LOW:                u32 = 0;
pub const PPB_TRCDEVARCH_ARCHID_HIGH:               u32 = 15;

// DEVARCH
pub const PPB_DEVARCH_ARCHITECT_LOW:                u32 = 21;
pub const PPB_DEVARCH_ARCHITECT_HIGH:               u32 = 31;
pub const PPB_DEVARCH_PRESENT_BIT:                  u32 = 20;
pub const PPB_DEVARCH_REVISION_LOW:                 u32 = 16;
pub const PPB_DEVARCH_REVISION_HIGH:                u32 = 19;
pub const PPB_DEVARCH_ARCHID_LOW:                   u32 = 0;
pub const PPB_DEVARCH_ARCHID_HIGH:                  u32 = 15;

// CTICONTROL
pub const PPB_CTICONTROL_GLBEN_BIT:                 u32 = 0;

// CTIINTACK
pub const PPB_CTIINTACK_INTACK_LOW:                 u32 = 0;
pub const PPB_CTIINTACK_INTACK_HIGH:                u32 = 7;

// CTIAPPSET
pub const PPB_CTIAPPSET_APPSET_LOW:                 u32 = 0;
pub const PPB_CTIAPPSET_APPSET_HIGH:                u32 = 3;

// CTIAPPCLEAR
pub const PPB_CTIAPPCLEAR_APPCLEAR_LOW:             u32 = 0;
pub const PPB_CTIAPPCLEAR_APPCLEAR_HIGH:            u32 = 3;

// CTIAPPPULSE
pub const PPB_CTIAPPPULSE_APPULSE_LOW:              u32 = 0;
pub const PPB_CTIAPPPULSE_APPULSE_HIGH:             u32 = 3;

// CTIINEN0, CTIINEN1, CTIINEN2, CTIINEN3, CTIINEN4, CTIINEN5, CTIINEN6, CTIINEN7
pub const PPB_CTIINEN_TRIGINEN_LOW:                 u32 = 0;
pub const PPB_CTIINEN_TRIGINEN_HIGH:                u32 = 3;

// CTIOUTEN0, CTIOUTEN1, CTIOUTEN2, CTIOUTEN3, CTIOUTEN4, CTIOUTEN5, CTIOUTEN6, CTIOUTEN7
pub const PPB_CTIOUTEN_TRIGOUTEN_LOW:               u32 = 0;
pub const PPB_CTIOUTEN_TRIGOUTEN_HIGH:              u32 = 3;

// CTITRIGINSTATUS
pub const PPB_CTITRIGINSTATUS_TRIGINSTATUS_LOW:     u32 = 0;
pub const PPB_CTITRIGINSTATUS_TRIGINSTATUS_HIGH:    u32 = 7;

// CTITRIGOUTSTATUS
pub const PPB_CTITRIGOUTSTATUS_TRIGOUTSTATUS_LOW:   u32 = 0;
pub const PPB_CTITRIGOUTSTATUS_TRIGOUTSTATUS_HIGH:  u32 = 7;

// CTICHINSTATUS
pub const PPB_CTICHINSTATUS_CTICHOUTSTATUS_LOW:     u32 = 0;
pub const PPB_CTICHINSTATUS_CTICHOUTSTATUS_HIGH:    u32 = 3;

// CTIGATE
pub const PPB_CTIGATE_CTIGATEEN3_BIT:               u32 = 3;
pub const PPB_CTIGATE_CTIGATEEN2_BIT:               u32 = 2;
pub const PPB_CTIGATE_CTIGATEEN1_BIT:               u32 = 1;
pub const PPB_CTIGATE_CTIGATEEN0_BIT:               u32 = 0;

// ITCHOUT
pub const PPB_ITCHOUT_CTCHOUT_LOW:                  u32 = 0;
pub const PPB_ITCHOUT_CTCHOUT_HIGH:                 u32 = 3;

// ITTRIGOUT
pub const PPB_ITTRIGOUT_CTTRIGOUT_LOW:              u32 = 0;
pub const PPB_ITTRIGOUT_CTTRIGOUT_HIGH:             u32 = 7;

// ITCHIN
pub const PPB_ITCHIN_CTCHIN_LOW:                    u32 = 0;
pub const PPB_ITCHIN_CTCHIN_HIGH:                   u32 = 3;

// DEVID
pub const PPB_DEVID_NUMCH_LOW:                      u32 = 16;
pub const PPB_DEVID_NUMCH_HIGH:                     u32 = 19;
pub const PPB_DEVID_NUMTRIG_LOW:                    u32 = 8;
pub const PPB_DEVID_NUMTRIG_HIGH:                   u32 = 15;
pub const PPB_DEVID_EXTMUXNUM_LOW:                  u32 = 0;
pub const PPB_DEVID_EXTMUXNUM_HIGH:                 u32 = 4;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
