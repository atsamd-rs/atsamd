#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub trng_cr: crate::Reg<trng_cr::TRNG_CR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub trng_ier: crate::Reg<trng_ier::TRNG_IER_SPEC>,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub trng_idr: crate::Reg<trng_idr::TRNG_IDR_SPEC>,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub trng_imr: crate::Reg<trng_imr::TRNG_IMR_SPEC>,
    #[doc = "0x1c - Interrupt Status Register"]
    pub trng_isr: crate::Reg<trng_isr::TRNG_ISR_SPEC>,
    _reserved5: [u8; 0x30],
    #[doc = "0x50 - Output Data Register"]
    pub trng_odata: crate::Reg<trng_odata::TRNG_ODATA_SPEC>,
}
#[doc = "TRNG_CR register accessor: an alias for `Reg<TRNG_CR_SPEC>`"]
pub type TRNG_CR = crate::Reg<trng_cr::TRNG_CR_SPEC>;
#[doc = "Control Register"]
pub mod trng_cr;
#[doc = "TRNG_IER register accessor: an alias for `Reg<TRNG_IER_SPEC>`"]
pub type TRNG_IER = crate::Reg<trng_ier::TRNG_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod trng_ier;
#[doc = "TRNG_IDR register accessor: an alias for `Reg<TRNG_IDR_SPEC>`"]
pub type TRNG_IDR = crate::Reg<trng_idr::TRNG_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod trng_idr;
#[doc = "TRNG_IMR register accessor: an alias for `Reg<TRNG_IMR_SPEC>`"]
pub type TRNG_IMR = crate::Reg<trng_imr::TRNG_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod trng_imr;
#[doc = "TRNG_ISR register accessor: an alias for `Reg<TRNG_ISR_SPEC>`"]
pub type TRNG_ISR = crate::Reg<trng_isr::TRNG_ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod trng_isr;
#[doc = "TRNG_ODATA register accessor: an alias for `Reg<TRNG_ODATA_SPEC>`"]
pub type TRNG_ODATA = crate::Reg<trng_odata::TRNG_ODATA_SPEC>;
#[doc = "Output Data Register"]
pub mod trng_odata;
