#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x01 - Sleep Mode"]
    pub sleep: crate::Reg<sleep::SLEEP_SPEC>,
    _reserved2: [u8; 0x06],
    #[doc = "0x08 - CPU Clock Select"]
    pub cpusel: crate::Reg<cpusel::CPUSEL_SPEC>,
    #[doc = "0x09 - APBA Clock Select"]
    pub apbasel: crate::Reg<apbasel::APBASEL_SPEC>,
    #[doc = "0x0a - APBB Clock Select"]
    pub apbbsel: crate::Reg<apbbsel::APBBSEL_SPEC>,
    #[doc = "0x0b - APBC Clock Select"]
    pub apbcsel: crate::Reg<apbcsel::APBCSEL_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x14 - AHB Mask"]
    pub ahbmask: crate::Reg<ahbmask::AHBMASK_SPEC>,
    #[doc = "0x18 - APBA Mask"]
    pub apbamask: crate::Reg<apbamask::APBAMASK_SPEC>,
    #[doc = "0x1c - APBB Mask"]
    pub apbbmask: crate::Reg<apbbmask::APBBMASK_SPEC>,
    #[doc = "0x20 - APBC Mask"]
    pub apbcmask: crate::Reg<apbcmask::APBCMASK_SPEC>,
    _reserved10: [u8; 0x10],
    #[doc = "0x34 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x35 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x36 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    _reserved13: [u8; 0x01],
    #[doc = "0x38 - Reset Cause"]
    pub rcause: crate::Reg<rcause::RCAUSE_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "SLEEP register accessor: an alias for `Reg<SLEEP_SPEC>`"]
pub type SLEEP = crate::Reg<sleep::SLEEP_SPEC>;
#[doc = "Sleep Mode"]
pub mod sleep;
#[doc = "CPUSEL register accessor: an alias for `Reg<CPUSEL_SPEC>`"]
pub type CPUSEL = crate::Reg<cpusel::CPUSEL_SPEC>;
#[doc = "CPU Clock Select"]
pub mod cpusel;
#[doc = "APBASEL register accessor: an alias for `Reg<APBASEL_SPEC>`"]
pub type APBASEL = crate::Reg<apbasel::APBASEL_SPEC>;
#[doc = "APBA Clock Select"]
pub mod apbasel;
#[doc = "APBBSEL register accessor: an alias for `Reg<APBBSEL_SPEC>`"]
pub type APBBSEL = crate::Reg<apbbsel::APBBSEL_SPEC>;
#[doc = "APBB Clock Select"]
pub mod apbbsel;
#[doc = "APBCSEL register accessor: an alias for `Reg<APBCSEL_SPEC>`"]
pub type APBCSEL = crate::Reg<apbcsel::APBCSEL_SPEC>;
#[doc = "APBC Clock Select"]
pub mod apbcsel;
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
#[doc = "RCAUSE register accessor: an alias for `Reg<RCAUSE_SPEC>`"]
pub type RCAUSE = crate::Reg<rcause::RCAUSE_SPEC>;
#[doc = "Reset Cause"]
pub mod rcause;
