#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub i2sc_cr: crate::Reg<i2sc_cr::I2SC_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub i2sc_mr: crate::Reg<i2sc_mr::I2SC_MR_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub i2sc_sr: crate::Reg<i2sc_sr::I2SC_SR_SPEC>,
    #[doc = "0x0c - Status Clear Register"]
    pub i2sc_scr: crate::Reg<i2sc_scr::I2SC_SCR_SPEC>,
    #[doc = "0x10 - Status Set Register"]
    pub i2sc_ssr: crate::Reg<i2sc_ssr::I2SC_SSR_SPEC>,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub i2sc_ier: crate::Reg<i2sc_ier::I2SC_IER_SPEC>,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub i2sc_idr: crate::Reg<i2sc_idr::I2SC_IDR_SPEC>,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub i2sc_imr: crate::Reg<i2sc_imr::I2SC_IMR_SPEC>,
    #[doc = "0x20 - Receiver Holding Register"]
    pub i2sc_rhr: crate::Reg<i2sc_rhr::I2SC_RHR_SPEC>,
    #[doc = "0x24 - Transmitter Holding Register"]
    pub i2sc_thr: crate::Reg<i2sc_thr::I2SC_THR_SPEC>,
}
#[doc = "I2SC_CR register accessor: an alias for `Reg<I2SC_CR_SPEC>`"]
pub type I2SC_CR = crate::Reg<i2sc_cr::I2SC_CR_SPEC>;
#[doc = "Control Register"]
pub mod i2sc_cr;
#[doc = "I2SC_MR register accessor: an alias for `Reg<I2SC_MR_SPEC>`"]
pub type I2SC_MR = crate::Reg<i2sc_mr::I2SC_MR_SPEC>;
#[doc = "Mode Register"]
pub mod i2sc_mr;
#[doc = "I2SC_SR register accessor: an alias for `Reg<I2SC_SR_SPEC>`"]
pub type I2SC_SR = crate::Reg<i2sc_sr::I2SC_SR_SPEC>;
#[doc = "Status Register"]
pub mod i2sc_sr;
#[doc = "I2SC_SCR register accessor: an alias for `Reg<I2SC_SCR_SPEC>`"]
pub type I2SC_SCR = crate::Reg<i2sc_scr::I2SC_SCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod i2sc_scr;
#[doc = "I2SC_SSR register accessor: an alias for `Reg<I2SC_SSR_SPEC>`"]
pub type I2SC_SSR = crate::Reg<i2sc_ssr::I2SC_SSR_SPEC>;
#[doc = "Status Set Register"]
pub mod i2sc_ssr;
#[doc = "I2SC_IER register accessor: an alias for `Reg<I2SC_IER_SPEC>`"]
pub type I2SC_IER = crate::Reg<i2sc_ier::I2SC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod i2sc_ier;
#[doc = "I2SC_IDR register accessor: an alias for `Reg<I2SC_IDR_SPEC>`"]
pub type I2SC_IDR = crate::Reg<i2sc_idr::I2SC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod i2sc_idr;
#[doc = "I2SC_IMR register accessor: an alias for `Reg<I2SC_IMR_SPEC>`"]
pub type I2SC_IMR = crate::Reg<i2sc_imr::I2SC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod i2sc_imr;
#[doc = "I2SC_RHR register accessor: an alias for `Reg<I2SC_RHR_SPEC>`"]
pub type I2SC_RHR = crate::Reg<i2sc_rhr::I2SC_RHR_SPEC>;
#[doc = "Receiver Holding Register"]
pub mod i2sc_rhr;
#[doc = "I2SC_THR register accessor: an alias for `Reg<I2SC_THR_SPEC>`"]
pub type I2SC_THR = crate::Reg<i2sc_thr::I2SC_THR_SPEC>;
#[doc = "Transmitter Holding Register"]
pub mod i2sc_thr;
