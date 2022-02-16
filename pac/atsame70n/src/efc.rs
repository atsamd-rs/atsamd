#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    pub eefc_fmr: crate::Reg<eefc_fmr::EEFC_FMR_SPEC>,
    #[doc = "0x04 - EEFC Flash Command Register"]
    pub eefc_fcr: crate::Reg<eefc_fcr::EEFC_FCR_SPEC>,
    #[doc = "0x08 - EEFC Flash Status Register"]
    pub eefc_fsr: crate::Reg<eefc_fsr::EEFC_FSR_SPEC>,
    #[doc = "0x0c - EEFC Flash Result Register"]
    pub eefc_frr: crate::Reg<eefc_frr::EEFC_FRR_SPEC>,
    _reserved4: [u8; 0xd4],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub eefc_wpmr: crate::Reg<eefc_wpmr::EEFC_WPMR_SPEC>,
}
#[doc = "EEFC_FMR register accessor: an alias for `Reg<EEFC_FMR_SPEC>`"]
pub type EEFC_FMR = crate::Reg<eefc_fmr::EEFC_FMR_SPEC>;
#[doc = "EEFC Flash Mode Register"]
pub mod eefc_fmr;
#[doc = "EEFC_FCR register accessor: an alias for `Reg<EEFC_FCR_SPEC>`"]
pub type EEFC_FCR = crate::Reg<eefc_fcr::EEFC_FCR_SPEC>;
#[doc = "EEFC Flash Command Register"]
pub mod eefc_fcr;
#[doc = "EEFC_FSR register accessor: an alias for `Reg<EEFC_FSR_SPEC>`"]
pub type EEFC_FSR = crate::Reg<eefc_fsr::EEFC_FSR_SPEC>;
#[doc = "EEFC Flash Status Register"]
pub mod eefc_fsr;
#[doc = "EEFC_FRR register accessor: an alias for `Reg<EEFC_FRR_SPEC>`"]
pub type EEFC_FRR = crate::Reg<eefc_frr::EEFC_FRR_SPEC>;
#[doc = "EEFC Flash Result Register"]
pub mod eefc_frr;
#[doc = "EEFC_WPMR register accessor: an alias for `Reg<EEFC_WPMR_SPEC>`"]
pub type EEFC_WPMR = crate::Reg<eefc_wpmr::EEFC_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod eefc_wpmr;
