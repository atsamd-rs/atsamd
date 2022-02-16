#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supply Controller Control Register"]
    pub supc_cr: crate::Reg<supc_cr::SUPC_CR_SPEC>,
    #[doc = "0x04 - Supply Controller Supply Monitor Mode Register"]
    pub supc_smmr: crate::Reg<supc_smmr::SUPC_SMMR_SPEC>,
    #[doc = "0x08 - Supply Controller Mode Register"]
    pub supc_mr: crate::Reg<supc_mr::SUPC_MR_SPEC>,
    #[doc = "0x0c - Supply Controller Wake-up Mode Register"]
    pub supc_wumr: crate::Reg<supc_wumr::SUPC_WUMR_SPEC>,
    #[doc = "0x10 - Supply Controller Wake-up Inputs Register"]
    pub supc_wuir: crate::Reg<supc_wuir::SUPC_WUIR_SPEC>,
    #[doc = "0x14 - Supply Controller Status Register"]
    pub supc_sr: crate::Reg<supc_sr::SUPC_SR_SPEC>,
}
#[doc = "SUPC_CR register accessor: an alias for `Reg<SUPC_CR_SPEC>`"]
pub type SUPC_CR = crate::Reg<supc_cr::SUPC_CR_SPEC>;
#[doc = "Supply Controller Control Register"]
pub mod supc_cr;
#[doc = "SUPC_SMMR register accessor: an alias for `Reg<SUPC_SMMR_SPEC>`"]
pub type SUPC_SMMR = crate::Reg<supc_smmr::SUPC_SMMR_SPEC>;
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub mod supc_smmr;
#[doc = "SUPC_MR register accessor: an alias for `Reg<SUPC_MR_SPEC>`"]
pub type SUPC_MR = crate::Reg<supc_mr::SUPC_MR_SPEC>;
#[doc = "Supply Controller Mode Register"]
pub mod supc_mr;
#[doc = "SUPC_WUMR register accessor: an alias for `Reg<SUPC_WUMR_SPEC>`"]
pub type SUPC_WUMR = crate::Reg<supc_wumr::SUPC_WUMR_SPEC>;
#[doc = "Supply Controller Wake-up Mode Register"]
pub mod supc_wumr;
#[doc = "SUPC_WUIR register accessor: an alias for `Reg<SUPC_WUIR_SPEC>`"]
pub type SUPC_WUIR = crate::Reg<supc_wuir::SUPC_WUIR_SPEC>;
#[doc = "Supply Controller Wake-up Inputs Register"]
pub mod supc_wuir;
#[doc = "SUPC_SR register accessor: an alias for `Reg<SUPC_SR_SPEC>`"]
pub type SUPC_SR = crate::Reg<supc_sr::SUPC_SR_SPEC>;
#[doc = "Supply Controller Status Register"]
pub mod supc_sr;
