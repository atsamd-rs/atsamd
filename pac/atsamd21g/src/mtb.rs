#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    position: Position,
    master: Master,
    flow: Flow,
    base: Base,
    _reserved4: [u8; 0x0ef0],
    itctrl: Itctrl,
    _reserved5: [u8; 0x9c],
    claimset: Claimset,
    claimclr: Claimclr,
    _reserved7: [u8; 0x08],
    lockaccess: Lockaccess,
    lockstatus: Lockstatus,
    authstatus: Authstatus,
    devarch: Devarch,
    _reserved11: [u8; 0x08],
    devid: Devid,
    devtype: Devtype,
    pid4: Pid4,
    pid5: Pid5,
    pid6: Pid6,
    pid7: Pid7,
    pid0: Pid0,
    pid1: Pid1,
    pid2: Pid2,
    pid3: Pid3,
    cid0: Cid0,
    cid1: Cid1,
    cid2: Cid2,
    cid3: Cid3,
}
impl RegisterBlock {
    #[doc = "0x00 - MTB Position"]
    #[inline(always)]
    pub const fn position(&self) -> &Position {
        &self.position
    }
    #[doc = "0x04 - MTB Master"]
    #[inline(always)]
    pub const fn master(&self) -> &Master {
        &self.master
    }
    #[doc = "0x08 - MTB Flow"]
    #[inline(always)]
    pub const fn flow(&self) -> &Flow {
        &self.flow
    }
    #[doc = "0x0c - MTB Base"]
    #[inline(always)]
    pub const fn base(&self) -> &Base {
        &self.base
    }
    #[doc = "0xf00 - MTB Integration Mode Control"]
    #[inline(always)]
    pub const fn itctrl(&self) -> &Itctrl {
        &self.itctrl
    }
    #[doc = "0xfa0 - MTB Claim Set"]
    #[inline(always)]
    pub const fn claimset(&self) -> &Claimset {
        &self.claimset
    }
    #[doc = "0xfa4 - MTB Claim Clear"]
    #[inline(always)]
    pub const fn claimclr(&self) -> &Claimclr {
        &self.claimclr
    }
    #[doc = "0xfb0 - MTB Lock Access"]
    #[inline(always)]
    pub const fn lockaccess(&self) -> &Lockaccess {
        &self.lockaccess
    }
    #[doc = "0xfb4 - MTB Lock Status"]
    #[inline(always)]
    pub const fn lockstatus(&self) -> &Lockstatus {
        &self.lockstatus
    }
    #[doc = "0xfb8 - MTB Authentication Status"]
    #[inline(always)]
    pub const fn authstatus(&self) -> &Authstatus {
        &self.authstatus
    }
    #[doc = "0xfbc - MTB Device Architecture"]
    #[inline(always)]
    pub const fn devarch(&self) -> &Devarch {
        &self.devarch
    }
    #[doc = "0xfc8 - MTB Device Configuration"]
    #[inline(always)]
    pub const fn devid(&self) -> &Devid {
        &self.devid
    }
    #[doc = "0xfcc - MTB Device Type"]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
    #[doc = "0xfd0 - CoreSight"]
    #[inline(always)]
    pub const fn pid4(&self) -> &Pid4 {
        &self.pid4
    }
    #[doc = "0xfd4 - CoreSight"]
    #[inline(always)]
    pub const fn pid5(&self) -> &Pid5 {
        &self.pid5
    }
    #[doc = "0xfd8 - CoreSight"]
    #[inline(always)]
    pub const fn pid6(&self) -> &Pid6 {
        &self.pid6
    }
    #[doc = "0xfdc - CoreSight"]
    #[inline(always)]
    pub const fn pid7(&self) -> &Pid7 {
        &self.pid7
    }
    #[doc = "0xfe0 - CoreSight"]
    #[inline(always)]
    pub const fn pid0(&self) -> &Pid0 {
        &self.pid0
    }
    #[doc = "0xfe4 - CoreSight"]
    #[inline(always)]
    pub const fn pid1(&self) -> &Pid1 {
        &self.pid1
    }
    #[doc = "0xfe8 - CoreSight"]
    #[inline(always)]
    pub const fn pid2(&self) -> &Pid2 {
        &self.pid2
    }
    #[doc = "0xfec - CoreSight"]
    #[inline(always)]
    pub const fn pid3(&self) -> &Pid3 {
        &self.pid3
    }
    #[doc = "0xff0 - CoreSight"]
    #[inline(always)]
    pub const fn cid0(&self) -> &Cid0 {
        &self.cid0
    }
    #[doc = "0xff4 - CoreSight"]
    #[inline(always)]
    pub const fn cid1(&self) -> &Cid1 {
        &self.cid1
    }
    #[doc = "0xff8 - CoreSight"]
    #[inline(always)]
    pub const fn cid2(&self) -> &Cid2 {
        &self.cid2
    }
    #[doc = "0xffc - CoreSight"]
    #[inline(always)]
    pub const fn cid3(&self) -> &Cid3 {
        &self.cid3
    }
}
#[doc = "POSITION (rw) register accessor: MTB Position\n\nYou can [`read`](crate::Reg::read) this register and get [`position::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`position::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@position`] module"]
#[doc(alias = "POSITION")]
pub type Position = crate::Reg<position::PositionSpec>;
#[doc = "MTB Position"]
pub mod position;
#[doc = "MASTER (rw) register accessor: MTB Master\n\nYou can [`read`](crate::Reg::read) this register and get [`master::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master`] module"]
#[doc(alias = "MASTER")]
pub type Master = crate::Reg<master::MasterSpec>;
#[doc = "MTB Master"]
pub mod master;
#[doc = "FLOW (rw) register accessor: MTB Flow\n\nYou can [`read`](crate::Reg::read) this register and get [`flow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow`] module"]
#[doc(alias = "FLOW")]
pub type Flow = crate::Reg<flow::FlowSpec>;
#[doc = "MTB Flow"]
pub mod flow;
#[doc = "BASE (r) register accessor: MTB Base\n\nYou can [`read`](crate::Reg::read) this register and get [`base::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base`] module"]
#[doc(alias = "BASE")]
pub type Base = crate::Reg<base::BaseSpec>;
#[doc = "MTB Base"]
pub mod base;
#[doc = "ITCTRL (rw) register accessor: MTB Integration Mode Control\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itctrl`] module"]
#[doc(alias = "ITCTRL")]
pub type Itctrl = crate::Reg<itctrl::ItctrlSpec>;
#[doc = "MTB Integration Mode Control"]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: MTB Claim Set\n\nYou can [`read`](crate::Reg::read) this register and get [`claimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimset`] module"]
#[doc(alias = "CLAIMSET")]
pub type Claimset = crate::Reg<claimset::ClaimsetSpec>;
#[doc = "MTB Claim Set"]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: MTB Claim Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`claimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimclr`] module"]
#[doc(alias = "CLAIMCLR")]
pub type Claimclr = crate::Reg<claimclr::ClaimclrSpec>;
#[doc = "MTB Claim Clear"]
pub mod claimclr;
#[doc = "LOCKACCESS (rw) register accessor: MTB Lock Access\n\nYou can [`read`](crate::Reg::read) this register and get [`lockaccess::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockaccess::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockaccess`] module"]
#[doc(alias = "LOCKACCESS")]
pub type Lockaccess = crate::Reg<lockaccess::LockaccessSpec>;
#[doc = "MTB Lock Access"]
pub mod lockaccess;
#[doc = "LOCKSTATUS (r) register accessor: MTB Lock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lockstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockstatus`] module"]
#[doc(alias = "LOCKSTATUS")]
pub type Lockstatus = crate::Reg<lockstatus::LockstatusSpec>;
#[doc = "MTB Lock Status"]
pub mod lockstatus;
#[doc = "AUTHSTATUS (r) register accessor: MTB Authentication Status\n\nYou can [`read`](crate::Reg::read) this register and get [`authstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authstatus`] module"]
#[doc(alias = "AUTHSTATUS")]
pub type Authstatus = crate::Reg<authstatus::AuthstatusSpec>;
#[doc = "MTB Authentication Status"]
pub mod authstatus;
#[doc = "DEVARCH (r) register accessor: MTB Device Architecture\n\nYou can [`read`](crate::Reg::read) this register and get [`devarch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devarch`] module"]
#[doc(alias = "DEVARCH")]
pub type Devarch = crate::Reg<devarch::DevarchSpec>;
#[doc = "MTB Device Architecture"]
pub mod devarch;
#[doc = "DEVID (r) register accessor: MTB Device Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devid`] module"]
#[doc(alias = "DEVID")]
pub type Devid = crate::Reg<devid::DevidSpec>;
#[doc = "MTB Device Configuration"]
pub mod devid;
#[doc = "DEVTYPE (r) register accessor: MTB Device Type\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`] module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "MTB Device Type"]
pub mod devtype;
#[doc = "PID4 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid4`] module"]
#[doc(alias = "PID4")]
pub type Pid4 = crate::Reg<pid4::Pid4Spec>;
#[doc = "CoreSight"]
pub mod pid4;
#[doc = "PID5 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid5`] module"]
#[doc(alias = "PID5")]
pub type Pid5 = crate::Reg<pid5::Pid5Spec>;
#[doc = "CoreSight"]
pub mod pid5;
#[doc = "PID6 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid6`] module"]
#[doc(alias = "PID6")]
pub type Pid6 = crate::Reg<pid6::Pid6Spec>;
#[doc = "CoreSight"]
pub mod pid6;
#[doc = "PID7 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid7`] module"]
#[doc(alias = "PID7")]
pub type Pid7 = crate::Reg<pid7::Pid7Spec>;
#[doc = "CoreSight"]
pub mod pid7;
#[doc = "PID0 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid0`] module"]
#[doc(alias = "PID0")]
pub type Pid0 = crate::Reg<pid0::Pid0Spec>;
#[doc = "CoreSight"]
pub mod pid0;
#[doc = "PID1 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid1`] module"]
#[doc(alias = "PID1")]
pub type Pid1 = crate::Reg<pid1::Pid1Spec>;
#[doc = "CoreSight"]
pub mod pid1;
#[doc = "PID2 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid2`] module"]
#[doc(alias = "PID2")]
pub type Pid2 = crate::Reg<pid2::Pid2Spec>;
#[doc = "CoreSight"]
pub mod pid2;
#[doc = "PID3 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid3`] module"]
#[doc(alias = "PID3")]
pub type Pid3 = crate::Reg<pid3::Pid3Spec>;
#[doc = "CoreSight"]
pub mod pid3;
#[doc = "CID0 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`cid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid0`] module"]
#[doc(alias = "CID0")]
pub type Cid0 = crate::Reg<cid0::Cid0Spec>;
#[doc = "CoreSight"]
pub mod cid0;
#[doc = "CID1 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`cid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid1`] module"]
#[doc(alias = "CID1")]
pub type Cid1 = crate::Reg<cid1::Cid1Spec>;
#[doc = "CoreSight"]
pub mod cid1;
#[doc = "CID2 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`cid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid2`] module"]
#[doc(alias = "CID2")]
pub type Cid2 = crate::Reg<cid2::Cid2Spec>;
#[doc = "CoreSight"]
pub mod cid2;
#[doc = "CID3 (r) register accessor: CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`cid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid3`] module"]
#[doc(alias = "CID3")]
pub type Cid3 = crate::Reg<cid3::Cid3Spec>;
#[doc = "CoreSight"]
pub mod cid3;
