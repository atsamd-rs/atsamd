#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISI Configuration 1 Register"]
    pub isi_cfg1: crate::Reg<isi_cfg1::ISI_CFG1_SPEC>,
    #[doc = "0x04 - ISI Configuration 2 Register"]
    pub isi_cfg2: crate::Reg<isi_cfg2::ISI_CFG2_SPEC>,
    #[doc = "0x08 - ISI Preview Size Register"]
    pub isi_psize: crate::Reg<isi_psize::ISI_PSIZE_SPEC>,
    #[doc = "0x0c - ISI Preview Decimation Factor Register"]
    pub isi_pdecf: crate::Reg<isi_pdecf::ISI_PDECF_SPEC>,
    #[doc = "0x10 - ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
    pub isi_y2r_set0: crate::Reg<isi_y2r_set0::ISI_Y2R_SET0_SPEC>,
    #[doc = "0x14 - ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
    pub isi_y2r_set1: crate::Reg<isi_y2r_set1::ISI_Y2R_SET1_SPEC>,
    #[doc = "0x18 - ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
    pub isi_r2y_set0: crate::Reg<isi_r2y_set0::ISI_R2Y_SET0_SPEC>,
    #[doc = "0x1c - ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
    pub isi_r2y_set1: crate::Reg<isi_r2y_set1::ISI_R2Y_SET1_SPEC>,
    #[doc = "0x20 - ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
    pub isi_r2y_set2: crate::Reg<isi_r2y_set2::ISI_R2Y_SET2_SPEC>,
    #[doc = "0x24 - ISI Control Register"]
    pub isi_cr: crate::Reg<isi_cr::ISI_CR_SPEC>,
    #[doc = "0x28 - ISI Status Register"]
    pub isi_sr: crate::Reg<isi_sr::ISI_SR_SPEC>,
    #[doc = "0x2c - ISI Interrupt Enable Register"]
    pub isi_ier: crate::Reg<isi_ier::ISI_IER_SPEC>,
    #[doc = "0x30 - ISI Interrupt Disable Register"]
    pub isi_idr: crate::Reg<isi_idr::ISI_IDR_SPEC>,
    #[doc = "0x34 - ISI Interrupt Mask Register"]
    pub isi_imr: crate::Reg<isi_imr::ISI_IMR_SPEC>,
    #[doc = "0x38 - DMA Channel Enable Register"]
    pub isi_dma_cher: crate::Reg<isi_dma_cher::ISI_DMA_CHER_SPEC>,
    #[doc = "0x3c - DMA Channel Disable Register"]
    pub isi_dma_chdr: crate::Reg<isi_dma_chdr::ISI_DMA_CHDR_SPEC>,
    #[doc = "0x40 - DMA Channel Status Register"]
    pub isi_dma_chsr: crate::Reg<isi_dma_chsr::ISI_DMA_CHSR_SPEC>,
    #[doc = "0x44 - DMA Preview Base Address Register"]
    pub isi_dma_p_addr: crate::Reg<isi_dma_p_addr::ISI_DMA_P_ADDR_SPEC>,
    #[doc = "0x48 - DMA Preview Control Register"]
    pub isi_dma_p_ctrl: crate::Reg<isi_dma_p_ctrl::ISI_DMA_P_CTRL_SPEC>,
    #[doc = "0x4c - DMA Preview Descriptor Address Register"]
    pub isi_dma_p_dscr: crate::Reg<isi_dma_p_dscr::ISI_DMA_P_DSCR_SPEC>,
    #[doc = "0x50 - DMA Codec Base Address Register"]
    pub isi_dma_c_addr: crate::Reg<isi_dma_c_addr::ISI_DMA_C_ADDR_SPEC>,
    #[doc = "0x54 - DMA Codec Control Register"]
    pub isi_dma_c_ctrl: crate::Reg<isi_dma_c_ctrl::ISI_DMA_C_CTRL_SPEC>,
    #[doc = "0x58 - DMA Codec Descriptor Address Register"]
    pub isi_dma_c_dscr: crate::Reg<isi_dma_c_dscr::ISI_DMA_C_DSCR_SPEC>,
    _reserved23: [u8; 0x88],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub isi_wpmr: crate::Reg<isi_wpmr::ISI_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub isi_wpsr: crate::Reg<isi_wpsr::ISI_WPSR_SPEC>,
}
#[doc = "ISI_CFG1 register accessor: an alias for `Reg<ISI_CFG1_SPEC>`"]
pub type ISI_CFG1 = crate::Reg<isi_cfg1::ISI_CFG1_SPEC>;
#[doc = "ISI Configuration 1 Register"]
pub mod isi_cfg1;
#[doc = "ISI_CFG2 register accessor: an alias for `Reg<ISI_CFG2_SPEC>`"]
pub type ISI_CFG2 = crate::Reg<isi_cfg2::ISI_CFG2_SPEC>;
#[doc = "ISI Configuration 2 Register"]
pub mod isi_cfg2;
#[doc = "ISI_PSIZE register accessor: an alias for `Reg<ISI_PSIZE_SPEC>`"]
pub type ISI_PSIZE = crate::Reg<isi_psize::ISI_PSIZE_SPEC>;
#[doc = "ISI Preview Size Register"]
pub mod isi_psize;
#[doc = "ISI_PDECF register accessor: an alias for `Reg<ISI_PDECF_SPEC>`"]
pub type ISI_PDECF = crate::Reg<isi_pdecf::ISI_PDECF_SPEC>;
#[doc = "ISI Preview Decimation Factor Register"]
pub mod isi_pdecf;
#[doc = "ISI_Y2R_SET0 register accessor: an alias for `Reg<ISI_Y2R_SET0_SPEC>`"]
pub type ISI_Y2R_SET0 = crate::Reg<isi_y2r_set0::ISI_Y2R_SET0_SPEC>;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
pub mod isi_y2r_set0;
#[doc = "ISI_Y2R_SET1 register accessor: an alias for `Reg<ISI_Y2R_SET1_SPEC>`"]
pub type ISI_Y2R_SET1 = crate::Reg<isi_y2r_set1::ISI_Y2R_SET1_SPEC>;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
pub mod isi_y2r_set1;
#[doc = "ISI_R2Y_SET0 register accessor: an alias for `Reg<ISI_R2Y_SET0_SPEC>`"]
pub type ISI_R2Y_SET0 = crate::Reg<isi_r2y_set0::ISI_R2Y_SET0_SPEC>;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
pub mod isi_r2y_set0;
#[doc = "ISI_R2Y_SET1 register accessor: an alias for `Reg<ISI_R2Y_SET1_SPEC>`"]
pub type ISI_R2Y_SET1 = crate::Reg<isi_r2y_set1::ISI_R2Y_SET1_SPEC>;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
pub mod isi_r2y_set1;
#[doc = "ISI_R2Y_SET2 register accessor: an alias for `Reg<ISI_R2Y_SET2_SPEC>`"]
pub type ISI_R2Y_SET2 = crate::Reg<isi_r2y_set2::ISI_R2Y_SET2_SPEC>;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
pub mod isi_r2y_set2;
#[doc = "ISI_CR register accessor: an alias for `Reg<ISI_CR_SPEC>`"]
pub type ISI_CR = crate::Reg<isi_cr::ISI_CR_SPEC>;
#[doc = "ISI Control Register"]
pub mod isi_cr;
#[doc = "ISI_SR register accessor: an alias for `Reg<ISI_SR_SPEC>`"]
pub type ISI_SR = crate::Reg<isi_sr::ISI_SR_SPEC>;
#[doc = "ISI Status Register"]
pub mod isi_sr;
#[doc = "ISI_IER register accessor: an alias for `Reg<ISI_IER_SPEC>`"]
pub type ISI_IER = crate::Reg<isi_ier::ISI_IER_SPEC>;
#[doc = "ISI Interrupt Enable Register"]
pub mod isi_ier;
#[doc = "ISI_IDR register accessor: an alias for `Reg<ISI_IDR_SPEC>`"]
pub type ISI_IDR = crate::Reg<isi_idr::ISI_IDR_SPEC>;
#[doc = "ISI Interrupt Disable Register"]
pub mod isi_idr;
#[doc = "ISI_IMR register accessor: an alias for `Reg<ISI_IMR_SPEC>`"]
pub type ISI_IMR = crate::Reg<isi_imr::ISI_IMR_SPEC>;
#[doc = "ISI Interrupt Mask Register"]
pub mod isi_imr;
#[doc = "ISI_DMA_CHER register accessor: an alias for `Reg<ISI_DMA_CHER_SPEC>`"]
pub type ISI_DMA_CHER = crate::Reg<isi_dma_cher::ISI_DMA_CHER_SPEC>;
#[doc = "DMA Channel Enable Register"]
pub mod isi_dma_cher;
#[doc = "ISI_DMA_CHDR register accessor: an alias for `Reg<ISI_DMA_CHDR_SPEC>`"]
pub type ISI_DMA_CHDR = crate::Reg<isi_dma_chdr::ISI_DMA_CHDR_SPEC>;
#[doc = "DMA Channel Disable Register"]
pub mod isi_dma_chdr;
#[doc = "ISI_DMA_CHSR register accessor: an alias for `Reg<ISI_DMA_CHSR_SPEC>`"]
pub type ISI_DMA_CHSR = crate::Reg<isi_dma_chsr::ISI_DMA_CHSR_SPEC>;
#[doc = "DMA Channel Status Register"]
pub mod isi_dma_chsr;
#[doc = "ISI_DMA_P_ADDR register accessor: an alias for `Reg<ISI_DMA_P_ADDR_SPEC>`"]
pub type ISI_DMA_P_ADDR = crate::Reg<isi_dma_p_addr::ISI_DMA_P_ADDR_SPEC>;
#[doc = "DMA Preview Base Address Register"]
pub mod isi_dma_p_addr;
#[doc = "ISI_DMA_P_CTRL register accessor: an alias for `Reg<ISI_DMA_P_CTRL_SPEC>`"]
pub type ISI_DMA_P_CTRL = crate::Reg<isi_dma_p_ctrl::ISI_DMA_P_CTRL_SPEC>;
#[doc = "DMA Preview Control Register"]
pub mod isi_dma_p_ctrl;
#[doc = "ISI_DMA_P_DSCR register accessor: an alias for `Reg<ISI_DMA_P_DSCR_SPEC>`"]
pub type ISI_DMA_P_DSCR = crate::Reg<isi_dma_p_dscr::ISI_DMA_P_DSCR_SPEC>;
#[doc = "DMA Preview Descriptor Address Register"]
pub mod isi_dma_p_dscr;
#[doc = "ISI_DMA_C_ADDR register accessor: an alias for `Reg<ISI_DMA_C_ADDR_SPEC>`"]
pub type ISI_DMA_C_ADDR = crate::Reg<isi_dma_c_addr::ISI_DMA_C_ADDR_SPEC>;
#[doc = "DMA Codec Base Address Register"]
pub mod isi_dma_c_addr;
#[doc = "ISI_DMA_C_CTRL register accessor: an alias for `Reg<ISI_DMA_C_CTRL_SPEC>`"]
pub type ISI_DMA_C_CTRL = crate::Reg<isi_dma_c_ctrl::ISI_DMA_C_CTRL_SPEC>;
#[doc = "DMA Codec Control Register"]
pub mod isi_dma_c_ctrl;
#[doc = "ISI_DMA_C_DSCR register accessor: an alias for `Reg<ISI_DMA_C_DSCR_SPEC>`"]
pub type ISI_DMA_C_DSCR = crate::Reg<isi_dma_c_dscr::ISI_DMA_C_DSCR_SPEC>;
#[doc = "DMA Codec Descriptor Address Register"]
pub mod isi_dma_c_dscr;
#[doc = "ISI_WPMR register accessor: an alias for `Reg<ISI_WPMR_SPEC>`"]
pub type ISI_WPMR = crate::Reg<isi_wpmr::ISI_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod isi_wpmr;
#[doc = "ISI_WPSR register accessor: an alias for `Reg<ISI_WPSR_SPEC>`"]
pub type ISI_WPSR = crate::Reg<isi_wpsr::ISI_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod isi_wpsr;
