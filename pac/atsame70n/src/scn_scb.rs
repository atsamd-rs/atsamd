#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Interrupt Controller Type Register"]
    pub ictr: crate::Reg<ictr::ICTR_SPEC>,
    #[doc = "0x08 - Auxiliary Control Register"]
    pub actlr: crate::Reg<actlr::ACTLR_SPEC>,
}
#[doc = "ICTR register accessor: an alias for `Reg<ICTR_SPEC>`"]
pub type ICTR = crate::Reg<ictr::ICTR_SPEC>;
#[doc = "Interrupt Controller Type Register"]
pub mod ictr;
#[doc = "ACTLR register accessor: an alias for `Reg<ACTLR_SPEC>`"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Auxiliary Control Register"]
pub mod actlr;
