#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub acc_cr: crate::Reg<acc_cr::ACC_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub acc_mr: crate::Reg<acc_mr::ACC_MR_SPEC>,
    _reserved2: [u8; 0x1c],
    #[doc = "0x24 - Interrupt Enable Register"]
    pub acc_ier: crate::Reg<acc_ier::ACC_IER_SPEC>,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub acc_idr: crate::Reg<acc_idr::ACC_IDR_SPEC>,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub acc_imr: crate::Reg<acc_imr::ACC_IMR_SPEC>,
    #[doc = "0x30 - Interrupt Status Register"]
    pub acc_isr: crate::Reg<acc_isr::ACC_ISR_SPEC>,
    _reserved6: [u8; 0x60],
    #[doc = "0x94 - Analog Control Register"]
    pub acc_acr: crate::Reg<acc_acr::ACC_ACR_SPEC>,
    _reserved7: [u8; 0x4c],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub acc_wpmr: crate::Reg<acc_wpmr::ACC_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub acc_wpsr: crate::Reg<acc_wpsr::ACC_WPSR_SPEC>,
}
#[doc = "ACC_CR register accessor: an alias for `Reg<ACC_CR_SPEC>`"]
pub type ACC_CR = crate::Reg<acc_cr::ACC_CR_SPEC>;
#[doc = "Control Register"]
pub mod acc_cr;
#[doc = "ACC_MR register accessor: an alias for `Reg<ACC_MR_SPEC>`"]
pub type ACC_MR = crate::Reg<acc_mr::ACC_MR_SPEC>;
#[doc = "Mode Register"]
pub mod acc_mr;
#[doc = "ACC_IER register accessor: an alias for `Reg<ACC_IER_SPEC>`"]
pub type ACC_IER = crate::Reg<acc_ier::ACC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod acc_ier;
#[doc = "ACC_IDR register accessor: an alias for `Reg<ACC_IDR_SPEC>`"]
pub type ACC_IDR = crate::Reg<acc_idr::ACC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod acc_idr;
#[doc = "ACC_IMR register accessor: an alias for `Reg<ACC_IMR_SPEC>`"]
pub type ACC_IMR = crate::Reg<acc_imr::ACC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod acc_imr;
#[doc = "ACC_ISR register accessor: an alias for `Reg<ACC_ISR_SPEC>`"]
pub type ACC_ISR = crate::Reg<acc_isr::ACC_ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod acc_isr;
#[doc = "ACC_ACR register accessor: an alias for `Reg<ACC_ACR_SPEC>`"]
pub type ACC_ACR = crate::Reg<acc_acr::ACC_ACR_SPEC>;
#[doc = "Analog Control Register"]
pub mod acc_acr;
#[doc = "ACC_WPMR register accessor: an alias for `Reg<ACC_WPMR_SPEC>`"]
pub type ACC_WPMR = crate::Reg<acc_wpmr::ACC_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod acc_wpmr;
#[doc = "ACC_WPSR register accessor: an alias for `Reg<ACC_WPSR_SPEC>`"]
pub type ACC_WPSR = crate::Reg<acc_wpsr::ACC_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod acc_wpsr;
