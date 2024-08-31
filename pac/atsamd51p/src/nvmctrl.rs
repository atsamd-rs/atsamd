#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x02],
    ctrlb: Ctrlb,
    _reserved2: [u8; 0x02],
    param: Param,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    addr: Addr,
    runlock: Runlock,
    pbldata: [Pbldata; 2],
    eccerr: Eccerr,
    dbgctrl: Dbgctrl,
    _reserved12: [u8; 0x01],
    seecfg: Seecfg,
    _reserved13: [u8; 0x01],
    seestat: Seestat,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - NVM Parameter"]
    #[inline(always)]
    pub const fn param(&self) -> &Param {
        &self.param
    }
    #[doc = "0x0c - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x0e - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x10 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x12 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - Address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x18 - Lock Section"]
    #[inline(always)]
    pub const fn runlock(&self) -> &Runlock {
        &self.runlock
    }
    #[doc = "0x1c..0x24 - Page Buffer Load Data x"]
    #[inline(always)]
    pub const fn pbldata(&self, n: usize) -> &Pbldata {
        &self.pbldata[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x24 - Page Buffer Load Data x"]
    #[inline(always)]
    pub fn pbldata_iter(&self) -> impl Iterator<Item = &Pbldata> {
        self.pbldata.iter()
    }
    #[doc = "0x24 - ECC Error Status Register"]
    #[inline(always)]
    pub const fn eccerr(&self) -> &Eccerr {
        &self.eccerr
    }
    #[doc = "0x28 - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x2a - SmartEEPROM Configuration Register"]
    #[inline(always)]
    pub const fn seecfg(&self) -> &Seecfg {
        &self.seecfg
    }
    #[doc = "0x2c - SmartEEPROM Status Register"]
    #[inline(always)]
    pub const fn seestat(&self) -> &Seestat {
        &self.seestat
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (w) register accessor: Control B\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "PARAM (r) register accessor: NVM Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param`]
module"]
#[doc(alias = "PARAM")]
pub type Param = crate::Reg<param::ParamSpec>;
#[doc = "NVM Parameter"]
pub mod param;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "ADDR (rw) register accessor: Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Address"]
pub mod addr;
#[doc = "RUNLOCK (r) register accessor: Lock Section\n\nYou can [`read`](crate::Reg::read) this register and get [`runlock::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@runlock`]
module"]
#[doc(alias = "RUNLOCK")]
pub type Runlock = crate::Reg<runlock::RunlockSpec>;
#[doc = "Lock Section"]
pub mod runlock;
#[doc = "PBLDATA (r) register accessor: Page Buffer Load Data x\n\nYou can [`read`](crate::Reg::read) this register and get [`pbldata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbldata`]
module"]
#[doc(alias = "PBLDATA")]
pub type Pbldata = crate::Reg<pbldata::PbldataSpec>;
#[doc = "Page Buffer Load Data x"]
pub mod pbldata;
#[doc = "ECCERR (r) register accessor: ECC Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccerr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccerr`]
module"]
#[doc(alias = "ECCERR")]
pub type Eccerr = crate::Reg<eccerr::EccerrSpec>;
#[doc = "ECC Error Status Register"]
pub mod eccerr;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SEECFG (rw) register accessor: SmartEEPROM Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`seecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seecfg`]
module"]
#[doc(alias = "SEECFG")]
pub type Seecfg = crate::Reg<seecfg::SeecfgSpec>;
#[doc = "SmartEEPROM Configuration Register"]
pub mod seecfg;
#[doc = "SEESTAT (r) register accessor: SmartEEPROM Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`seestat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seestat`]
module"]
#[doc(alias = "SEESTAT")]
pub type Seestat = crate::Reg<seestat::SeestatSpec>;
#[doc = "SmartEEPROM Status Register"]
pub mod seestat;
