#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub hsmci_cr: crate::Reg<hsmci_cr::HSMCI_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub hsmci_mr: crate::Reg<hsmci_mr::HSMCI_MR_SPEC>,
    #[doc = "0x08 - Data Timeout Register"]
    pub hsmci_dtor: crate::Reg<hsmci_dtor::HSMCI_DTOR_SPEC>,
    #[doc = "0x0c - SD/SDIO Card Register"]
    pub hsmci_sdcr: crate::Reg<hsmci_sdcr::HSMCI_SDCR_SPEC>,
    #[doc = "0x10 - Argument Register"]
    pub hsmci_argr: crate::Reg<hsmci_argr::HSMCI_ARGR_SPEC>,
    #[doc = "0x14 - Command Register"]
    pub hsmci_cmdr: crate::Reg<hsmci_cmdr::HSMCI_CMDR_SPEC>,
    #[doc = "0x18 - Block Register"]
    pub hsmci_blkr: crate::Reg<hsmci_blkr::HSMCI_BLKR_SPEC>,
    #[doc = "0x1c - Completion Signal Timeout Register"]
    pub hsmci_cstor: crate::Reg<hsmci_cstor::HSMCI_CSTOR_SPEC>,
    #[doc = "0x20..0x30 - Response Register 0"]
    pub hsmci_rspr: [crate::Reg<hsmci_rspr::HSMCI_RSPR_SPEC>; 4],
    #[doc = "0x30 - Receive Data Register"]
    pub hsmci_rdr: crate::Reg<hsmci_rdr::HSMCI_RDR_SPEC>,
    #[doc = "0x34 - Transmit Data Register"]
    pub hsmci_tdr: crate::Reg<hsmci_tdr::HSMCI_TDR_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x40 - Status Register"]
    pub hsmci_sr: crate::Reg<hsmci_sr::HSMCI_SR_SPEC>,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub hsmci_ier: crate::Reg<hsmci_ier::HSMCI_IER_SPEC>,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub hsmci_idr: crate::Reg<hsmci_idr::HSMCI_IDR_SPEC>,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub hsmci_imr: crate::Reg<hsmci_imr::HSMCI_IMR_SPEC>,
    #[doc = "0x50 - DMA Configuration Register"]
    pub hsmci_dma: crate::Reg<hsmci_dma::HSMCI_DMA_SPEC>,
    #[doc = "0x54 - Configuration Register"]
    pub hsmci_cfg: crate::Reg<hsmci_cfg::HSMCI_CFG_SPEC>,
    _reserved17: [u8; 0x8c],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub hsmci_wpmr: crate::Reg<hsmci_wpmr::HSMCI_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub hsmci_wpsr: crate::Reg<hsmci_wpsr::HSMCI_WPSR_SPEC>,
    _reserved19: [u8; 0x0114],
    #[doc = "0x200..0x600 - FIFO Memory Aperture0 0"]
    pub hsmci_fifo: [crate::Reg<hsmci_fifo::HSMCI_FIFO_SPEC>; 256],
}
#[doc = "HSMCI_CR register accessor: an alias for `Reg<HSMCI_CR_SPEC>`"]
pub type HSMCI_CR = crate::Reg<hsmci_cr::HSMCI_CR_SPEC>;
#[doc = "Control Register"]
pub mod hsmci_cr;
#[doc = "HSMCI_MR register accessor: an alias for `Reg<HSMCI_MR_SPEC>`"]
pub type HSMCI_MR = crate::Reg<hsmci_mr::HSMCI_MR_SPEC>;
#[doc = "Mode Register"]
pub mod hsmci_mr;
#[doc = "HSMCI_DTOR register accessor: an alias for `Reg<HSMCI_DTOR_SPEC>`"]
pub type HSMCI_DTOR = crate::Reg<hsmci_dtor::HSMCI_DTOR_SPEC>;
#[doc = "Data Timeout Register"]
pub mod hsmci_dtor;
#[doc = "HSMCI_SDCR register accessor: an alias for `Reg<HSMCI_SDCR_SPEC>`"]
pub type HSMCI_SDCR = crate::Reg<hsmci_sdcr::HSMCI_SDCR_SPEC>;
#[doc = "SD/SDIO Card Register"]
pub mod hsmci_sdcr;
#[doc = "HSMCI_ARGR register accessor: an alias for `Reg<HSMCI_ARGR_SPEC>`"]
pub type HSMCI_ARGR = crate::Reg<hsmci_argr::HSMCI_ARGR_SPEC>;
#[doc = "Argument Register"]
pub mod hsmci_argr;
#[doc = "HSMCI_CMDR register accessor: an alias for `Reg<HSMCI_CMDR_SPEC>`"]
pub type HSMCI_CMDR = crate::Reg<hsmci_cmdr::HSMCI_CMDR_SPEC>;
#[doc = "Command Register"]
pub mod hsmci_cmdr;
#[doc = "HSMCI_BLKR register accessor: an alias for `Reg<HSMCI_BLKR_SPEC>`"]
pub type HSMCI_BLKR = crate::Reg<hsmci_blkr::HSMCI_BLKR_SPEC>;
#[doc = "Block Register"]
pub mod hsmci_blkr;
#[doc = "HSMCI_CSTOR register accessor: an alias for `Reg<HSMCI_CSTOR_SPEC>`"]
pub type HSMCI_CSTOR = crate::Reg<hsmci_cstor::HSMCI_CSTOR_SPEC>;
#[doc = "Completion Signal Timeout Register"]
pub mod hsmci_cstor;
#[doc = "HSMCI_RSPR register accessor: an alias for `Reg<HSMCI_RSPR_SPEC>`"]
pub type HSMCI_RSPR = crate::Reg<hsmci_rspr::HSMCI_RSPR_SPEC>;
#[doc = "Response Register 0"]
pub mod hsmci_rspr;
#[doc = "HSMCI_RDR register accessor: an alias for `Reg<HSMCI_RDR_SPEC>`"]
pub type HSMCI_RDR = crate::Reg<hsmci_rdr::HSMCI_RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod hsmci_rdr;
#[doc = "HSMCI_TDR register accessor: an alias for `Reg<HSMCI_TDR_SPEC>`"]
pub type HSMCI_TDR = crate::Reg<hsmci_tdr::HSMCI_TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod hsmci_tdr;
#[doc = "HSMCI_SR register accessor: an alias for `Reg<HSMCI_SR_SPEC>`"]
pub type HSMCI_SR = crate::Reg<hsmci_sr::HSMCI_SR_SPEC>;
#[doc = "Status Register"]
pub mod hsmci_sr;
#[doc = "HSMCI_IER register accessor: an alias for `Reg<HSMCI_IER_SPEC>`"]
pub type HSMCI_IER = crate::Reg<hsmci_ier::HSMCI_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod hsmci_ier;
#[doc = "HSMCI_IDR register accessor: an alias for `Reg<HSMCI_IDR_SPEC>`"]
pub type HSMCI_IDR = crate::Reg<hsmci_idr::HSMCI_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod hsmci_idr;
#[doc = "HSMCI_IMR register accessor: an alias for `Reg<HSMCI_IMR_SPEC>`"]
pub type HSMCI_IMR = crate::Reg<hsmci_imr::HSMCI_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod hsmci_imr;
#[doc = "HSMCI_DMA register accessor: an alias for `Reg<HSMCI_DMA_SPEC>`"]
pub type HSMCI_DMA = crate::Reg<hsmci_dma::HSMCI_DMA_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod hsmci_dma;
#[doc = "HSMCI_CFG register accessor: an alias for `Reg<HSMCI_CFG_SPEC>`"]
pub type HSMCI_CFG = crate::Reg<hsmci_cfg::HSMCI_CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod hsmci_cfg;
#[doc = "HSMCI_WPMR register accessor: an alias for `Reg<HSMCI_WPMR_SPEC>`"]
pub type HSMCI_WPMR = crate::Reg<hsmci_wpmr::HSMCI_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod hsmci_wpmr;
#[doc = "HSMCI_WPSR register accessor: an alias for `Reg<HSMCI_WPSR_SPEC>`"]
pub type HSMCI_WPSR = crate::Reg<hsmci_wpsr::HSMCI_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod hsmci_wpsr;
#[doc = "HSMCI_FIFO register accessor: an alias for `Reg<HSMCI_FIFO_SPEC>`"]
pub type HSMCI_FIFO = crate::Reg<hsmci_fifo::HSMCI_FIFO_SPEC>;
#[doc = "FIFO Memory Aperture0 0"]
pub mod hsmci_fifo;
