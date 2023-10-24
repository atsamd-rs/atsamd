#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB Position"]
    pub position: POSITION,
    #[doc = "0x04 - MTB Master"]
    pub master: MASTER,
    #[doc = "0x08 - MTB Flow"]
    pub flow: FLOW,
    #[doc = "0x0c - MTB Base"]
    pub base: BASE,
    _reserved4: [u8; 0x0ef0],
    #[doc = "0xf00 - MTB Integration Mode Control"]
    pub itctrl: ITCTRL,
    _reserved5: [u8; 0x9c],
    #[doc = "0xfa0 - MTB Claim Set"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - MTB Claim Clear"]
    pub claimclr: CLAIMCLR,
    _reserved7: [u8; 0x08],
    #[doc = "0xfb0 - MTB Lock Access"]
    pub lockaccess: LOCKACCESS,
    #[doc = "0xfb4 - MTB Lock Status"]
    pub lockstatus: LOCKSTATUS,
    #[doc = "0xfb8 - MTB Authentication Status"]
    pub authstatus: AUTHSTATUS,
    #[doc = "0xfbc - MTB Device Architecture"]
    pub devarch: DEVARCH,
    _reserved11: [u8; 0x08],
    #[doc = "0xfc8 - MTB Device Configuration"]
    pub devid: DEVID,
    #[doc = "0xfcc - MTB Device Type"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - CoreSight"]
    pub pid4: PID4,
    #[doc = "0xfd4 - CoreSight"]
    pub pid5: PID5,
    #[doc = "0xfd8 - CoreSight"]
    pub pid6: PID6,
    #[doc = "0xfdc - CoreSight"]
    pub pid7: PID7,
    #[doc = "0xfe0 - CoreSight"]
    pub pid0: PID0,
    #[doc = "0xfe4 - CoreSight"]
    pub pid1: PID1,
    #[doc = "0xfe8 - CoreSight"]
    pub pid2: PID2,
    #[doc = "0xfec - CoreSight"]
    pub pid3: PID3,
    #[doc = "0xff0 - CoreSight"]
    pub cid0: CID0,
    #[doc = "0xff4 - CoreSight"]
    pub cid1: CID1,
    #[doc = "0xff8 - CoreSight"]
    pub cid2: CID2,
    #[doc = "0xffc - CoreSight"]
    pub cid3: CID3,
}
#[doc = "POSITION (rw) register accessor: MTB Position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`position::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`position::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@position`]
module"]
pub type POSITION = crate::Reg<position::POSITION_SPEC>;
#[doc = "MTB Position"]
pub mod position;
#[doc = "MASTER (rw) register accessor: MTB Master\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`master::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`master::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master`]
module"]
pub type MASTER = crate::Reg<master::MASTER_SPEC>;
#[doc = "MTB Master"]
pub mod master;
#[doc = "FLOW (rw) register accessor: MTB Flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow`]
module"]
pub type FLOW = crate::Reg<flow::FLOW_SPEC>;
#[doc = "MTB Flow"]
pub mod flow;
#[doc = "BASE (r) register accessor: MTB Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base`]
module"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "MTB Base"]
pub mod base;
#[doc = "ITCTRL (rw) register accessor: MTB Integration Mode Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itctrl`]
module"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "MTB Integration Mode Control"]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: MTB Claim Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimset`]
module"]
pub type CLAIMSET = crate::Reg<claimset::CLAIMSET_SPEC>;
#[doc = "MTB Claim Set"]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: MTB Claim Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimclr`]
module"]
pub type CLAIMCLR = crate::Reg<claimclr::CLAIMCLR_SPEC>;
#[doc = "MTB Claim Clear"]
pub mod claimclr;
#[doc = "LOCKACCESS (rw) register accessor: MTB Lock Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockaccess::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockaccess::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockaccess`]
module"]
pub type LOCKACCESS = crate::Reg<lockaccess::LOCKACCESS_SPEC>;
#[doc = "MTB Lock Access"]
pub mod lockaccess;
#[doc = "LOCKSTATUS (r) register accessor: MTB Lock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockstatus`]
module"]
pub type LOCKSTATUS = crate::Reg<lockstatus::LOCKSTATUS_SPEC>;
#[doc = "MTB Lock Status"]
pub mod lockstatus;
#[doc = "AUTHSTATUS (r) register accessor: MTB Authentication Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`authstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authstatus`]
module"]
pub type AUTHSTATUS = crate::Reg<authstatus::AUTHSTATUS_SPEC>;
#[doc = "MTB Authentication Status"]
pub mod authstatus;
#[doc = "DEVARCH (r) register accessor: MTB Device Architecture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devarch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devarch`]
module"]
pub type DEVARCH = crate::Reg<devarch::DEVARCH_SPEC>;
#[doc = "MTB Device Architecture"]
pub mod devarch;
#[doc = "DEVID (r) register accessor: MTB Device Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devid`]
module"]
pub type DEVID = crate::Reg<devid::DEVID_SPEC>;
#[doc = "MTB Device Configuration"]
pub mod devid;
#[doc = "DEVTYPE (r) register accessor: MTB Device Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devtype::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`]
module"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "MTB Device Type"]
pub mod devtype;
#[doc = "PID4 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid4`]
module"]
pub type PID4 = crate::Reg<pid4::PID4_SPEC>;
#[doc = "CoreSight"]
pub mod pid4;
#[doc = "PID5 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid5`]
module"]
pub type PID5 = crate::Reg<pid5::PID5_SPEC>;
#[doc = "CoreSight"]
pub mod pid5;
#[doc = "PID6 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid6`]
module"]
pub type PID6 = crate::Reg<pid6::PID6_SPEC>;
#[doc = "CoreSight"]
pub mod pid6;
#[doc = "PID7 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid7`]
module"]
pub type PID7 = crate::Reg<pid7::PID7_SPEC>;
#[doc = "CoreSight"]
pub mod pid7;
#[doc = "PID0 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid0`]
module"]
pub type PID0 = crate::Reg<pid0::PID0_SPEC>;
#[doc = "CoreSight"]
pub mod pid0;
#[doc = "PID1 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid1`]
module"]
pub type PID1 = crate::Reg<pid1::PID1_SPEC>;
#[doc = "CoreSight"]
pub mod pid1;
#[doc = "PID2 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid2`]
module"]
pub type PID2 = crate::Reg<pid2::PID2_SPEC>;
#[doc = "CoreSight"]
pub mod pid2;
#[doc = "PID3 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid3`]
module"]
pub type PID3 = crate::Reg<pid3::PID3_SPEC>;
#[doc = "CoreSight"]
pub mod pid3;
#[doc = "CID0 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid0`]
module"]
pub type CID0 = crate::Reg<cid0::CID0_SPEC>;
#[doc = "CoreSight"]
pub mod cid0;
#[doc = "CID1 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid1`]
module"]
pub type CID1 = crate::Reg<cid1::CID1_SPEC>;
#[doc = "CoreSight"]
pub mod cid1;
#[doc = "CID2 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid2`]
module"]
pub type CID2 = crate::Reg<cid2::CID2_SPEC>;
#[doc = "CoreSight"]
pub mod cid2;
#[doc = "CID3 (r) register accessor: CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid3`]
module"]
pub type CID3 = crate::Reg<cid3::CID3_SPEC>;
#[doc = "CoreSight"]
pub mod cid3;
