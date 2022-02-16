#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub uart_cr: crate::Reg<uart_cr::UART_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub uart_mr: crate::Reg<uart_mr::UART_MR_SPEC>,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub uart_ier: crate::Reg<uart_ier::UART_IER_SPEC>,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub uart_idr: crate::Reg<uart_idr::UART_IDR_SPEC>,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub uart_imr: crate::Reg<uart_imr::UART_IMR_SPEC>,
    #[doc = "0x14 - Status Register"]
    pub uart_sr: crate::Reg<uart_sr::UART_SR_SPEC>,
    #[doc = "0x18 - Receive Holding Register"]
    pub uart_rhr: crate::Reg<uart_rhr::UART_RHR_SPEC>,
    #[doc = "0x1c - Transmit Holding Register"]
    pub uart_thr: crate::Reg<uart_thr::UART_THR_SPEC>,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub uart_brgr: crate::Reg<uart_brgr::UART_BRGR_SPEC>,
    #[doc = "0x24 - Comparison Register"]
    pub uart_cmpr: crate::Reg<uart_cmpr::UART_CMPR_SPEC>,
    _reserved10: [u8; 0xbc],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub uart_wpmr: crate::Reg<uart_wpmr::UART_WPMR_SPEC>,
}
#[doc = "UART_CR register accessor: an alias for `Reg<UART_CR_SPEC>`"]
pub type UART_CR = crate::Reg<uart_cr::UART_CR_SPEC>;
#[doc = "Control Register"]
pub mod uart_cr;
#[doc = "UART_MR register accessor: an alias for `Reg<UART_MR_SPEC>`"]
pub type UART_MR = crate::Reg<uart_mr::UART_MR_SPEC>;
#[doc = "Mode Register"]
pub mod uart_mr;
#[doc = "UART_IER register accessor: an alias for `Reg<UART_IER_SPEC>`"]
pub type UART_IER = crate::Reg<uart_ier::UART_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod uart_ier;
#[doc = "UART_IDR register accessor: an alias for `Reg<UART_IDR_SPEC>`"]
pub type UART_IDR = crate::Reg<uart_idr::UART_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod uart_idr;
#[doc = "UART_IMR register accessor: an alias for `Reg<UART_IMR_SPEC>`"]
pub type UART_IMR = crate::Reg<uart_imr::UART_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod uart_imr;
#[doc = "UART_SR register accessor: an alias for `Reg<UART_SR_SPEC>`"]
pub type UART_SR = crate::Reg<uart_sr::UART_SR_SPEC>;
#[doc = "Status Register"]
pub mod uart_sr;
#[doc = "UART_RHR register accessor: an alias for `Reg<UART_RHR_SPEC>`"]
pub type UART_RHR = crate::Reg<uart_rhr::UART_RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod uart_rhr;
#[doc = "UART_THR register accessor: an alias for `Reg<UART_THR_SPEC>`"]
pub type UART_THR = crate::Reg<uart_thr::UART_THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod uart_thr;
#[doc = "UART_BRGR register accessor: an alias for `Reg<UART_BRGR_SPEC>`"]
pub type UART_BRGR = crate::Reg<uart_brgr::UART_BRGR_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod uart_brgr;
#[doc = "UART_CMPR register accessor: an alias for `Reg<UART_CMPR_SPEC>`"]
pub type UART_CMPR = crate::Reg<uart_cmpr::UART_CMPR_SPEC>;
#[doc = "Comparison Register"]
pub mod uart_cmpr;
#[doc = "UART_WPMR register accessor: an alias for `Reg<UART_WPMR_SPEC>`"]
pub type UART_WPMR = crate::Reg<uart_wpmr::UART_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod uart_wpmr;
