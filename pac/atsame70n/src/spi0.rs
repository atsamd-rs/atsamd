#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub spi_cr: crate::Reg<spi_cr::SPI_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub spi_mr: crate::Reg<spi_mr::SPI_MR_SPEC>,
    #[doc = "0x08 - Receive Data Register"]
    pub spi_rdr: crate::Reg<spi_rdr::SPI_RDR_SPEC>,
    #[doc = "0x0c - Transmit Data Register"]
    pub spi_tdr: crate::Reg<spi_tdr::SPI_TDR_SPEC>,
    #[doc = "0x10 - Status Register"]
    pub spi_sr: crate::Reg<spi_sr::SPI_SR_SPEC>,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub spi_ier: crate::Reg<spi_ier::SPI_IER_SPEC>,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub spi_idr: crate::Reg<spi_idr::SPI_IDR_SPEC>,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub spi_imr: crate::Reg<spi_imr::SPI_IMR_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x30..0x40 - Chip Select Register"]
    pub spi_csr: [crate::Reg<spi_csr::SPI_CSR_SPEC>; 4],
    _reserved9: [u8; 0xa4],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub spi_wpmr: crate::Reg<spi_wpmr::SPI_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub spi_wpsr: crate::Reg<spi_wpsr::SPI_WPSR_SPEC>,
}
#[doc = "SPI_CR register accessor: an alias for `Reg<SPI_CR_SPEC>`"]
pub type SPI_CR = crate::Reg<spi_cr::SPI_CR_SPEC>;
#[doc = "Control Register"]
pub mod spi_cr;
#[doc = "SPI_MR register accessor: an alias for `Reg<SPI_MR_SPEC>`"]
pub type SPI_MR = crate::Reg<spi_mr::SPI_MR_SPEC>;
#[doc = "Mode Register"]
pub mod spi_mr;
#[doc = "SPI_RDR register accessor: an alias for `Reg<SPI_RDR_SPEC>`"]
pub type SPI_RDR = crate::Reg<spi_rdr::SPI_RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod spi_rdr;
#[doc = "SPI_TDR register accessor: an alias for `Reg<SPI_TDR_SPEC>`"]
pub type SPI_TDR = crate::Reg<spi_tdr::SPI_TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod spi_tdr;
#[doc = "SPI_SR register accessor: an alias for `Reg<SPI_SR_SPEC>`"]
pub type SPI_SR = crate::Reg<spi_sr::SPI_SR_SPEC>;
#[doc = "Status Register"]
pub mod spi_sr;
#[doc = "SPI_IER register accessor: an alias for `Reg<SPI_IER_SPEC>`"]
pub type SPI_IER = crate::Reg<spi_ier::SPI_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod spi_ier;
#[doc = "SPI_IDR register accessor: an alias for `Reg<SPI_IDR_SPEC>`"]
pub type SPI_IDR = crate::Reg<spi_idr::SPI_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod spi_idr;
#[doc = "SPI_IMR register accessor: an alias for `Reg<SPI_IMR_SPEC>`"]
pub type SPI_IMR = crate::Reg<spi_imr::SPI_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod spi_imr;
#[doc = "SPI_CSR register accessor: an alias for `Reg<SPI_CSR_SPEC>`"]
pub type SPI_CSR = crate::Reg<spi_csr::SPI_CSR_SPEC>;
#[doc = "Chip Select Register"]
pub mod spi_csr;
#[doc = "SPI_WPMR register accessor: an alias for `Reg<SPI_WPMR_SPEC>`"]
pub type SPI_WPMR = crate::Reg<spi_wpmr::SPI_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod spi_wpmr;
#[doc = "SPI_WPSR register accessor: an alias for `Reg<SPI_WPSR_SPEC>`"]
pub type SPI_WPSR = crate::Reg<spi_wpsr::SPI_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod spi_wpsr;
