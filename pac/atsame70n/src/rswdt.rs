#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub rswdt_cr: crate::Reg<rswdt_cr::RSWDT_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub rswdt_mr: crate::Reg<rswdt_mr::RSWDT_MR_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub rswdt_sr: crate::Reg<rswdt_sr::RSWDT_SR_SPEC>,
}
#[doc = "RSWDT_CR register accessor: an alias for `Reg<RSWDT_CR_SPEC>`"]
pub type RSWDT_CR = crate::Reg<rswdt_cr::RSWDT_CR_SPEC>;
#[doc = "Control Register"]
pub mod rswdt_cr;
#[doc = "RSWDT_MR register accessor: an alias for `Reg<RSWDT_MR_SPEC>`"]
pub type RSWDT_MR = crate::Reg<rswdt_mr::RSWDT_MR_SPEC>;
#[doc = "Mode Register"]
pub mod rswdt_mr;
#[doc = "RSWDT_SR register accessor: an alias for `Reg<RSWDT_SR_SPEC>`"]
pub type RSWDT_SR = crate::Reg<rswdt_sr::RSWDT_SR_SPEC>;
#[doc = "Status Register"]
pub mod rswdt_sr;
