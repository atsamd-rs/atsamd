#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    pub pwm_clk: crate::Reg<pwm_clk::PWM_CLK_SPEC>,
    #[doc = "0x04 - PWM Enable Register"]
    pub pwm_ena: crate::Reg<pwm_ena::PWM_ENA_SPEC>,
    #[doc = "0x08 - PWM Disable Register"]
    pub pwm_dis: crate::Reg<pwm_dis::PWM_DIS_SPEC>,
    #[doc = "0x0c - PWM Status Register"]
    pub pwm_sr: crate::Reg<pwm_sr::PWM_SR_SPEC>,
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    pub pwm_ier1: crate::Reg<pwm_ier1::PWM_IER1_SPEC>,
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    pub pwm_idr1: crate::Reg<pwm_idr1::PWM_IDR1_SPEC>,
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    pub pwm_imr1: crate::Reg<pwm_imr1::PWM_IMR1_SPEC>,
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    pub pwm_isr1: crate::Reg<pwm_isr1::PWM_ISR1_SPEC>,
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    pub pwm_scm: crate::Reg<pwm_scm::PWM_SCM_SPEC>,
    #[doc = "0x24 - PWM DMA Register"]
    pub pwm_dmar: crate::Reg<pwm_dmar::PWM_DMAR_SPEC>,
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    pub pwm_scuc: crate::Reg<pwm_scuc::PWM_SCUC_SPEC>,
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    pub pwm_scup: crate::Reg<pwm_scup::PWM_SCUP_SPEC>,
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    pub pwm_scupupd: crate::Reg<pwm_scupupd::PWM_SCUPUPD_SPEC>,
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    pub pwm_ier2: crate::Reg<pwm_ier2::PWM_IER2_SPEC>,
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    pub pwm_idr2: crate::Reg<pwm_idr2::PWM_IDR2_SPEC>,
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    pub pwm_imr2: crate::Reg<pwm_imr2::PWM_IMR2_SPEC>,
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    pub pwm_isr2: crate::Reg<pwm_isr2::PWM_ISR2_SPEC>,
    #[doc = "0x44 - PWM Output Override Value Register"]
    pub pwm_oov: crate::Reg<pwm_oov::PWM_OOV_SPEC>,
    #[doc = "0x48 - PWM Output Selection Register"]
    pub pwm_os: crate::Reg<pwm_os::PWM_OS_SPEC>,
    #[doc = "0x4c - PWM Output Selection Set Register"]
    pub pwm_oss: crate::Reg<pwm_oss::PWM_OSS_SPEC>,
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    pub pwm_osc: crate::Reg<pwm_osc::PWM_OSC_SPEC>,
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    pub pwm_ossupd: crate::Reg<pwm_ossupd::PWM_OSSUPD_SPEC>,
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    pub pwm_oscupd: crate::Reg<pwm_oscupd::PWM_OSCUPD_SPEC>,
    #[doc = "0x5c - PWM Fault Mode Register"]
    pub pwm_fmr: crate::Reg<pwm_fmr::PWM_FMR_SPEC>,
    #[doc = "0x60 - PWM Fault Status Register"]
    pub pwm_fsr: crate::Reg<pwm_fsr::PWM_FSR_SPEC>,
    #[doc = "0x64 - PWM Fault Clear Register"]
    pub pwm_fcr: crate::Reg<pwm_fcr::PWM_FCR_SPEC>,
    #[doc = "0x68 - PWM Fault Protection Value Register 1"]
    pub pwm_fpv1: crate::Reg<pwm_fpv1::PWM_FPV1_SPEC>,
    #[doc = "0x6c - PWM Fault Protection Enable Register"]
    pub pwm_fpe: crate::Reg<pwm_fpe::PWM_FPE_SPEC>,
    _reserved28: [u8; 0x0c],
    #[doc = "0x7c..0x84 - PWM Event Line 0 Mode Register 0"]
    pub pwm_elmr: [crate::Reg<pwm_elmr::PWM_ELMR_SPEC>; 2],
    _reserved29: [u8; 0x1c],
    #[doc = "0xa0 - PWM Spread Spectrum Register"]
    pub pwm_sspr: crate::Reg<pwm_sspr::PWM_SSPR_SPEC>,
    #[doc = "0xa4 - PWM Spread Spectrum Update Register"]
    pub pwm_sspup: crate::Reg<pwm_sspup::PWM_SSPUP_SPEC>,
    _reserved31: [u8; 0x08],
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    pub pwm_smmr: crate::Reg<pwm_smmr::PWM_SMMR_SPEC>,
    _reserved32: [u8; 0x0c],
    #[doc = "0xc0 - PWM Fault Protection Value 2 Register"]
    pub pwm_fpv2: crate::Reg<pwm_fpv2::PWM_FPV2_SPEC>,
    _reserved33: [u8; 0x20],
    #[doc = "0xe4 - PWM Write Protection Control Register"]
    pub pwm_wpcr: crate::Reg<pwm_wpcr::PWM_WPCR_SPEC>,
    #[doc = "0xe8 - PWM Write Protection Status Register"]
    pub pwm_wpsr: crate::Reg<pwm_wpsr::PWM_WPSR_SPEC>,
    _reserved35: [u8; 0x44],
    #[doc = "0x130..0x1b0 - PWM Comparison 0 Value Register"]
    pub pwm_cmp: [PWM_CMP; 8],
    _reserved36: [u8; 0x50],
    #[doc = "0x200..0x280 - PWM Channel Mode Register"]
    pub pwm_ch_num: [PWM_CH_NUM; 4],
    _reserved37: [u8; 0x0180],
    #[doc = "0x400 - PWM Channel Mode Update Register (ch_num = 0)"]
    pub pwm_cmupd0: crate::Reg<pwm_cmupd0::PWM_CMUPD0_SPEC>,
    _reserved38: [u8; 0x1c],
    #[doc = "0x420 - PWM Channel Mode Update Register (ch_num = 1)"]
    pub pwm_cmupd1: crate::Reg<pwm_cmupd1::PWM_CMUPD1_SPEC>,
    _reserved39: [u8; 0x08],
    #[doc = "0x42c - PWM External Trigger Register (trg_num = 1)"]
    pub pwm_etrg1: crate::Reg<pwm_etrg1::PWM_ETRG1_SPEC>,
    #[doc = "0x430 - PWM Leading-Edge Blanking Register (trg_num = 1)"]
    pub pwm_lebr1: crate::Reg<pwm_lebr1::PWM_LEBR1_SPEC>,
    _reserved41: [u8; 0x0c],
    #[doc = "0x440 - PWM Channel Mode Update Register (ch_num = 2)"]
    pub pwm_cmupd2: crate::Reg<pwm_cmupd2::PWM_CMUPD2_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0x44c - PWM External Trigger Register (trg_num = 2)"]
    pub pwm_etrg2: crate::Reg<pwm_etrg2::PWM_ETRG2_SPEC>,
    #[doc = "0x450 - PWM Leading-Edge Blanking Register (trg_num = 2)"]
    pub pwm_lebr2: crate::Reg<pwm_lebr2::PWM_LEBR2_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0x460 - PWM Channel Mode Update Register (ch_num = 3)"]
    pub pwm_cmupd3: crate::Reg<pwm_cmupd3::PWM_CMUPD3_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CMP {
    #[doc = "0x00 - PWM Comparison 0 Value Register"]
    pub pwm_cmpv: crate::Reg<self::pwm_cmp::pwm_cmpv::PWM_CMPV_SPEC>,
    #[doc = "0x04 - PWM Comparison 0 Value Update Register"]
    pub pwm_cmpvupd: crate::Reg<self::pwm_cmp::pwm_cmpvupd::PWM_CMPVUPD_SPEC>,
    #[doc = "0x08 - PWM Comparison 0 Mode Register"]
    pub pwm_cmpm: crate::Reg<self::pwm_cmp::pwm_cmpm::PWM_CMPM_SPEC>,
    #[doc = "0x0c - PWM Comparison 0 Mode Update Register"]
    pub pwm_cmpmupd: crate::Reg<self::pwm_cmp::pwm_cmpmupd::PWM_CMPMUPD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PWM Comparison 0 Value Register"]
pub mod pwm_cmp;
#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CH_NUM {
    #[doc = "0x00 - PWM Channel Mode Register"]
    pub pwm_cmr: crate::Reg<self::pwm_ch_num::pwm_cmr::PWM_CMR_SPEC>,
    #[doc = "0x04 - PWM Channel Duty Cycle Register"]
    pub pwm_cdty: crate::Reg<self::pwm_ch_num::pwm_cdty::PWM_CDTY_SPEC>,
    #[doc = "0x08 - PWM Channel Duty Cycle Update Register"]
    pub pwm_cdtyupd: crate::Reg<self::pwm_ch_num::pwm_cdtyupd::PWM_CDTYUPD_SPEC>,
    #[doc = "0x0c - PWM Channel Period Register"]
    pub pwm_cprd: crate::Reg<self::pwm_ch_num::pwm_cprd::PWM_CPRD_SPEC>,
    #[doc = "0x10 - PWM Channel Period Update Register"]
    pub pwm_cprdupd: crate::Reg<self::pwm_ch_num::pwm_cprdupd::PWM_CPRDUPD_SPEC>,
    #[doc = "0x14 - PWM Channel Counter Register"]
    pub pwm_ccnt: crate::Reg<self::pwm_ch_num::pwm_ccnt::PWM_CCNT_SPEC>,
    #[doc = "0x18 - PWM Channel Dead Time Register"]
    pub pwm_dt: crate::Reg<self::pwm_ch_num::pwm_dt::PWM_DT_SPEC>,
    #[doc = "0x1c - PWM Channel Dead Time Update Register"]
    pub pwm_dtupd: crate::Reg<self::pwm_ch_num::pwm_dtupd::PWM_DTUPD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PWM Channel Mode Register"]
pub mod pwm_ch_num;
#[doc = "PWM_CLK register accessor: an alias for `Reg<PWM_CLK_SPEC>`"]
pub type PWM_CLK = crate::Reg<pwm_clk::PWM_CLK_SPEC>;
#[doc = "PWM Clock Register"]
pub mod pwm_clk;
#[doc = "PWM_ENA register accessor: an alias for `Reg<PWM_ENA_SPEC>`"]
pub type PWM_ENA = crate::Reg<pwm_ena::PWM_ENA_SPEC>;
#[doc = "PWM Enable Register"]
pub mod pwm_ena;
#[doc = "PWM_DIS register accessor: an alias for `Reg<PWM_DIS_SPEC>`"]
pub type PWM_DIS = crate::Reg<pwm_dis::PWM_DIS_SPEC>;
#[doc = "PWM Disable Register"]
pub mod pwm_dis;
#[doc = "PWM_SR register accessor: an alias for `Reg<PWM_SR_SPEC>`"]
pub type PWM_SR = crate::Reg<pwm_sr::PWM_SR_SPEC>;
#[doc = "PWM Status Register"]
pub mod pwm_sr;
#[doc = "PWM_IER1 register accessor: an alias for `Reg<PWM_IER1_SPEC>`"]
pub type PWM_IER1 = crate::Reg<pwm_ier1::PWM_IER1_SPEC>;
#[doc = "PWM Interrupt Enable Register 1"]
pub mod pwm_ier1;
#[doc = "PWM_IDR1 register accessor: an alias for `Reg<PWM_IDR1_SPEC>`"]
pub type PWM_IDR1 = crate::Reg<pwm_idr1::PWM_IDR1_SPEC>;
#[doc = "PWM Interrupt Disable Register 1"]
pub mod pwm_idr1;
#[doc = "PWM_IMR1 register accessor: an alias for `Reg<PWM_IMR1_SPEC>`"]
pub type PWM_IMR1 = crate::Reg<pwm_imr1::PWM_IMR1_SPEC>;
#[doc = "PWM Interrupt Mask Register 1"]
pub mod pwm_imr1;
#[doc = "PWM_ISR1 register accessor: an alias for `Reg<PWM_ISR1_SPEC>`"]
pub type PWM_ISR1 = crate::Reg<pwm_isr1::PWM_ISR1_SPEC>;
#[doc = "PWM Interrupt Status Register 1"]
pub mod pwm_isr1;
#[doc = "PWM_SCM register accessor: an alias for `Reg<PWM_SCM_SPEC>`"]
pub type PWM_SCM = crate::Reg<pwm_scm::PWM_SCM_SPEC>;
#[doc = "PWM Sync Channels Mode Register"]
pub mod pwm_scm;
#[doc = "PWM_DMAR register accessor: an alias for `Reg<PWM_DMAR_SPEC>`"]
pub type PWM_DMAR = crate::Reg<pwm_dmar::PWM_DMAR_SPEC>;
#[doc = "PWM DMA Register"]
pub mod pwm_dmar;
#[doc = "PWM_SCUC register accessor: an alias for `Reg<PWM_SCUC_SPEC>`"]
pub type PWM_SCUC = crate::Reg<pwm_scuc::PWM_SCUC_SPEC>;
#[doc = "PWM Sync Channels Update Control Register"]
pub mod pwm_scuc;
#[doc = "PWM_SCUP register accessor: an alias for `Reg<PWM_SCUP_SPEC>`"]
pub type PWM_SCUP = crate::Reg<pwm_scup::PWM_SCUP_SPEC>;
#[doc = "PWM Sync Channels Update Period Register"]
pub mod pwm_scup;
#[doc = "PWM_SCUPUPD register accessor: an alias for `Reg<PWM_SCUPUPD_SPEC>`"]
pub type PWM_SCUPUPD = crate::Reg<pwm_scupupd::PWM_SCUPUPD_SPEC>;
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod pwm_scupupd;
#[doc = "PWM_IER2 register accessor: an alias for `Reg<PWM_IER2_SPEC>`"]
pub type PWM_IER2 = crate::Reg<pwm_ier2::PWM_IER2_SPEC>;
#[doc = "PWM Interrupt Enable Register 2"]
pub mod pwm_ier2;
#[doc = "PWM_IDR2 register accessor: an alias for `Reg<PWM_IDR2_SPEC>`"]
pub type PWM_IDR2 = crate::Reg<pwm_idr2::PWM_IDR2_SPEC>;
#[doc = "PWM Interrupt Disable Register 2"]
pub mod pwm_idr2;
#[doc = "PWM_IMR2 register accessor: an alias for `Reg<PWM_IMR2_SPEC>`"]
pub type PWM_IMR2 = crate::Reg<pwm_imr2::PWM_IMR2_SPEC>;
#[doc = "PWM Interrupt Mask Register 2"]
pub mod pwm_imr2;
#[doc = "PWM_ISR2 register accessor: an alias for `Reg<PWM_ISR2_SPEC>`"]
pub type PWM_ISR2 = crate::Reg<pwm_isr2::PWM_ISR2_SPEC>;
#[doc = "PWM Interrupt Status Register 2"]
pub mod pwm_isr2;
#[doc = "PWM_OOV register accessor: an alias for `Reg<PWM_OOV_SPEC>`"]
pub type PWM_OOV = crate::Reg<pwm_oov::PWM_OOV_SPEC>;
#[doc = "PWM Output Override Value Register"]
pub mod pwm_oov;
#[doc = "PWM_OS register accessor: an alias for `Reg<PWM_OS_SPEC>`"]
pub type PWM_OS = crate::Reg<pwm_os::PWM_OS_SPEC>;
#[doc = "PWM Output Selection Register"]
pub mod pwm_os;
#[doc = "PWM_OSS register accessor: an alias for `Reg<PWM_OSS_SPEC>`"]
pub type PWM_OSS = crate::Reg<pwm_oss::PWM_OSS_SPEC>;
#[doc = "PWM Output Selection Set Register"]
pub mod pwm_oss;
#[doc = "PWM_OSC register accessor: an alias for `Reg<PWM_OSC_SPEC>`"]
pub type PWM_OSC = crate::Reg<pwm_osc::PWM_OSC_SPEC>;
#[doc = "PWM Output Selection Clear Register"]
pub mod pwm_osc;
#[doc = "PWM_OSSUPD register accessor: an alias for `Reg<PWM_OSSUPD_SPEC>`"]
pub type PWM_OSSUPD = crate::Reg<pwm_ossupd::PWM_OSSUPD_SPEC>;
#[doc = "PWM Output Selection Set Update Register"]
pub mod pwm_ossupd;
#[doc = "PWM_OSCUPD register accessor: an alias for `Reg<PWM_OSCUPD_SPEC>`"]
pub type PWM_OSCUPD = crate::Reg<pwm_oscupd::PWM_OSCUPD_SPEC>;
#[doc = "PWM Output Selection Clear Update Register"]
pub mod pwm_oscupd;
#[doc = "PWM_FMR register accessor: an alias for `Reg<PWM_FMR_SPEC>`"]
pub type PWM_FMR = crate::Reg<pwm_fmr::PWM_FMR_SPEC>;
#[doc = "PWM Fault Mode Register"]
pub mod pwm_fmr;
#[doc = "PWM_FSR register accessor: an alias for `Reg<PWM_FSR_SPEC>`"]
pub type PWM_FSR = crate::Reg<pwm_fsr::PWM_FSR_SPEC>;
#[doc = "PWM Fault Status Register"]
pub mod pwm_fsr;
#[doc = "PWM_FCR register accessor: an alias for `Reg<PWM_FCR_SPEC>`"]
pub type PWM_FCR = crate::Reg<pwm_fcr::PWM_FCR_SPEC>;
#[doc = "PWM Fault Clear Register"]
pub mod pwm_fcr;
#[doc = "PWM_FPV1 register accessor: an alias for `Reg<PWM_FPV1_SPEC>`"]
pub type PWM_FPV1 = crate::Reg<pwm_fpv1::PWM_FPV1_SPEC>;
#[doc = "PWM Fault Protection Value Register 1"]
pub mod pwm_fpv1;
#[doc = "PWM_FPE register accessor: an alias for `Reg<PWM_FPE_SPEC>`"]
pub type PWM_FPE = crate::Reg<pwm_fpe::PWM_FPE_SPEC>;
#[doc = "PWM Fault Protection Enable Register"]
pub mod pwm_fpe;
#[doc = "PWM_ELMR register accessor: an alias for `Reg<PWM_ELMR_SPEC>`"]
pub type PWM_ELMR = crate::Reg<pwm_elmr::PWM_ELMR_SPEC>;
#[doc = "PWM Event Line 0 Mode Register 0"]
pub mod pwm_elmr;
#[doc = "PWM_SSPR register accessor: an alias for `Reg<PWM_SSPR_SPEC>`"]
pub type PWM_SSPR = crate::Reg<pwm_sspr::PWM_SSPR_SPEC>;
#[doc = "PWM Spread Spectrum Register"]
pub mod pwm_sspr;
#[doc = "PWM_SSPUP register accessor: an alias for `Reg<PWM_SSPUP_SPEC>`"]
pub type PWM_SSPUP = crate::Reg<pwm_sspup::PWM_SSPUP_SPEC>;
#[doc = "PWM Spread Spectrum Update Register"]
pub mod pwm_sspup;
#[doc = "PWM_SMMR register accessor: an alias for `Reg<PWM_SMMR_SPEC>`"]
pub type PWM_SMMR = crate::Reg<pwm_smmr::PWM_SMMR_SPEC>;
#[doc = "PWM Stepper Motor Mode Register"]
pub mod pwm_smmr;
#[doc = "PWM_FPV2 register accessor: an alias for `Reg<PWM_FPV2_SPEC>`"]
pub type PWM_FPV2 = crate::Reg<pwm_fpv2::PWM_FPV2_SPEC>;
#[doc = "PWM Fault Protection Value 2 Register"]
pub mod pwm_fpv2;
#[doc = "PWM_WPCR register accessor: an alias for `Reg<PWM_WPCR_SPEC>`"]
pub type PWM_WPCR = crate::Reg<pwm_wpcr::PWM_WPCR_SPEC>;
#[doc = "PWM Write Protection Control Register"]
pub mod pwm_wpcr;
#[doc = "PWM_WPSR register accessor: an alias for `Reg<PWM_WPSR_SPEC>`"]
pub type PWM_WPSR = crate::Reg<pwm_wpsr::PWM_WPSR_SPEC>;
#[doc = "PWM Write Protection Status Register"]
pub mod pwm_wpsr;
#[doc = "PWM_CMUPD0 register accessor: an alias for `Reg<PWM_CMUPD0_SPEC>`"]
pub type PWM_CMUPD0 = crate::Reg<pwm_cmupd0::PWM_CMUPD0_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 0)"]
pub mod pwm_cmupd0;
#[doc = "PWM_CMUPD1 register accessor: an alias for `Reg<PWM_CMUPD1_SPEC>`"]
pub type PWM_CMUPD1 = crate::Reg<pwm_cmupd1::PWM_CMUPD1_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 1)"]
pub mod pwm_cmupd1;
#[doc = "PWM_ETRG1 register accessor: an alias for `Reg<PWM_ETRG1_SPEC>`"]
pub type PWM_ETRG1 = crate::Reg<pwm_etrg1::PWM_ETRG1_SPEC>;
#[doc = "PWM External Trigger Register (trg_num = 1)"]
pub mod pwm_etrg1;
#[doc = "PWM_LEBR1 register accessor: an alias for `Reg<PWM_LEBR1_SPEC>`"]
pub type PWM_LEBR1 = crate::Reg<pwm_lebr1::PWM_LEBR1_SPEC>;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)"]
pub mod pwm_lebr1;
#[doc = "PWM_CMUPD2 register accessor: an alias for `Reg<PWM_CMUPD2_SPEC>`"]
pub type PWM_CMUPD2 = crate::Reg<pwm_cmupd2::PWM_CMUPD2_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 2)"]
pub mod pwm_cmupd2;
#[doc = "PWM_ETRG2 register accessor: an alias for `Reg<PWM_ETRG2_SPEC>`"]
pub type PWM_ETRG2 = crate::Reg<pwm_etrg2::PWM_ETRG2_SPEC>;
#[doc = "PWM External Trigger Register (trg_num = 2)"]
pub mod pwm_etrg2;
#[doc = "PWM_LEBR2 register accessor: an alias for `Reg<PWM_LEBR2_SPEC>`"]
pub type PWM_LEBR2 = crate::Reg<pwm_lebr2::PWM_LEBR2_SPEC>;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)"]
pub mod pwm_lebr2;
#[doc = "PWM_CMUPD3 register accessor: an alias for `Reg<PWM_CMUPD3_SPEC>`"]
pub type PWM_CMUPD3 = crate::Reg<pwm_cmupd3::PWM_CMUPD3_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 3)"]
pub mod pwm_cmupd3;
