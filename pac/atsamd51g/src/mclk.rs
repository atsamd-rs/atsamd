#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x02 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x03 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x04 - HS Clock Division"]
    pub hsdiv: HSDIV,
    #[doc = "0x05 - CPU Clock Division"]
    pub cpudiv: CPUDIV,
    _reserved5: [u8; 0x0a],
    #[doc = "0x10 - AHB Mask"]
    pub ahbmask: AHBMASK,
    #[doc = "0x14 - APBA Mask"]
    pub apbamask: APBAMASK,
    #[doc = "0x18 - APBB Mask"]
    pub apbbmask: APBBMASK,
    #[doc = "0x1c - APBC Mask"]
    pub apbcmask: APBCMASK,
    #[doc = "0x20 - APBD Mask"]
    pub apbdmask: APBDMASK,
}
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "HSDIV (r) register accessor: an alias for `Reg<HSDIV_SPEC>`"]
pub type HSDIV = crate::Reg<hsdiv::HSDIV_SPEC>;
#[doc = "HS Clock Division"]
pub mod hsdiv;
#[doc = "CPUDIV (rw) register accessor: an alias for `Reg<CPUDIV_SPEC>`"]
pub type CPUDIV = crate::Reg<cpudiv::CPUDIV_SPEC>;
#[doc = "CPU Clock Division"]
pub mod cpudiv;
#[doc = "AHBMASK (rw) register accessor: an alias for `Reg<AHBMASK_SPEC>`"]
pub type AHBMASK = crate::Reg<ahbmask::AHBMASK_SPEC>;
#[doc = "AHB Mask"]
pub mod ahbmask;
#[doc = "APBAMASK (rw) register accessor: an alias for `Reg<APBAMASK_SPEC>`"]
pub type APBAMASK = crate::Reg<apbamask::APBAMASK_SPEC>;
#[doc = "APBA Mask"]
pub mod apbamask;
#[doc = "APBBMASK (rw) register accessor: an alias for `Reg<APBBMASK_SPEC>`"]
pub type APBBMASK = crate::Reg<apbbmask::APBBMASK_SPEC>;
#[doc = "APBB Mask"]
pub mod apbbmask;
#[doc = "APBCMASK (rw) register accessor: an alias for `Reg<APBCMASK_SPEC>`"]
pub type APBCMASK = crate::Reg<apbcmask::APBCMASK_SPEC>;
#[doc = "APBC Mask"]
pub mod apbcmask;
#[doc = "APBDMASK (rw) register accessor: an alias for `Reg<APBDMASK_SPEC>`"]
pub type APBDMASK = crate::Reg<apbdmask::APBDMASK_SPEC>;
#[doc = "APBD Mask"]
pub mod apbdmask;
