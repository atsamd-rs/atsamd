#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ssc_cr: crate::Reg<ssc_cr::SSC_CR_SPEC>,
    #[doc = "0x04 - Clock Mode Register"]
    pub ssc_cmr: crate::Reg<ssc_cmr::SSC_CMR_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Receive Clock Mode Register"]
    pub ssc_rcmr: crate::Reg<ssc_rcmr::SSC_RCMR_SPEC>,
    #[doc = "0x14 - Receive Frame Mode Register"]
    pub ssc_rfmr: crate::Reg<ssc_rfmr::SSC_RFMR_SPEC>,
    #[doc = "0x18 - Transmit Clock Mode Register"]
    pub ssc_tcmr: crate::Reg<ssc_tcmr::SSC_TCMR_SPEC>,
    #[doc = "0x1c - Transmit Frame Mode Register"]
    pub ssc_tfmr: crate::Reg<ssc_tfmr::SSC_TFMR_SPEC>,
    #[doc = "0x20 - Receive Holding Register"]
    pub ssc_rhr: crate::Reg<ssc_rhr::SSC_RHR_SPEC>,
    #[doc = "0x24 - Transmit Holding Register"]
    pub ssc_thr: crate::Reg<ssc_thr::SSC_THR_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - Receive Sync. Holding Register"]
    pub ssc_rshr: crate::Reg<ssc_rshr::SSC_RSHR_SPEC>,
    #[doc = "0x34 - Transmit Sync. Holding Register"]
    pub ssc_tshr: crate::Reg<ssc_tshr::SSC_TSHR_SPEC>,
    #[doc = "0x38 - Receive Compare 0 Register"]
    pub ssc_rc0r: crate::Reg<ssc_rc0r::SSC_RC0R_SPEC>,
    #[doc = "0x3c - Receive Compare 1 Register"]
    pub ssc_rc1r: crate::Reg<ssc_rc1r::SSC_RC1R_SPEC>,
    #[doc = "0x40 - Status Register"]
    pub ssc_sr: crate::Reg<ssc_sr::SSC_SR_SPEC>,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ssc_ier: crate::Reg<ssc_ier::SSC_IER_SPEC>,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub ssc_idr: crate::Reg<ssc_idr::SSC_IDR_SPEC>,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub ssc_imr: crate::Reg<ssc_imr::SSC_IMR_SPEC>,
    _reserved16: [u8; 0x94],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub ssc_wpmr: crate::Reg<ssc_wpmr::SSC_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub ssc_wpsr: crate::Reg<ssc_wpsr::SSC_WPSR_SPEC>,
}
#[doc = "SSC_CR register accessor: an alias for `Reg<SSC_CR_SPEC>`"]
pub type SSC_CR = crate::Reg<ssc_cr::SSC_CR_SPEC>;
#[doc = "Control Register"]
pub mod ssc_cr;
#[doc = "SSC_CMR register accessor: an alias for `Reg<SSC_CMR_SPEC>`"]
pub type SSC_CMR = crate::Reg<ssc_cmr::SSC_CMR_SPEC>;
#[doc = "Clock Mode Register"]
pub mod ssc_cmr;
#[doc = "SSC_RCMR register accessor: an alias for `Reg<SSC_RCMR_SPEC>`"]
pub type SSC_RCMR = crate::Reg<ssc_rcmr::SSC_RCMR_SPEC>;
#[doc = "Receive Clock Mode Register"]
pub mod ssc_rcmr;
#[doc = "SSC_RFMR register accessor: an alias for `Reg<SSC_RFMR_SPEC>`"]
pub type SSC_RFMR = crate::Reg<ssc_rfmr::SSC_RFMR_SPEC>;
#[doc = "Receive Frame Mode Register"]
pub mod ssc_rfmr;
#[doc = "SSC_TCMR register accessor: an alias for `Reg<SSC_TCMR_SPEC>`"]
pub type SSC_TCMR = crate::Reg<ssc_tcmr::SSC_TCMR_SPEC>;
#[doc = "Transmit Clock Mode Register"]
pub mod ssc_tcmr;
#[doc = "SSC_TFMR register accessor: an alias for `Reg<SSC_TFMR_SPEC>`"]
pub type SSC_TFMR = crate::Reg<ssc_tfmr::SSC_TFMR_SPEC>;
#[doc = "Transmit Frame Mode Register"]
pub mod ssc_tfmr;
#[doc = "SSC_RHR register accessor: an alias for `Reg<SSC_RHR_SPEC>`"]
pub type SSC_RHR = crate::Reg<ssc_rhr::SSC_RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod ssc_rhr;
#[doc = "SSC_THR register accessor: an alias for `Reg<SSC_THR_SPEC>`"]
pub type SSC_THR = crate::Reg<ssc_thr::SSC_THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod ssc_thr;
#[doc = "SSC_RSHR register accessor: an alias for `Reg<SSC_RSHR_SPEC>`"]
pub type SSC_RSHR = crate::Reg<ssc_rshr::SSC_RSHR_SPEC>;
#[doc = "Receive Sync. Holding Register"]
pub mod ssc_rshr;
#[doc = "SSC_TSHR register accessor: an alias for `Reg<SSC_TSHR_SPEC>`"]
pub type SSC_TSHR = crate::Reg<ssc_tshr::SSC_TSHR_SPEC>;
#[doc = "Transmit Sync. Holding Register"]
pub mod ssc_tshr;
#[doc = "SSC_RC0R register accessor: an alias for `Reg<SSC_RC0R_SPEC>`"]
pub type SSC_RC0R = crate::Reg<ssc_rc0r::SSC_RC0R_SPEC>;
#[doc = "Receive Compare 0 Register"]
pub mod ssc_rc0r;
#[doc = "SSC_RC1R register accessor: an alias for `Reg<SSC_RC1R_SPEC>`"]
pub type SSC_RC1R = crate::Reg<ssc_rc1r::SSC_RC1R_SPEC>;
#[doc = "Receive Compare 1 Register"]
pub mod ssc_rc1r;
#[doc = "SSC_SR register accessor: an alias for `Reg<SSC_SR_SPEC>`"]
pub type SSC_SR = crate::Reg<ssc_sr::SSC_SR_SPEC>;
#[doc = "Status Register"]
pub mod ssc_sr;
#[doc = "SSC_IER register accessor: an alias for `Reg<SSC_IER_SPEC>`"]
pub type SSC_IER = crate::Reg<ssc_ier::SSC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ssc_ier;
#[doc = "SSC_IDR register accessor: an alias for `Reg<SSC_IDR_SPEC>`"]
pub type SSC_IDR = crate::Reg<ssc_idr::SSC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod ssc_idr;
#[doc = "SSC_IMR register accessor: an alias for `Reg<SSC_IMR_SPEC>`"]
pub type SSC_IMR = crate::Reg<ssc_imr::SSC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod ssc_imr;
#[doc = "SSC_WPMR register accessor: an alias for `Reg<SSC_WPMR_SPEC>`"]
pub type SSC_WPMR = crate::Reg<ssc_wpmr::SSC_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod ssc_wpmr;
#[doc = "SSC_WPSR register accessor: an alias for `Reg<SSC_WPSR_SPEC>`"]
pub type SSC_WPSR = crate::Reg<ssc_wpsr::SSC_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod ssc_wpsr;
