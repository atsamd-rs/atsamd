#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub dacc_cr: crate::Reg<dacc_cr::DACC_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub dacc_mr: crate::Reg<dacc_mr::DACC_MR_SPEC>,
    #[doc = "0x08 - Trigger Register"]
    pub dacc_trigr: crate::Reg<dacc_trigr::DACC_TRIGR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Channel Enable Register"]
    pub dacc_cher: crate::Reg<dacc_cher::DACC_CHER_SPEC>,
    #[doc = "0x14 - Channel Disable Register"]
    pub dacc_chdr: crate::Reg<dacc_chdr::DACC_CHDR_SPEC>,
    #[doc = "0x18 - Channel Status Register"]
    pub dacc_chsr: crate::Reg<dacc_chsr::DACC_CHSR_SPEC>,
    #[doc = "0x1c..0x24 - Conversion Data Register 0"]
    pub dacc_cdr: [crate::Reg<dacc_cdr::DACC_CDR_SPEC>; 2],
    #[doc = "0x24 - Interrupt Enable Register"]
    pub dacc_ier: crate::Reg<dacc_ier::DACC_IER_SPEC>,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub dacc_idr: crate::Reg<dacc_idr::DACC_IDR_SPEC>,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub dacc_imr: crate::Reg<dacc_imr::DACC_IMR_SPEC>,
    #[doc = "0x30 - Interrupt Status Register"]
    pub dacc_isr: crate::Reg<dacc_isr::DACC_ISR_SPEC>,
    _reserved11: [u8; 0x60],
    #[doc = "0x94 - Analog Current Register"]
    pub dacc_acr: crate::Reg<dacc_acr::DACC_ACR_SPEC>,
    _reserved12: [u8; 0x4c],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub dacc_wpmr: crate::Reg<dacc_wpmr::DACC_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub dacc_wpsr: crate::Reg<dacc_wpsr::DACC_WPSR_SPEC>,
}
#[doc = "DACC_CR register accessor: an alias for `Reg<DACC_CR_SPEC>`"]
pub type DACC_CR = crate::Reg<dacc_cr::DACC_CR_SPEC>;
#[doc = "Control Register"]
pub mod dacc_cr;
#[doc = "DACC_MR register accessor: an alias for `Reg<DACC_MR_SPEC>`"]
pub type DACC_MR = crate::Reg<dacc_mr::DACC_MR_SPEC>;
#[doc = "Mode Register"]
pub mod dacc_mr;
#[doc = "DACC_TRIGR register accessor: an alias for `Reg<DACC_TRIGR_SPEC>`"]
pub type DACC_TRIGR = crate::Reg<dacc_trigr::DACC_TRIGR_SPEC>;
#[doc = "Trigger Register"]
pub mod dacc_trigr;
#[doc = "DACC_CHER register accessor: an alias for `Reg<DACC_CHER_SPEC>`"]
pub type DACC_CHER = crate::Reg<dacc_cher::DACC_CHER_SPEC>;
#[doc = "Channel Enable Register"]
pub mod dacc_cher;
#[doc = "DACC_CHDR register accessor: an alias for `Reg<DACC_CHDR_SPEC>`"]
pub type DACC_CHDR = crate::Reg<dacc_chdr::DACC_CHDR_SPEC>;
#[doc = "Channel Disable Register"]
pub mod dacc_chdr;
#[doc = "DACC_CHSR register accessor: an alias for `Reg<DACC_CHSR_SPEC>`"]
pub type DACC_CHSR = crate::Reg<dacc_chsr::DACC_CHSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod dacc_chsr;
#[doc = "DACC_CDR register accessor: an alias for `Reg<DACC_CDR_SPEC>`"]
pub type DACC_CDR = crate::Reg<dacc_cdr::DACC_CDR_SPEC>;
#[doc = "Conversion Data Register 0"]
pub mod dacc_cdr;
#[doc = "DACC_IER register accessor: an alias for `Reg<DACC_IER_SPEC>`"]
pub type DACC_IER = crate::Reg<dacc_ier::DACC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod dacc_ier;
#[doc = "DACC_IDR register accessor: an alias for `Reg<DACC_IDR_SPEC>`"]
pub type DACC_IDR = crate::Reg<dacc_idr::DACC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod dacc_idr;
#[doc = "DACC_IMR register accessor: an alias for `Reg<DACC_IMR_SPEC>`"]
pub type DACC_IMR = crate::Reg<dacc_imr::DACC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod dacc_imr;
#[doc = "DACC_ISR register accessor: an alias for `Reg<DACC_ISR_SPEC>`"]
pub type DACC_ISR = crate::Reg<dacc_isr::DACC_ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod dacc_isr;
#[doc = "DACC_ACR register accessor: an alias for `Reg<DACC_ACR_SPEC>`"]
pub type DACC_ACR = crate::Reg<dacc_acr::DACC_ACR_SPEC>;
#[doc = "Analog Current Register"]
pub mod dacc_acr;
#[doc = "DACC_WPMR register accessor: an alias for `Reg<DACC_WPMR_SPEC>`"]
pub type DACC_WPMR = crate::Reg<dacc_wpmr::DACC_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod dacc_wpmr;
#[doc = "DACC_WPSR register accessor: an alias for `Reg<DACC_WPSR_SPEC>`"]
pub type DACC_WPSR = crate::Reg<dacc_wpsr::DACC_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod dacc_wpsr;
