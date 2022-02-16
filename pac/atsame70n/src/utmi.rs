#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - OHCI Interrupt Configuration Register"]
    pub utmi_ohciicr: crate::Reg<utmi_ohciicr::UTMI_OHCIICR_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x30 - UTMI Clock Trimming Register"]
    pub utmi_cktrim: crate::Reg<utmi_cktrim::UTMI_CKTRIM_SPEC>,
}
#[doc = "UTMI_OHCIICR register accessor: an alias for `Reg<UTMI_OHCIICR_SPEC>`"]
pub type UTMI_OHCIICR = crate::Reg<utmi_ohciicr::UTMI_OHCIICR_SPEC>;
#[doc = "OHCI Interrupt Configuration Register"]
pub mod utmi_ohciicr;
#[doc = "UTMI_CKTRIM register accessor: an alias for `Reg<UTMI_CKTRIM_SPEC>`"]
pub type UTMI_CKTRIM = crate::Reg<utmi_cktrim::UTMI_CKTRIM_SPEC>;
#[doc = "UTMI Clock Trimming Register"]
pub mod utmi_cktrim;
