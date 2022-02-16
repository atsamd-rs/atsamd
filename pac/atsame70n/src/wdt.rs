#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub wdt_cr: crate::Reg<wdt_cr::WDT_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub wdt_mr: crate::Reg<wdt_mr::WDT_MR_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub wdt_sr: crate::Reg<wdt_sr::WDT_SR_SPEC>,
}
#[doc = "WDT_CR register accessor: an alias for `Reg<WDT_CR_SPEC>`"]
pub type WDT_CR = crate::Reg<wdt_cr::WDT_CR_SPEC>;
#[doc = "Control Register"]
pub mod wdt_cr;
#[doc = "WDT_MR register accessor: an alias for `Reg<WDT_MR_SPEC>`"]
pub type WDT_MR = crate::Reg<wdt_mr::WDT_MR_SPEC>;
#[doc = "Mode Register"]
pub mod wdt_mr;
#[doc = "WDT_SR register accessor: an alias for `Reg<WDT_SR_SPEC>`"]
pub type WDT_SR = crate::Reg<wdt_sr::WDT_SR_SPEC>;
#[doc = "Status Register"]
pub mod wdt_sr;
