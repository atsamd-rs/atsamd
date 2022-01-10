#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - NVM Parameter"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x0e - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x10 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x12 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x14 - Address"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x18 - Lock Section"]
    pub runlock: crate::Reg<runlock::RUNLOCK_SPEC>,
    #[doc = "0x1c..0x24 - Page Buffer Load Data x"]
    pub pbldata: [crate::Reg<pbldata::PBLDATA_SPEC>; 2],
    #[doc = "0x24 - ECC Error Status Register"]
    pub eccerr: crate::Reg<eccerr::ECCERR_SPEC>,
    #[doc = "0x28 - Debug Control"]
    pub dbgctrl: crate::Reg<dbgctrl::DBGCTRL_SPEC>,
    _reserved12: [u8; 0x01],
    #[doc = "0x2a - SmartEEPROM Configuration Register"]
    pub seecfg: crate::Reg<seecfg::SEECFG_SPEC>,
    _reserved13: [u8; 0x01],
    #[doc = "0x2c - SmartEEPROM Status Register"]
    pub seestat: crate::Reg<seestat::SEESTAT_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "NVM Parameter"]
pub mod param;
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
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address"]
pub mod addr;
#[doc = "RUNLOCK register accessor: an alias for `Reg<RUNLOCK_SPEC>`"]
pub type RUNLOCK = crate::Reg<runlock::RUNLOCK_SPEC>;
#[doc = "Lock Section"]
pub mod runlock;
#[doc = "PBLDATA register accessor: an alias for `Reg<PBLDATA_SPEC>`"]
pub type PBLDATA = crate::Reg<pbldata::PBLDATA_SPEC>;
#[doc = "Page Buffer Load Data x"]
pub mod pbldata;
#[doc = "ECCERR register accessor: an alias for `Reg<ECCERR_SPEC>`"]
pub type ECCERR = crate::Reg<eccerr::ECCERR_SPEC>;
#[doc = "ECC Error Status Register"]
pub mod eccerr;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SEECFG register accessor: an alias for `Reg<SEECFG_SPEC>`"]
pub type SEECFG = crate::Reg<seecfg::SEECFG_SPEC>;
#[doc = "SmartEEPROM Configuration Register"]
pub mod seecfg;
#[doc = "SEESTAT register accessor: an alias for `Reg<SEESTAT_SPEC>`"]
pub type SEESTAT = crate::Reg<seestat::SEESTAT_SPEC>;
#[doc = "SmartEEPROM Status Register"]
pub mod seestat;
