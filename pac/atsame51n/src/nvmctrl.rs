#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - NVM Parameter"]
    pub param: PARAM,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0e - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x10 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x12 - Status"]
    pub status: STATUS,
    #[doc = "0x14 - Address"]
    pub addr: ADDR,
    #[doc = "0x18 - Lock Section"]
    pub runlock: RUNLOCK,
    #[doc = "0x1c..0x24 - Page Buffer Load Data x"]
    pub pbldata: [PBLDATA; 2],
    #[doc = "0x24 - ECC Error Status Register"]
    pub eccerr: ECCERR,
    #[doc = "0x28 - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved12: [u8; 0x01],
    #[doc = "0x2a - SmartEEPROM Configuration Register"]
    pub seecfg: SEECFG,
    _reserved13: [u8; 0x01],
    #[doc = "0x2c - SmartEEPROM Status Register"]
    pub seestat: SEESTAT,
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (w) register accessor: Control B\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "PARAM (r) register accessor: NVM Parameter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`param::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param`]
module"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "NVM Parameter"]
pub mod param;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ADDR (rw) register accessor: Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address"]
pub mod addr;
#[doc = "RUNLOCK (r) register accessor: Lock Section\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`runlock::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@runlock`]
module"]
pub type RUNLOCK = crate::Reg<runlock::RUNLOCK_SPEC>;
#[doc = "Lock Section"]
pub mod runlock;
#[doc = "PBLDATA (r) register accessor: Page Buffer Load Data x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbldata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbldata`]
module"]
pub type PBLDATA = crate::Reg<pbldata::PBLDATA_SPEC>;
#[doc = "Page Buffer Load Data x"]
pub mod pbldata;
#[doc = "ECCERR (r) register accessor: ECC Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccerr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccerr`]
module"]
pub type ECCERR = crate::Reg<eccerr::ECCERR_SPEC>;
#[doc = "ECC Error Status Register"]
pub mod eccerr;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SEECFG (rw) register accessor: SmartEEPROM Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seecfg`]
module"]
pub type SEECFG = crate::Reg<seecfg::SEECFG_SPEC>;
#[doc = "SmartEEPROM Configuration Register"]
pub mod seecfg;
#[doc = "SEESTAT (r) register accessor: SmartEEPROM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seestat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seestat`]
module"]
pub type SEESTAT = crate::Reg<seestat::SEESTAT_SPEC>;
#[doc = "SmartEEPROM Status Register"]
pub mod seestat;
