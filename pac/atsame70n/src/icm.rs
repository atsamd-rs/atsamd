#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub icm_cfg: crate::Reg<icm_cfg::ICM_CFG_SPEC>,
    #[doc = "0x04 - Control Register"]
    pub icm_ctrl: crate::Reg<icm_ctrl::ICM_CTRL_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub icm_sr: crate::Reg<icm_sr::ICM_SR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub icm_ier: crate::Reg<icm_ier::ICM_IER_SPEC>,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub icm_idr: crate::Reg<icm_idr::ICM_IDR_SPEC>,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub icm_imr: crate::Reg<icm_imr::ICM_IMR_SPEC>,
    #[doc = "0x1c - Interrupt Status Register"]
    pub icm_isr: crate::Reg<icm_isr::ICM_ISR_SPEC>,
    #[doc = "0x20 - Undefined Access Status Register"]
    pub icm_uasr: crate::Reg<icm_uasr::ICM_UASR_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - Region Descriptor Area Start Address Register"]
    pub icm_dscr: crate::Reg<icm_dscr::ICM_DSCR_SPEC>,
    #[doc = "0x34 - Region Hash Area Start Address Register"]
    pub icm_hash: crate::Reg<icm_hash::ICM_HASH_SPEC>,
    #[doc = "0x38..0x58 - User Initial Hash Value 0 Register 0"]
    pub icm_uihval: [crate::Reg<icm_uihval::ICM_UIHVAL_SPEC>; 8],
}
#[doc = "ICM_CFG register accessor: an alias for `Reg<ICM_CFG_SPEC>`"]
pub type ICM_CFG = crate::Reg<icm_cfg::ICM_CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod icm_cfg;
#[doc = "ICM_CTRL register accessor: an alias for `Reg<ICM_CTRL_SPEC>`"]
pub type ICM_CTRL = crate::Reg<icm_ctrl::ICM_CTRL_SPEC>;
#[doc = "Control Register"]
pub mod icm_ctrl;
#[doc = "ICM_SR register accessor: an alias for `Reg<ICM_SR_SPEC>`"]
pub type ICM_SR = crate::Reg<icm_sr::ICM_SR_SPEC>;
#[doc = "Status Register"]
pub mod icm_sr;
#[doc = "ICM_IER register accessor: an alias for `Reg<ICM_IER_SPEC>`"]
pub type ICM_IER = crate::Reg<icm_ier::ICM_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod icm_ier;
#[doc = "ICM_IDR register accessor: an alias for `Reg<ICM_IDR_SPEC>`"]
pub type ICM_IDR = crate::Reg<icm_idr::ICM_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod icm_idr;
#[doc = "ICM_IMR register accessor: an alias for `Reg<ICM_IMR_SPEC>`"]
pub type ICM_IMR = crate::Reg<icm_imr::ICM_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod icm_imr;
#[doc = "ICM_ISR register accessor: an alias for `Reg<ICM_ISR_SPEC>`"]
pub type ICM_ISR = crate::Reg<icm_isr::ICM_ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod icm_isr;
#[doc = "ICM_UASR register accessor: an alias for `Reg<ICM_UASR_SPEC>`"]
pub type ICM_UASR = crate::Reg<icm_uasr::ICM_UASR_SPEC>;
#[doc = "Undefined Access Status Register"]
pub mod icm_uasr;
#[doc = "ICM_DSCR register accessor: an alias for `Reg<ICM_DSCR_SPEC>`"]
pub type ICM_DSCR = crate::Reg<icm_dscr::ICM_DSCR_SPEC>;
#[doc = "Region Descriptor Area Start Address Register"]
pub mod icm_dscr;
#[doc = "ICM_HASH register accessor: an alias for `Reg<ICM_HASH_SPEC>`"]
pub type ICM_HASH = crate::Reg<icm_hash::ICM_HASH_SPEC>;
#[doc = "Region Hash Area Start Address Register"]
pub mod icm_hash;
#[doc = "ICM_UIHVAL register accessor: an alias for `Reg<ICM_UIHVAL_SPEC>`"]
pub type ICM_UIHVAL = crate::Reg<icm_uihval::ICM_UIHVAL_SPEC>;
#[doc = "User Initial Hash Value 0 Register 0"]
pub mod icm_uihval;
