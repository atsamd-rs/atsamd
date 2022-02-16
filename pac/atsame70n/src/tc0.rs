#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x34 - Channel Control Register (channel = 0)"]
    pub tc_channel0: TC_CHANNEL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x40..0x74 - Channel Control Register (channel = 0)"]
    pub tc_channel1: TC_CHANNEL,
    _reserved2: [u8; 0x0c],
    #[doc = "0x80..0xb4 - Channel Control Register (channel = 0)"]
    pub tc_channel2: TC_CHANNEL,
    _reserved3: [u8; 0x0c],
    #[doc = "0xc0 - Block Control Register"]
    pub tc_bcr: crate::Reg<tc_bcr::TC_BCR_SPEC>,
    #[doc = "0xc4 - Block Mode Register"]
    pub tc_bmr: crate::Reg<tc_bmr::TC_BMR_SPEC>,
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    pub tc_qier: crate::Reg<tc_qier::TC_QIER_SPEC>,
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    pub tc_qidr: crate::Reg<tc_qidr::TC_QIDR_SPEC>,
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    pub tc_qimr: crate::Reg<tc_qimr::TC_QIMR_SPEC>,
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    pub tc_qisr: crate::Reg<tc_qisr::TC_QISR_SPEC>,
    #[doc = "0xd8 - Fault Mode Register"]
    pub tc_fmr: crate::Reg<tc_fmr::TC_FMR_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub tc_wpmr: crate::Reg<tc_wpmr::TC_WPMR_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TC_CHANNEL {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub tc_ccr: crate::Reg<self::tc_channel::tc_ccr::TC_CCR_SPEC>,
    _reserved_1_tc_cmr: [u8; 0x04],
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    pub tc_smmr: crate::Reg<self::tc_channel::tc_smmr::TC_SMMR_SPEC>,
    #[doc = "0x0c - Register AB (channel = 0)"]
    pub tc_rab: crate::Reg<self::tc_channel::tc_rab::TC_RAB_SPEC>,
    #[doc = "0x10 - Counter Value (channel = 0)"]
    pub tc_cv: crate::Reg<self::tc_channel::tc_cv::TC_CV_SPEC>,
    #[doc = "0x14 - Register A (channel = 0)"]
    pub tc_ra: crate::Reg<self::tc_channel::tc_ra::TC_RA_SPEC>,
    #[doc = "0x18 - Register B (channel = 0)"]
    pub tc_rb: crate::Reg<self::tc_channel::tc_rb::TC_RB_SPEC>,
    #[doc = "0x1c - Register C (channel = 0)"]
    pub tc_rc: crate::Reg<self::tc_channel::tc_rc::TC_RC_SPEC>,
    #[doc = "0x20 - Status Register (channel = 0)"]
    pub tc_sr: crate::Reg<self::tc_channel::tc_sr::TC_SR_SPEC>,
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    pub tc_ier: crate::Reg<self::tc_channel::tc_ier::TC_IER_SPEC>,
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    pub tc_idr: crate::Reg<self::tc_channel::tc_idr::TC_IDR_SPEC>,
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    pub tc_imr: crate::Reg<self::tc_channel::tc_imr::TC_IMR_SPEC>,
    #[doc = "0x30 - Extended Mode Register (channel = 0)"]
    pub tc_emr: crate::Reg<self::tc_channel::tc_emr::TC_EMR_SPEC>,
}
impl TC_CHANNEL {
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn tc_cmr_waveform_mode(
        &self,
    ) -> &crate::Reg<self::tc_channel::tc_cmr_waveform_mode::TC_CMR_WAVEFORM_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<
                    self::tc_channel::tc_cmr_waveform_mode::TC_CMR_WAVEFORM_MODE_SPEC,
                >)
        }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn tc_cmr_capture_mode(
        &self,
    ) -> &crate::Reg<self::tc_channel::tc_cmr_capture_mode::TC_CMR_CAPTURE_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<
                    self::tc_channel::tc_cmr_capture_mode::TC_CMR_CAPTURE_MODE_SPEC,
                >)
        }
    }
}
#[doc = r"Register block"]
#[doc = "Channel Control Register (channel = 0)"]
pub mod tc_channel;
#[doc = "TC_BCR register accessor: an alias for `Reg<TC_BCR_SPEC>`"]
pub type TC_BCR = crate::Reg<tc_bcr::TC_BCR_SPEC>;
#[doc = "Block Control Register"]
pub mod tc_bcr;
#[doc = "TC_BMR register accessor: an alias for `Reg<TC_BMR_SPEC>`"]
pub type TC_BMR = crate::Reg<tc_bmr::TC_BMR_SPEC>;
#[doc = "Block Mode Register"]
pub mod tc_bmr;
#[doc = "TC_QIER register accessor: an alias for `Reg<TC_QIER_SPEC>`"]
pub type TC_QIER = crate::Reg<tc_qier::TC_QIER_SPEC>;
#[doc = "QDEC Interrupt Enable Register"]
pub mod tc_qier;
#[doc = "TC_QIDR register accessor: an alias for `Reg<TC_QIDR_SPEC>`"]
pub type TC_QIDR = crate::Reg<tc_qidr::TC_QIDR_SPEC>;
#[doc = "QDEC Interrupt Disable Register"]
pub mod tc_qidr;
#[doc = "TC_QIMR register accessor: an alias for `Reg<TC_QIMR_SPEC>`"]
pub type TC_QIMR = crate::Reg<tc_qimr::TC_QIMR_SPEC>;
#[doc = "QDEC Interrupt Mask Register"]
pub mod tc_qimr;
#[doc = "TC_QISR register accessor: an alias for `Reg<TC_QISR_SPEC>`"]
pub type TC_QISR = crate::Reg<tc_qisr::TC_QISR_SPEC>;
#[doc = "QDEC Interrupt Status Register"]
pub mod tc_qisr;
#[doc = "TC_FMR register accessor: an alias for `Reg<TC_FMR_SPEC>`"]
pub type TC_FMR = crate::Reg<tc_fmr::TC_FMR_SPEC>;
#[doc = "Fault Mode Register"]
pub mod tc_fmr;
#[doc = "TC_WPMR register accessor: an alias for `Reg<TC_WPMR_SPEC>`"]
pub type TC_WPMR = crate::Reg<tc_wpmr::TC_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod tc_wpmr;
