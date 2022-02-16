#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFEC Control Register"]
    pub afec_cr: crate::Reg<afec_cr::AFEC_CR_SPEC>,
    #[doc = "0x04 - AFEC Mode Register"]
    pub afec_mr: crate::Reg<afec_mr::AFEC_MR_SPEC>,
    #[doc = "0x08 - AFEC Extended Mode Register"]
    pub afec_emr: crate::Reg<afec_emr::AFEC_EMR_SPEC>,
    #[doc = "0x0c - AFEC Channel Sequence 1 Register"]
    pub afec_seq1r: crate::Reg<afec_seq1r::AFEC_SEQ1R_SPEC>,
    #[doc = "0x10 - AFEC Channel Sequence 2 Register"]
    pub afec_seq2r: crate::Reg<afec_seq2r::AFEC_SEQ2R_SPEC>,
    #[doc = "0x14 - AFEC Channel Enable Register"]
    pub afec_cher: crate::Reg<afec_cher::AFEC_CHER_SPEC>,
    #[doc = "0x18 - AFEC Channel Disable Register"]
    pub afec_chdr: crate::Reg<afec_chdr::AFEC_CHDR_SPEC>,
    #[doc = "0x1c - AFEC Channel Status Register"]
    pub afec_chsr: crate::Reg<afec_chsr::AFEC_CHSR_SPEC>,
    #[doc = "0x20 - AFEC Last Converted Data Register"]
    pub afec_lcdr: crate::Reg<afec_lcdr::AFEC_LCDR_SPEC>,
    #[doc = "0x24 - AFEC Interrupt Enable Register"]
    pub afec_ier: crate::Reg<afec_ier::AFEC_IER_SPEC>,
    #[doc = "0x28 - AFEC Interrupt Disable Register"]
    pub afec_idr: crate::Reg<afec_idr::AFEC_IDR_SPEC>,
    #[doc = "0x2c - AFEC Interrupt Mask Register"]
    pub afec_imr: crate::Reg<afec_imr::AFEC_IMR_SPEC>,
    #[doc = "0x30 - AFEC Interrupt Status Register"]
    pub afec_isr: crate::Reg<afec_isr::AFEC_ISR_SPEC>,
    _reserved13: [u8; 0x18],
    #[doc = "0x4c - AFEC Overrun Status Register"]
    pub afec_over: crate::Reg<afec_over::AFEC_OVER_SPEC>,
    #[doc = "0x50 - AFEC Compare Window Register"]
    pub afec_cwr: crate::Reg<afec_cwr::AFEC_CWR_SPEC>,
    #[doc = "0x54 - AFEC Channel Gain Register"]
    pub afec_cgr: crate::Reg<afec_cgr::AFEC_CGR_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x60 - AFEC Channel Differential Register"]
    pub afec_diffr: crate::Reg<afec_diffr::AFEC_DIFFR_SPEC>,
    #[doc = "0x64 - AFEC Channel Selection Register"]
    pub afec_cselr: crate::Reg<afec_cselr::AFEC_CSELR_SPEC>,
    #[doc = "0x68 - AFEC Channel Data Register"]
    pub afec_cdr: crate::Reg<afec_cdr::AFEC_CDR_SPEC>,
    #[doc = "0x6c - AFEC Channel Offset Compensation Register"]
    pub afec_cocr: crate::Reg<afec_cocr::AFEC_COCR_SPEC>,
    #[doc = "0x70 - AFEC Temperature Sensor Mode Register"]
    pub afec_tempmr: crate::Reg<afec_tempmr::AFEC_TEMPMR_SPEC>,
    #[doc = "0x74 - AFEC Temperature Compare Window Register"]
    pub afec_tempcwr: crate::Reg<afec_tempcwr::AFEC_TEMPCWR_SPEC>,
    _reserved22: [u8; 0x1c],
    #[doc = "0x94 - AFEC Analog Control Register"]
    pub afec_acr: crate::Reg<afec_acr::AFEC_ACR_SPEC>,
    _reserved23: [u8; 0x08],
    #[doc = "0xa0 - AFEC Sample & Hold Mode Register"]
    pub afec_shmr: crate::Reg<afec_shmr::AFEC_SHMR_SPEC>,
    _reserved24: [u8; 0x2c],
    #[doc = "0xd0 - AFEC Correction Select Register"]
    pub afec_cosr: crate::Reg<afec_cosr::AFEC_COSR_SPEC>,
    #[doc = "0xd4 - AFEC Correction Values Register"]
    pub afec_cvr: crate::Reg<afec_cvr::AFEC_CVR_SPEC>,
    #[doc = "0xd8 - AFEC Channel Error Correction Register"]
    pub afec_cecr: crate::Reg<afec_cecr::AFEC_CECR_SPEC>,
    _reserved27: [u8; 0x08],
    #[doc = "0xe4 - AFEC Write Protection Mode Register"]
    pub afec_wpmr: crate::Reg<afec_wpmr::AFEC_WPMR_SPEC>,
    #[doc = "0xe8 - AFEC Write Protection Status Register"]
    pub afec_wpsr: crate::Reg<afec_wpsr::AFEC_WPSR_SPEC>,
}
#[doc = "AFEC_CR register accessor: an alias for `Reg<AFEC_CR_SPEC>`"]
pub type AFEC_CR = crate::Reg<afec_cr::AFEC_CR_SPEC>;
#[doc = "AFEC Control Register"]
pub mod afec_cr;
#[doc = "AFEC_MR register accessor: an alias for `Reg<AFEC_MR_SPEC>`"]
pub type AFEC_MR = crate::Reg<afec_mr::AFEC_MR_SPEC>;
#[doc = "AFEC Mode Register"]
pub mod afec_mr;
#[doc = "AFEC_EMR register accessor: an alias for `Reg<AFEC_EMR_SPEC>`"]
pub type AFEC_EMR = crate::Reg<afec_emr::AFEC_EMR_SPEC>;
#[doc = "AFEC Extended Mode Register"]
pub mod afec_emr;
#[doc = "AFEC_SEQ1R register accessor: an alias for `Reg<AFEC_SEQ1R_SPEC>`"]
pub type AFEC_SEQ1R = crate::Reg<afec_seq1r::AFEC_SEQ1R_SPEC>;
#[doc = "AFEC Channel Sequence 1 Register"]
pub mod afec_seq1r;
#[doc = "AFEC_SEQ2R register accessor: an alias for `Reg<AFEC_SEQ2R_SPEC>`"]
pub type AFEC_SEQ2R = crate::Reg<afec_seq2r::AFEC_SEQ2R_SPEC>;
#[doc = "AFEC Channel Sequence 2 Register"]
pub mod afec_seq2r;
#[doc = "AFEC_CHER register accessor: an alias for `Reg<AFEC_CHER_SPEC>`"]
pub type AFEC_CHER = crate::Reg<afec_cher::AFEC_CHER_SPEC>;
#[doc = "AFEC Channel Enable Register"]
pub mod afec_cher;
#[doc = "AFEC_CHDR register accessor: an alias for `Reg<AFEC_CHDR_SPEC>`"]
pub type AFEC_CHDR = crate::Reg<afec_chdr::AFEC_CHDR_SPEC>;
#[doc = "AFEC Channel Disable Register"]
pub mod afec_chdr;
#[doc = "AFEC_CHSR register accessor: an alias for `Reg<AFEC_CHSR_SPEC>`"]
pub type AFEC_CHSR = crate::Reg<afec_chsr::AFEC_CHSR_SPEC>;
#[doc = "AFEC Channel Status Register"]
pub mod afec_chsr;
#[doc = "AFEC_LCDR register accessor: an alias for `Reg<AFEC_LCDR_SPEC>`"]
pub type AFEC_LCDR = crate::Reg<afec_lcdr::AFEC_LCDR_SPEC>;
#[doc = "AFEC Last Converted Data Register"]
pub mod afec_lcdr;
#[doc = "AFEC_IER register accessor: an alias for `Reg<AFEC_IER_SPEC>`"]
pub type AFEC_IER = crate::Reg<afec_ier::AFEC_IER_SPEC>;
#[doc = "AFEC Interrupt Enable Register"]
pub mod afec_ier;
#[doc = "AFEC_IDR register accessor: an alias for `Reg<AFEC_IDR_SPEC>`"]
pub type AFEC_IDR = crate::Reg<afec_idr::AFEC_IDR_SPEC>;
#[doc = "AFEC Interrupt Disable Register"]
pub mod afec_idr;
#[doc = "AFEC_IMR register accessor: an alias for `Reg<AFEC_IMR_SPEC>`"]
pub type AFEC_IMR = crate::Reg<afec_imr::AFEC_IMR_SPEC>;
#[doc = "AFEC Interrupt Mask Register"]
pub mod afec_imr;
#[doc = "AFEC_ISR register accessor: an alias for `Reg<AFEC_ISR_SPEC>`"]
pub type AFEC_ISR = crate::Reg<afec_isr::AFEC_ISR_SPEC>;
#[doc = "AFEC Interrupt Status Register"]
pub mod afec_isr;
#[doc = "AFEC_OVER register accessor: an alias for `Reg<AFEC_OVER_SPEC>`"]
pub type AFEC_OVER = crate::Reg<afec_over::AFEC_OVER_SPEC>;
#[doc = "AFEC Overrun Status Register"]
pub mod afec_over;
#[doc = "AFEC_CWR register accessor: an alias for `Reg<AFEC_CWR_SPEC>`"]
pub type AFEC_CWR = crate::Reg<afec_cwr::AFEC_CWR_SPEC>;
#[doc = "AFEC Compare Window Register"]
pub mod afec_cwr;
#[doc = "AFEC_CGR register accessor: an alias for `Reg<AFEC_CGR_SPEC>`"]
pub type AFEC_CGR = crate::Reg<afec_cgr::AFEC_CGR_SPEC>;
#[doc = "AFEC Channel Gain Register"]
pub mod afec_cgr;
#[doc = "AFEC_DIFFR register accessor: an alias for `Reg<AFEC_DIFFR_SPEC>`"]
pub type AFEC_DIFFR = crate::Reg<afec_diffr::AFEC_DIFFR_SPEC>;
#[doc = "AFEC Channel Differential Register"]
pub mod afec_diffr;
#[doc = "AFEC_CSELR register accessor: an alias for `Reg<AFEC_CSELR_SPEC>`"]
pub type AFEC_CSELR = crate::Reg<afec_cselr::AFEC_CSELR_SPEC>;
#[doc = "AFEC Channel Selection Register"]
pub mod afec_cselr;
#[doc = "AFEC_CDR register accessor: an alias for `Reg<AFEC_CDR_SPEC>`"]
pub type AFEC_CDR = crate::Reg<afec_cdr::AFEC_CDR_SPEC>;
#[doc = "AFEC Channel Data Register"]
pub mod afec_cdr;
#[doc = "AFEC_COCR register accessor: an alias for `Reg<AFEC_COCR_SPEC>`"]
pub type AFEC_COCR = crate::Reg<afec_cocr::AFEC_COCR_SPEC>;
#[doc = "AFEC Channel Offset Compensation Register"]
pub mod afec_cocr;
#[doc = "AFEC_TEMPMR register accessor: an alias for `Reg<AFEC_TEMPMR_SPEC>`"]
pub type AFEC_TEMPMR = crate::Reg<afec_tempmr::AFEC_TEMPMR_SPEC>;
#[doc = "AFEC Temperature Sensor Mode Register"]
pub mod afec_tempmr;
#[doc = "AFEC_TEMPCWR register accessor: an alias for `Reg<AFEC_TEMPCWR_SPEC>`"]
pub type AFEC_TEMPCWR = crate::Reg<afec_tempcwr::AFEC_TEMPCWR_SPEC>;
#[doc = "AFEC Temperature Compare Window Register"]
pub mod afec_tempcwr;
#[doc = "AFEC_ACR register accessor: an alias for `Reg<AFEC_ACR_SPEC>`"]
pub type AFEC_ACR = crate::Reg<afec_acr::AFEC_ACR_SPEC>;
#[doc = "AFEC Analog Control Register"]
pub mod afec_acr;
#[doc = "AFEC_SHMR register accessor: an alias for `Reg<AFEC_SHMR_SPEC>`"]
pub type AFEC_SHMR = crate::Reg<afec_shmr::AFEC_SHMR_SPEC>;
#[doc = "AFEC Sample & Hold Mode Register"]
pub mod afec_shmr;
#[doc = "AFEC_COSR register accessor: an alias for `Reg<AFEC_COSR_SPEC>`"]
pub type AFEC_COSR = crate::Reg<afec_cosr::AFEC_COSR_SPEC>;
#[doc = "AFEC Correction Select Register"]
pub mod afec_cosr;
#[doc = "AFEC_CVR register accessor: an alias for `Reg<AFEC_CVR_SPEC>`"]
pub type AFEC_CVR = crate::Reg<afec_cvr::AFEC_CVR_SPEC>;
#[doc = "AFEC Correction Values Register"]
pub mod afec_cvr;
#[doc = "AFEC_CECR register accessor: an alias for `Reg<AFEC_CECR_SPEC>`"]
pub type AFEC_CECR = crate::Reg<afec_cecr::AFEC_CECR_SPEC>;
#[doc = "AFEC Channel Error Correction Register"]
pub mod afec_cecr;
#[doc = "AFEC_WPMR register accessor: an alias for `Reg<AFEC_WPMR_SPEC>`"]
pub type AFEC_WPMR = crate::Reg<afec_wpmr::AFEC_WPMR_SPEC>;
#[doc = "AFEC Write Protection Mode Register"]
pub mod afec_wpmr;
#[doc = "AFEC_WPSR register accessor: an alias for `Reg<AFEC_WPSR_SPEC>`"]
pub type AFEC_WPSR = crate::Reg<afec_wpsr::AFEC_WPSR_SPEC>;
#[doc = "AFEC Write Protection Status Register"]
pub mod afec_wpsr;
