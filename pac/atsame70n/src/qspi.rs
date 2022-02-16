#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub qspi_cr: crate::Reg<qspi_cr::QSPI_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub qspi_mr: crate::Reg<qspi_mr::QSPI_MR_SPEC>,
    #[doc = "0x08 - Receive Data Register"]
    pub qspi_rdr: crate::Reg<qspi_rdr::QSPI_RDR_SPEC>,
    #[doc = "0x0c - Transmit Data Register"]
    pub qspi_tdr: crate::Reg<qspi_tdr::QSPI_TDR_SPEC>,
    #[doc = "0x10 - Status Register"]
    pub qspi_sr: crate::Reg<qspi_sr::QSPI_SR_SPEC>,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub qspi_ier: crate::Reg<qspi_ier::QSPI_IER_SPEC>,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub qspi_idr: crate::Reg<qspi_idr::QSPI_IDR_SPEC>,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub qspi_imr: crate::Reg<qspi_imr::QSPI_IMR_SPEC>,
    #[doc = "0x20 - Serial Clock Register"]
    pub qspi_scr: crate::Reg<qspi_scr::QSPI_SCR_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - Instruction Address Register"]
    pub qspi_iar: crate::Reg<qspi_iar::QSPI_IAR_SPEC>,
    #[doc = "0x34 - Instruction Code Register"]
    pub qspi_icr: crate::Reg<qspi_icr::QSPI_ICR_SPEC>,
    #[doc = "0x38 - Instruction Frame Register"]
    pub qspi_ifr: crate::Reg<qspi_ifr::QSPI_IFR_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - Scrambling Mode Register"]
    pub qspi_smr: crate::Reg<qspi_smr::QSPI_SMR_SPEC>,
    #[doc = "0x44 - Scrambling Key Register"]
    pub qspi_skr: crate::Reg<qspi_skr::QSPI_SKR_SPEC>,
    _reserved14: [u8; 0x9c],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub qspi_wpmr: crate::Reg<qspi_wpmr::QSPI_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub qspi_wpsr: crate::Reg<qspi_wpsr::QSPI_WPSR_SPEC>,
}
#[doc = "QSPI_CR register accessor: an alias for `Reg<QSPI_CR_SPEC>`"]
pub type QSPI_CR = crate::Reg<qspi_cr::QSPI_CR_SPEC>;
#[doc = "Control Register"]
pub mod qspi_cr;
#[doc = "QSPI_MR register accessor: an alias for `Reg<QSPI_MR_SPEC>`"]
pub type QSPI_MR = crate::Reg<qspi_mr::QSPI_MR_SPEC>;
#[doc = "Mode Register"]
pub mod qspi_mr;
#[doc = "QSPI_RDR register accessor: an alias for `Reg<QSPI_RDR_SPEC>`"]
pub type QSPI_RDR = crate::Reg<qspi_rdr::QSPI_RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod qspi_rdr;
#[doc = "QSPI_TDR register accessor: an alias for `Reg<QSPI_TDR_SPEC>`"]
pub type QSPI_TDR = crate::Reg<qspi_tdr::QSPI_TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod qspi_tdr;
#[doc = "QSPI_SR register accessor: an alias for `Reg<QSPI_SR_SPEC>`"]
pub type QSPI_SR = crate::Reg<qspi_sr::QSPI_SR_SPEC>;
#[doc = "Status Register"]
pub mod qspi_sr;
#[doc = "QSPI_IER register accessor: an alias for `Reg<QSPI_IER_SPEC>`"]
pub type QSPI_IER = crate::Reg<qspi_ier::QSPI_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod qspi_ier;
#[doc = "QSPI_IDR register accessor: an alias for `Reg<QSPI_IDR_SPEC>`"]
pub type QSPI_IDR = crate::Reg<qspi_idr::QSPI_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod qspi_idr;
#[doc = "QSPI_IMR register accessor: an alias for `Reg<QSPI_IMR_SPEC>`"]
pub type QSPI_IMR = crate::Reg<qspi_imr::QSPI_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod qspi_imr;
#[doc = "QSPI_SCR register accessor: an alias for `Reg<QSPI_SCR_SPEC>`"]
pub type QSPI_SCR = crate::Reg<qspi_scr::QSPI_SCR_SPEC>;
#[doc = "Serial Clock Register"]
pub mod qspi_scr;
#[doc = "QSPI_IAR register accessor: an alias for `Reg<QSPI_IAR_SPEC>`"]
pub type QSPI_IAR = crate::Reg<qspi_iar::QSPI_IAR_SPEC>;
#[doc = "Instruction Address Register"]
pub mod qspi_iar;
#[doc = "QSPI_ICR register accessor: an alias for `Reg<QSPI_ICR_SPEC>`"]
pub type QSPI_ICR = crate::Reg<qspi_icr::QSPI_ICR_SPEC>;
#[doc = "Instruction Code Register"]
pub mod qspi_icr;
#[doc = "QSPI_IFR register accessor: an alias for `Reg<QSPI_IFR_SPEC>`"]
pub type QSPI_IFR = crate::Reg<qspi_ifr::QSPI_IFR_SPEC>;
#[doc = "Instruction Frame Register"]
pub mod qspi_ifr;
#[doc = "QSPI_SMR register accessor: an alias for `Reg<QSPI_SMR_SPEC>`"]
pub type QSPI_SMR = crate::Reg<qspi_smr::QSPI_SMR_SPEC>;
#[doc = "Scrambling Mode Register"]
pub mod qspi_smr;
#[doc = "QSPI_SKR register accessor: an alias for `Reg<QSPI_SKR_SPEC>`"]
pub type QSPI_SKR = crate::Reg<qspi_skr::QSPI_SKR_SPEC>;
#[doc = "Scrambling Key Register"]
pub mod qspi_skr;
#[doc = "QSPI_WPMR register accessor: an alias for `Reg<QSPI_WPMR_SPEC>`"]
pub type QSPI_WPMR = crate::Reg<qspi_wpmr::QSPI_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod qspi_wpmr;
#[doc = "QSPI_WPSR register accessor: an alias for `Reg<QSPI_WPSR_SPEC>`"]
pub type QSPI_WPSR = crate::Reg<qspi_wpsr::QSPI_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod qspi_wpsr;
