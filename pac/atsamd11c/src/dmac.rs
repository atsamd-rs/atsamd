#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    crcctrl: Crcctrl,
    crcdatain: Crcdatain,
    crcchksum: Crcchksum,
    crcstatus: Crcstatus,
    dbgctrl: Dbgctrl,
    qosctrl: Qosctrl,
    _reserved7: [u8; 0x01],
    swtrigctrl: Swtrigctrl,
    prictrl0: Prictrl0,
    _reserved9: [u8; 0x08],
    intpend: Intpend,
    _reserved10: [u8; 0x02],
    intstatus: Intstatus,
    busych: Busych,
    pendch: Pendch,
    active: Active,
    baseaddr: Baseaddr,
    wrbaddr: Wrbaddr,
    _reserved16: [u8; 0x03],
    chid: Chid,
    chctrla: Chctrla,
    _reserved18: [u8; 0x03],
    chctrlb: Chctrlb,
    _reserved19: [u8; 0x04],
    chintenclr: Chintenclr,
    chintenset: Chintenset,
    chintflag: Chintflag,
    chstatus: Chstatus,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x02 - CRC Control"]
    #[inline(always)]
    pub const fn crcctrl(&self) -> &Crcctrl {
        &self.crcctrl
    }
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn crcdatain(&self) -> &Crcdatain {
        &self.crcdatain
    }
    #[doc = "0x08 - CRC Checksum"]
    #[inline(always)]
    pub const fn crcchksum(&self) -> &Crcchksum {
        &self.crcchksum
    }
    #[doc = "0x0c - CRC Status"]
    #[inline(always)]
    pub const fn crcstatus(&self) -> &Crcstatus {
        &self.crcstatus
    }
    #[doc = "0x0d - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x0e - QOS Control"]
    #[inline(always)]
    pub const fn qosctrl(&self) -> &Qosctrl {
        &self.qosctrl
    }
    #[doc = "0x10 - Software Trigger Control"]
    #[inline(always)]
    pub const fn swtrigctrl(&self) -> &Swtrigctrl {
        &self.swtrigctrl
    }
    #[doc = "0x14 - Priority Control 0"]
    #[inline(always)]
    pub const fn prictrl0(&self) -> &Prictrl0 {
        &self.prictrl0
    }
    #[doc = "0x20 - Interrupt Pending"]
    #[inline(always)]
    pub const fn intpend(&self) -> &Intpend {
        &self.intpend
    }
    #[doc = "0x24 - Interrupt Status"]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        &self.intstatus
    }
    #[doc = "0x28 - Busy Channels"]
    #[inline(always)]
    pub const fn busych(&self) -> &Busych {
        &self.busych
    }
    #[doc = "0x2c - Pending Channels"]
    #[inline(always)]
    pub const fn pendch(&self) -> &Pendch {
        &self.pendch
    }
    #[doc = "0x30 - Active Channel and Levels"]
    #[inline(always)]
    pub const fn active(&self) -> &Active {
        &self.active
    }
    #[doc = "0x34 - Descriptor Memory Section Base Address"]
    #[inline(always)]
    pub const fn baseaddr(&self) -> &Baseaddr {
        &self.baseaddr
    }
    #[doc = "0x38 - Write-Back Memory Section Base Address"]
    #[inline(always)]
    pub const fn wrbaddr(&self) -> &Wrbaddr {
        &self.wrbaddr
    }
    #[doc = "0x3f - Channel ID"]
    #[inline(always)]
    pub const fn chid(&self) -> &Chid {
        &self.chid
    }
    #[doc = "0x40 - Channel Control A"]
    #[inline(always)]
    pub const fn chctrla(&self) -> &Chctrla {
        &self.chctrla
    }
    #[doc = "0x44 - Channel Control B"]
    #[inline(always)]
    pub const fn chctrlb(&self) -> &Chctrlb {
        &self.chctrlb
    }
    #[doc = "0x4c - Channel Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn chintenclr(&self) -> &Chintenclr {
        &self.chintenclr
    }
    #[doc = "0x4d - Channel Interrupt Enable Set"]
    #[inline(always)]
    pub const fn chintenset(&self) -> &Chintenset {
        &self.chintenset
    }
    #[doc = "0x4e - Channel Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn chintflag(&self) -> &Chintflag {
        &self.chintflag
    }
    #[doc = "0x4f - Channel Status"]
    #[inline(always)]
    pub const fn chstatus(&self) -> &Chstatus {
        &self.chstatus
    }
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "CRCCTRL (rw) register accessor: CRC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcctrl`]
module"]
#[doc(alias = "CRCCTRL")]
pub type Crcctrl = crate::Reg<crcctrl::CrcctrlSpec>;
#[doc = "CRC Control"]
pub mod crcctrl;
#[doc = "CRCDATAIN (rw) register accessor: CRC Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdatain::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdatain::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdatain`]
module"]
#[doc(alias = "CRCDATAIN")]
pub type Crcdatain = crate::Reg<crcdatain::CrcdatainSpec>;
#[doc = "CRC Data Input"]
pub mod crcdatain;
#[doc = "CRCCHKSUM (rw) register accessor: CRC Checksum\n\nYou can [`read`](crate::Reg::read) this register and get [`crcchksum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcchksum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcchksum`]
module"]
#[doc(alias = "CRCCHKSUM")]
pub type Crcchksum = crate::Reg<crcchksum::CrcchksumSpec>;
#[doc = "CRC Checksum"]
pub mod crcchksum;
#[doc = "CRCSTATUS (rw) register accessor: CRC Status\n\nYou can [`read`](crate::Reg::read) this register and get [`crcstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcstatus`]
module"]
#[doc(alias = "CRCSTATUS")]
pub type Crcstatus = crate::Reg<crcstatus::CrcstatusSpec>;
#[doc = "CRC Status"]
pub mod crcstatus;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "QOSCTRL (rw) register accessor: QOS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qosctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qosctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qosctrl`]
module"]
#[doc(alias = "QOSCTRL")]
pub type Qosctrl = crate::Reg<qosctrl::QosctrlSpec>;
#[doc = "QOS Control"]
pub mod qosctrl;
#[doc = "SWTRIGCTRL (rw) register accessor: Software Trigger Control\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrigctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrigctrl`]
module"]
#[doc(alias = "SWTRIGCTRL")]
pub type Swtrigctrl = crate::Reg<swtrigctrl::SwtrigctrlSpec>;
#[doc = "Software Trigger Control"]
pub mod swtrigctrl;
#[doc = "PRICTRL0 (rw) register accessor: Priority Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`prictrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prictrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prictrl0`]
module"]
#[doc(alias = "PRICTRL0")]
pub type Prictrl0 = crate::Reg<prictrl0::Prictrl0Spec>;
#[doc = "Priority Control 0"]
pub mod prictrl0;
#[doc = "INTPEND (rw) register accessor: Interrupt Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend`]
module"]
#[doc(alias = "INTPEND")]
pub type Intpend = crate::Reg<intpend::IntpendSpec>;
#[doc = "Interrupt Pending"]
pub mod intpend;
#[doc = "INTSTATUS (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
#[doc(alias = "INTSTATUS")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "BUSYCH (r) register accessor: Busy Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`busych::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busych`]
module"]
#[doc(alias = "BUSYCH")]
pub type Busych = crate::Reg<busych::BusychSpec>;
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "PENDCH (r) register accessor: Pending Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`pendch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pendch`]
module"]
#[doc(alias = "PENDCH")]
pub type Pendch = crate::Reg<pendch::PendchSpec>;
#[doc = "Pending Channels"]
pub mod pendch;
#[doc = "ACTIVE (r) register accessor: Active Channel and Levels\n\nYou can [`read`](crate::Reg::read) this register and get [`active::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active`]
module"]
#[doc(alias = "ACTIVE")]
pub type Active = crate::Reg<active::ActiveSpec>;
#[doc = "Active Channel and Levels"]
pub mod active;
#[doc = "BASEADDR (rw) register accessor: Descriptor Memory Section Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`baseaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baseaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baseaddr`]
module"]
#[doc(alias = "BASEADDR")]
pub type Baseaddr = crate::Reg<baseaddr::BaseaddrSpec>;
#[doc = "Descriptor Memory Section Base Address"]
pub mod baseaddr;
#[doc = "WRBADDR (rw) register accessor: Write-Back Memory Section Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`wrbaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrbaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrbaddr`]
module"]
#[doc(alias = "WRBADDR")]
pub type Wrbaddr = crate::Reg<wrbaddr::WrbaddrSpec>;
#[doc = "Write-Back Memory Section Base Address"]
pub mod wrbaddr;
#[doc = "CHID (rw) register accessor: Channel ID\n\nYou can [`read`](crate::Reg::read) this register and get [`chid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chid`]
module"]
#[doc(alias = "CHID")]
pub type Chid = crate::Reg<chid::ChidSpec>;
#[doc = "Channel ID"]
pub mod chid;
#[doc = "CHCTRLA (rw) register accessor: Channel Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrla`]
module"]
#[doc(alias = "CHCTRLA")]
pub type Chctrla = crate::Reg<chctrla::ChctrlaSpec>;
#[doc = "Channel Control A"]
pub mod chctrla;
#[doc = "CHCTRLB (rw) register accessor: Channel Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrlb`]
module"]
#[doc(alias = "CHCTRLB")]
pub type Chctrlb = crate::Reg<chctrlb::ChctrlbSpec>;
#[doc = "Channel Control B"]
pub mod chctrlb;
#[doc = "CHINTENCLR (rw) register accessor: Channel Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenclr`]
module"]
#[doc(alias = "CHINTENCLR")]
pub type Chintenclr = crate::Reg<chintenclr::ChintenclrSpec>;
#[doc = "Channel Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "CHINTENSET (rw) register accessor: Channel Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenset`]
module"]
#[doc(alias = "CHINTENSET")]
pub type Chintenset = crate::Reg<chintenset::ChintensetSpec>;
#[doc = "Channel Interrupt Enable Set"]
pub mod chintenset;
#[doc = "CHINTFLAG (rw) register accessor: Channel Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintflag`]
module"]
#[doc(alias = "CHINTFLAG")]
pub type Chintflag = crate::Reg<chintflag::ChintflagSpec>;
#[doc = "Channel Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "CHSTATUS (r) register accessor: Channel Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatus`]
module"]
#[doc(alias = "CHSTATUS")]
pub type Chstatus = crate::Reg<chstatus::ChstatusSpec>;
#[doc = "Channel Status"]
pub mod chstatus;
