#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub rstc_cr: crate::Reg<rstc_cr::RSTC_CR_SPEC>,
    #[doc = "0x04 - Status Register"]
    pub rstc_sr: crate::Reg<rstc_sr::RSTC_SR_SPEC>,
    #[doc = "0x08 - Mode Register"]
    pub rstc_mr: crate::Reg<rstc_mr::RSTC_MR_SPEC>,
}
#[doc = "RSTC_CR register accessor: an alias for `Reg<RSTC_CR_SPEC>`"]
pub type RSTC_CR = crate::Reg<rstc_cr::RSTC_CR_SPEC>;
#[doc = "Control Register"]
pub mod rstc_cr;
#[doc = "RSTC_SR register accessor: an alias for `Reg<RSTC_SR_SPEC>`"]
pub type RSTC_SR = crate::Reg<rstc_sr::RSTC_SR_SPEC>;
#[doc = "Status Register"]
pub mod rstc_sr;
#[doc = "RSTC_MR register accessor: an alias for `Reg<RSTC_MR_SPEC>`"]
pub type RSTC_MR = crate::Reg<rstc_mr::RSTC_MR_SPEC>;
#[doc = "Mode Register"]
pub mod rstc_mr;
