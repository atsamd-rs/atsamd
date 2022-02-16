#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub twihs_cr: crate::Reg<twihs_cr::TWIHS_CR_SPEC>,
    #[doc = "0x04 - Master Mode Register"]
    pub twihs_mmr: crate::Reg<twihs_mmr::TWIHS_MMR_SPEC>,
    #[doc = "0x08 - Slave Mode Register"]
    pub twihs_smr: crate::Reg<twihs_smr::TWIHS_SMR_SPEC>,
    #[doc = "0x0c - Internal Address Register"]
    pub twihs_iadr: crate::Reg<twihs_iadr::TWIHS_IADR_SPEC>,
    #[doc = "0x10 - Clock Waveform Generator Register"]
    pub twihs_cwgr: crate::Reg<twihs_cwgr::TWIHS_CWGR_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Status Register"]
    pub twihs_sr: crate::Reg<twihs_sr::TWIHS_SR_SPEC>,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub twihs_ier: crate::Reg<twihs_ier::TWIHS_IER_SPEC>,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub twihs_idr: crate::Reg<twihs_idr::TWIHS_IDR_SPEC>,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub twihs_imr: crate::Reg<twihs_imr::TWIHS_IMR_SPEC>,
    #[doc = "0x30 - Receive Holding Register"]
    pub twihs_rhr: crate::Reg<twihs_rhr::TWIHS_RHR_SPEC>,
    #[doc = "0x34 - Transmit Holding Register"]
    pub twihs_thr: crate::Reg<twihs_thr::TWIHS_THR_SPEC>,
    #[doc = "0x38 - SMBus Timing Register"]
    pub twihs_smbtr: crate::Reg<twihs_smbtr::TWIHS_SMBTR_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x44 - Filter Register"]
    pub twihs_filtr: crate::Reg<twihs_filtr::TWIHS_FILTR_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x4c - SleepWalking Matching Register"]
    pub twihs_swmr: crate::Reg<twihs_swmr::TWIHS_SWMR_SPEC>,
    _reserved14: [u8; 0x94],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub twihs_wpmr: crate::Reg<twihs_wpmr::TWIHS_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub twihs_wpsr: crate::Reg<twihs_wpsr::TWIHS_WPSR_SPEC>,
}
#[doc = "TWIHS_CR register accessor: an alias for `Reg<TWIHS_CR_SPEC>`"]
pub type TWIHS_CR = crate::Reg<twihs_cr::TWIHS_CR_SPEC>;
#[doc = "Control Register"]
pub mod twihs_cr;
#[doc = "TWIHS_MMR register accessor: an alias for `Reg<TWIHS_MMR_SPEC>`"]
pub type TWIHS_MMR = crate::Reg<twihs_mmr::TWIHS_MMR_SPEC>;
#[doc = "Master Mode Register"]
pub mod twihs_mmr;
#[doc = "TWIHS_SMR register accessor: an alias for `Reg<TWIHS_SMR_SPEC>`"]
pub type TWIHS_SMR = crate::Reg<twihs_smr::TWIHS_SMR_SPEC>;
#[doc = "Slave Mode Register"]
pub mod twihs_smr;
#[doc = "TWIHS_IADR register accessor: an alias for `Reg<TWIHS_IADR_SPEC>`"]
pub type TWIHS_IADR = crate::Reg<twihs_iadr::TWIHS_IADR_SPEC>;
#[doc = "Internal Address Register"]
pub mod twihs_iadr;
#[doc = "TWIHS_CWGR register accessor: an alias for `Reg<TWIHS_CWGR_SPEC>`"]
pub type TWIHS_CWGR = crate::Reg<twihs_cwgr::TWIHS_CWGR_SPEC>;
#[doc = "Clock Waveform Generator Register"]
pub mod twihs_cwgr;
#[doc = "TWIHS_SR register accessor: an alias for `Reg<TWIHS_SR_SPEC>`"]
pub type TWIHS_SR = crate::Reg<twihs_sr::TWIHS_SR_SPEC>;
#[doc = "Status Register"]
pub mod twihs_sr;
#[doc = "TWIHS_IER register accessor: an alias for `Reg<TWIHS_IER_SPEC>`"]
pub type TWIHS_IER = crate::Reg<twihs_ier::TWIHS_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod twihs_ier;
#[doc = "TWIHS_IDR register accessor: an alias for `Reg<TWIHS_IDR_SPEC>`"]
pub type TWIHS_IDR = crate::Reg<twihs_idr::TWIHS_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod twihs_idr;
#[doc = "TWIHS_IMR register accessor: an alias for `Reg<TWIHS_IMR_SPEC>`"]
pub type TWIHS_IMR = crate::Reg<twihs_imr::TWIHS_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod twihs_imr;
#[doc = "TWIHS_RHR register accessor: an alias for `Reg<TWIHS_RHR_SPEC>`"]
pub type TWIHS_RHR = crate::Reg<twihs_rhr::TWIHS_RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod twihs_rhr;
#[doc = "TWIHS_THR register accessor: an alias for `Reg<TWIHS_THR_SPEC>`"]
pub type TWIHS_THR = crate::Reg<twihs_thr::TWIHS_THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod twihs_thr;
#[doc = "TWIHS_SMBTR register accessor: an alias for `Reg<TWIHS_SMBTR_SPEC>`"]
pub type TWIHS_SMBTR = crate::Reg<twihs_smbtr::TWIHS_SMBTR_SPEC>;
#[doc = "SMBus Timing Register"]
pub mod twihs_smbtr;
#[doc = "TWIHS_FILTR register accessor: an alias for `Reg<TWIHS_FILTR_SPEC>`"]
pub type TWIHS_FILTR = crate::Reg<twihs_filtr::TWIHS_FILTR_SPEC>;
#[doc = "Filter Register"]
pub mod twihs_filtr;
#[doc = "TWIHS_SWMR register accessor: an alias for `Reg<TWIHS_SWMR_SPEC>`"]
pub type TWIHS_SWMR = crate::Reg<twihs_swmr::TWIHS_SWMR_SPEC>;
#[doc = "SleepWalking Matching Register"]
pub mod twihs_swmr;
#[doc = "TWIHS_WPMR register accessor: an alias for `Reg<TWIHS_WPMR_SPEC>`"]
pub type TWIHS_WPMR = crate::Reg<twihs_wpmr::TWIHS_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod twihs_wpmr;
#[doc = "TWIHS_WPSR register accessor: an alias for `Reg<TWIHS_WPSR_SPEC>`"]
pub type TWIHS_WPSR = crate::Reg<twihs_wpsr::TWIHS_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod twihs_wpsr;
