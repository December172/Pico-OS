#![allow(dead_code)]
// PPB
pub const PPB_BASE:                                 u32 = 0xE000_0000;
pub const PPB_SYST_CSR:                             u32 = PPB_BASE + 0xE010;
pub const PPB_SYST_RVR:                             u32 = PPB_BASE + 0xE014;
pub const PPB_SYST_CVR:                             u32 = PPB_BASE + 0xE018;
pub const PPB_SYST_CALIB:                           u32 = PPB_BASE + 0xE01C;
pub const PPB_NVIC_ISER:                            u32 = PPB_BASE + 0xE100;
pub const PPB_NVIC_ICER:                            u32 = PPB_BASE + 0xE180;
pub const PPB_NVIC_ISPR:                            u32 = PPB_BASE + 0xE200;
pub const PPB_NVIC_ICPR:                            u32 = PPB_BASE + 0xE280;
pub fn PPB_NVIC_IPR(n: u32) -> u32 {
    return PPB_BASE + 0xE400 + n * 4
}
pub const PPB_CPUID:                                u32 = PPB_BASE + 0xED00;
pub const PPB_ICSR:                                 u32 = PPB_BASE + 0xED04;
pub const PPB_VTOR:                                 u32 = PPB_BASE + 0xED08;
pub const PPB_AIRCR:                                u32 = PPB_BASE + 0xED0C;
pub const PPB_SCR:                                  u32 = PPB_BASE + 0xED10;
pub const PPB_CCR:                                  u32 = PPB_BASE + 0xED14;
pub const PPB_SHPR2:                                u32 = PPB_BASE + 0xED1C;
pub const PPB_SHPR3:                                u32 = PPB_BASE + 0xED20;
pub const PPB_SHCSR:                                u32 = PPB_BASE + 0xED24;
pub const PPB_MPU_TYPE:                             u32 = PPB_BASE + 0xED90;
pub const PPB_MPU_CTRL:                             u32 = PPB_BASE + 0xED94;
pub const PPB_MPU_RNR:                              u32 = PPB_BASE + 0xED98;
pub const PPB_MPU_RBAR:                             u32 = PPB_BASE + 0xED9C;
pub const PPB_MPU_RASR:                             u32 = PPB_BASE + 0xEDA0;

// ==== BEGIN AUTO-GENERATED FIELD BIT RANGES (tools/gen_field_ranges.py) ====
// Generated from specs/RP2040.svd -- do not edit by hand.

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
// NVIC_ISER
pub const PPB_NVIC_ISER_SETENA_LOW:                 u32 = 0;
pub const PPB_NVIC_ISER_SETENA_HIGH:                u32 = 31;
// NVIC_ICER
pub const PPB_NVIC_ICER_CLRENA_LOW:                 u32 = 0;
pub const PPB_NVIC_ICER_CLRENA_HIGH:                u32 = 31;
// NVIC_ISPR
pub const PPB_NVIC_ISPR_SETPEND_LOW:                u32 = 0;
pub const PPB_NVIC_ISPR_SETPEND_HIGH:               u32 = 31;
// NVIC_ICPR
pub const PPB_NVIC_ICPR_CLRPEND_LOW:                u32 = 0;
pub const PPB_NVIC_ICPR_CLRPEND_HIGH:               u32 = 31;
// NVIC_IPR0
pub const PPB_NVIC_IPR0_IP_3_LOW:                   u32 = 30;
pub const PPB_NVIC_IPR0_IP_3_HIGH:                  u32 = 31;
pub const PPB_NVIC_IPR0_IP_2_LOW:                   u32 = 22;
pub const PPB_NVIC_IPR0_IP_2_HIGH:                  u32 = 23;
pub const PPB_NVIC_IPR0_IP_1_LOW:                   u32 = 14;
pub const PPB_NVIC_IPR0_IP_1_HIGH:                  u32 = 15;
pub const PPB_NVIC_IPR0_IP_0_LOW:                   u32 = 6;
pub const PPB_NVIC_IPR0_IP_0_HIGH:                  u32 = 7;
// NVIC_IPR1
pub const PPB_NVIC_IPR1_IP_7_LOW:                   u32 = 30;
pub const PPB_NVIC_IPR1_IP_7_HIGH:                  u32 = 31;
pub const PPB_NVIC_IPR1_IP_6_LOW:                   u32 = 22;
pub const PPB_NVIC_IPR1_IP_6_HIGH:                  u32 = 23;
pub const PPB_NVIC_IPR1_IP_5_LOW:                   u32 = 14;
pub const PPB_NVIC_IPR1_IP_5_HIGH:                  u32 = 15;
pub const PPB_NVIC_IPR1_IP_4_LOW:                   u32 = 6;
pub const PPB_NVIC_IPR1_IP_4_HIGH:                  u32 = 7;
// NVIC_IPR2
pub const PPB_NVIC_IPR2_IP_11_LOW:                  u32 = 30;
pub const PPB_NVIC_IPR2_IP_11_HIGH:                 u32 = 31;
pub const PPB_NVIC_IPR2_IP_10_LOW:                  u32 = 22;
pub const PPB_NVIC_IPR2_IP_10_HIGH:                 u32 = 23;
pub const PPB_NVIC_IPR2_IP_9_LOW:                   u32 = 14;
pub const PPB_NVIC_IPR2_IP_9_HIGH:                  u32 = 15;
pub const PPB_NVIC_IPR2_IP_8_LOW:                   u32 = 6;
pub const PPB_NVIC_IPR2_IP_8_HIGH:                  u32 = 7;
// NVIC_IPR3
pub const PPB_NVIC_IPR3_IP_15_LOW:                  u32 = 30;
pub const PPB_NVIC_IPR3_IP_15_HIGH:                 u32 = 31;
pub const PPB_NVIC_IPR3_IP_14_LOW:                  u32 = 22;
pub const PPB_NVIC_IPR3_IP_14_HIGH:                 u32 = 23;
pub const PPB_NVIC_IPR3_IP_13_LOW:                  u32 = 14;
pub const PPB_NVIC_IPR3_IP_13_HIGH:                 u32 = 15;
pub const PPB_NVIC_IPR3_IP_12_LOW:                  u32 = 6;
pub const PPB_NVIC_IPR3_IP_12_HIGH:                 u32 = 7;
// NVIC_IPR4
pub const PPB_NVIC_IPR4_IP_19_LOW:                  u32 = 30;
pub const PPB_NVIC_IPR4_IP_19_HIGH:                 u32 = 31;
pub const PPB_NVIC_IPR4_IP_18_LOW:                  u32 = 22;
pub const PPB_NVIC_IPR4_IP_18_HIGH:                 u32 = 23;
pub const PPB_NVIC_IPR4_IP_17_LOW:                  u32 = 14;
pub const PPB_NVIC_IPR4_IP_17_HIGH:                 u32 = 15;
pub const PPB_NVIC_IPR4_IP_16_LOW:                  u32 = 6;
pub const PPB_NVIC_IPR4_IP_16_HIGH:                 u32 = 7;
// NVIC_IPR5
pub const PPB_NVIC_IPR5_IP_23_LOW:                  u32 = 30;
pub const PPB_NVIC_IPR5_IP_23_HIGH:                 u32 = 31;
pub const PPB_NVIC_IPR5_IP_22_LOW:                  u32 = 22;
pub const PPB_NVIC_IPR5_IP_22_HIGH:                 u32 = 23;
pub const PPB_NVIC_IPR5_IP_21_LOW:                  u32 = 14;
pub const PPB_NVIC_IPR5_IP_21_HIGH:                 u32 = 15;
pub const PPB_NVIC_IPR5_IP_20_LOW:                  u32 = 6;
pub const PPB_NVIC_IPR5_IP_20_HIGH:                 u32 = 7;
// NVIC_IPR6
pub const PPB_NVIC_IPR6_IP_27_LOW:                  u32 = 30;
pub const PPB_NVIC_IPR6_IP_27_HIGH:                 u32 = 31;
pub const PPB_NVIC_IPR6_IP_26_LOW:                  u32 = 22;
pub const PPB_NVIC_IPR6_IP_26_HIGH:                 u32 = 23;
pub const PPB_NVIC_IPR6_IP_25_LOW:                  u32 = 14;
pub const PPB_NVIC_IPR6_IP_25_HIGH:                 u32 = 15;
pub const PPB_NVIC_IPR6_IP_24_LOW:                  u32 = 6;
pub const PPB_NVIC_IPR6_IP_24_HIGH:                 u32 = 7;
// NVIC_IPR7
pub const PPB_NVIC_IPR7_IP_31_LOW:                  u32 = 30;
pub const PPB_NVIC_IPR7_IP_31_HIGH:                 u32 = 31;
pub const PPB_NVIC_IPR7_IP_30_LOW:                  u32 = 22;
pub const PPB_NVIC_IPR7_IP_30_HIGH:                 u32 = 23;
pub const PPB_NVIC_IPR7_IP_29_LOW:                  u32 = 14;
pub const PPB_NVIC_IPR7_IP_29_HIGH:                 u32 = 15;
pub const PPB_NVIC_IPR7_IP_28_LOW:                  u32 = 6;
pub const PPB_NVIC_IPR7_IP_28_HIGH:                 u32 = 7;
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
pub const PPB_ICSR_NMIPENDSET_BIT:                  u32 = 31;
pub const PPB_ICSR_PENDSVSET_BIT:                   u32 = 28;
pub const PPB_ICSR_PENDSVCLR_BIT:                   u32 = 27;
pub const PPB_ICSR_PENDSTSET_BIT:                   u32 = 26;
pub const PPB_ICSR_PENDSTCLR_BIT:                   u32 = 25;
pub const PPB_ICSR_ISRPREEMPT_BIT:                  u32 = 23;
pub const PPB_ICSR_ISRPENDING_BIT:                  u32 = 22;
pub const PPB_ICSR_VECTPENDING_LOW:                 u32 = 12;
pub const PPB_ICSR_VECTPENDING_HIGH:                u32 = 20;
pub const PPB_ICSR_VECTACTIVE_LOW:                  u32 = 0;
pub const PPB_ICSR_VECTACTIVE_HIGH:                 u32 = 8;
// VTOR
pub const PPB_VTOR_TBLOFF_LOW:                      u32 = 8;
pub const PPB_VTOR_TBLOFF_HIGH:                     u32 = 31;
// AIRCR
pub const PPB_AIRCR_VECTKEY_LOW:                    u32 = 16;
pub const PPB_AIRCR_VECTKEY_HIGH:                   u32 = 31;
pub const PPB_AIRCR_ENDIANESS_BIT:                  u32 = 15;
pub const PPB_AIRCR_SYSRESETREQ_BIT:                u32 = 2;
pub const PPB_AIRCR_VECTCLRACTIVE_BIT:              u32 = 1;
// SCR
pub const PPB_SCR_SEVONPEND_BIT:                    u32 = 4;
pub const PPB_SCR_SLEEPDEEP_BIT:                    u32 = 2;
pub const PPB_SCR_SLEEPONEXIT_BIT:                  u32 = 1;
// CCR
pub const PPB_CCR_STKALIGN_BIT:                     u32 = 9;
pub const PPB_CCR_UNALIGN_TRP_BIT:                  u32 = 3;
// SHPR2
pub const PPB_SHPR2_PRI_11_LOW:                     u32 = 30;
pub const PPB_SHPR2_PRI_11_HIGH:                    u32 = 31;
// SHPR3
pub const PPB_SHPR3_PRI_15_LOW:                     u32 = 30;
pub const PPB_SHPR3_PRI_15_HIGH:                    u32 = 31;
pub const PPB_SHPR3_PRI_14_LOW:                     u32 = 22;
pub const PPB_SHPR3_PRI_14_HIGH:                    u32 = 23;
// SHCSR
pub const PPB_SHCSR_SVCALLPENDED_BIT:               u32 = 15;
// MPU_TYPE
pub const PPB_MPU_TYPE_IREGION_LOW:                 u32 = 16;
pub const PPB_MPU_TYPE_IREGION_HIGH:                u32 = 23;
pub const PPB_MPU_TYPE_DREGION_LOW:                 u32 = 8;
pub const PPB_MPU_TYPE_DREGION_HIGH:                u32 = 15;
pub const PPB_MPU_TYPE_SEPARATE_BIT:                u32 = 0;
// MPU_CTRL
pub const PPB_MPU_CTRL_PRIVDEFENA_BIT:              u32 = 2;
pub const PPB_MPU_CTRL_HFNMIENA_BIT:                u32 = 1;
pub const PPB_MPU_CTRL_ENABLE_BIT:                  u32 = 0;
// MPU_RNR
pub const PPB_MPU_RNR_REGION_LOW:                   u32 = 0;
pub const PPB_MPU_RNR_REGION_HIGH:                  u32 = 3;
// MPU_RBAR
pub const PPB_MPU_RBAR_ADDR_LOW:                    u32 = 8;
pub const PPB_MPU_RBAR_ADDR_HIGH:                   u32 = 31;
pub const PPB_MPU_RBAR_VALID_BIT:                   u32 = 4;
pub const PPB_MPU_RBAR_REGION_LOW:                  u32 = 0;
pub const PPB_MPU_RBAR_REGION_HIGH:                 u32 = 3;
// MPU_RASR
pub const PPB_MPU_RASR_ATTRS_LOW:                   u32 = 16;
pub const PPB_MPU_RASR_ATTRS_HIGH:                  u32 = 31;
pub const PPB_MPU_RASR_SRD_LOW:                     u32 = 8;
pub const PPB_MPU_RASR_SRD_HIGH:                    u32 = 15;
pub const PPB_MPU_RASR_SIZE_LOW:                    u32 = 1;
pub const PPB_MPU_RASR_SIZE_HIGH:                   u32 = 5;
pub const PPB_MPU_RASR_ENABLE_BIT:                  u32 = 0;
// ==== END AUTO-GENERATED FIELD BIT RANGES ====
