#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x02 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x03 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x04 - HS Clock Division"]
    pub hsdiv: crate::Reg<hsdiv::HSDIV_SPEC>,
    #[doc = "0x05 - CPU Clock Division"]
    pub cpudiv: crate::Reg<cpudiv::CPUDIV_SPEC>,
    _reserved5: [u8; 0x0a],
    #[doc = "0x10 - AHB Mask"]
    pub ahbmask: crate::Reg<ahbmask::AHBMASK_SPEC>,
    #[doc = "0x14 - APBA Mask"]
    pub apbamask: crate::Reg<apbamask::APBAMASK_SPEC>,
    #[doc = "0x18 - APBB Mask"]
    pub apbbmask: crate::Reg<apbbmask::APBBMASK_SPEC>,
    #[doc = "0x1c - APBC Mask"]
    pub apbcmask: crate::Reg<apbcmask::APBCMASK_SPEC>,
    #[doc = "0x20 - APBD Mask"]
    pub apbdmask: crate::Reg<apbdmask::APBDMASK_SPEC>,
}
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "HSDIV register accessor: an alias for `Reg<HSDIV_SPEC>`"]
pub type HSDIV = crate::Reg<hsdiv::HSDIV_SPEC>;
#[doc = "HS Clock Division"]
pub mod hsdiv;
#[doc = "CPUDIV register accessor: an alias for `Reg<CPUDIV_SPEC>`"]
pub type CPUDIV = crate::Reg<cpudiv::CPUDIV_SPEC>;
#[doc = "CPU Clock Division"]
pub mod cpudiv;
#[doc = "AHBMASK register accessor: an alias for `Reg<AHBMASK_SPEC>`"]
pub type AHBMASK = crate::Reg<ahbmask::AHBMASK_SPEC>;
#[doc = "AHB Mask"]
pub mod ahbmask;
#[doc = "APBAMASK register accessor: an alias for `Reg<APBAMASK_SPEC>`"]
pub type APBAMASK = crate::Reg<apbamask::APBAMASK_SPEC>;
#[doc = "APBA Mask"]
pub mod apbamask;
#[doc = "APBBMASK register accessor: an alias for `Reg<APBBMASK_SPEC>`"]
pub type APBBMASK = crate::Reg<apbbmask::APBBMASK_SPEC>;
#[doc = "APBB Mask"]
pub mod apbbmask;
#[doc = "APBCMASK register accessor: an alias for `Reg<APBCMASK_SPEC>`"]
pub type APBCMASK = crate::Reg<apbcmask::APBCMASK_SPEC>;
#[doc = "APBC Mask"]
pub mod apbcmask;
#[doc = "APBDMASK register accessor: an alias for `Reg<APBDMASK_SPEC>`"]
pub type APBDMASK = crate::Reg<apbdmask::APBDMASK_SPEC>;
#[doc = "APBD Mask"]
pub mod apbdmask;
