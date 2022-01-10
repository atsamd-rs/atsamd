#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB Position"]
    pub position: crate::Reg<position::POSITION_SPEC>,
    #[doc = "0x04 - MTB Master"]
    pub master: crate::Reg<master::MASTER_SPEC>,
    #[doc = "0x08 - MTB Flow"]
    pub flow: crate::Reg<flow::FLOW_SPEC>,
    #[doc = "0x0c - MTB Base"]
    pub base: crate::Reg<base::BASE_SPEC>,
    _reserved4: [u8; 0x0ef0],
    #[doc = "0xf00 - MTB Integration Mode Control"]
    pub itctrl: crate::Reg<itctrl::ITCTRL_SPEC>,
    _reserved5: [u8; 0x9c],
    #[doc = "0xfa0 - MTB Claim Set"]
    pub claimset: crate::Reg<claimset::CLAIMSET_SPEC>,
    #[doc = "0xfa4 - MTB Claim Clear"]
    pub claimclr: crate::Reg<claimclr::CLAIMCLR_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0xfb0 - MTB Lock Access"]
    pub lockaccess: crate::Reg<lockaccess::LOCKACCESS_SPEC>,
    #[doc = "0xfb4 - MTB Lock Status"]
    pub lockstatus: crate::Reg<lockstatus::LOCKSTATUS_SPEC>,
    #[doc = "0xfb8 - MTB Authentication Status"]
    pub authstatus: crate::Reg<authstatus::AUTHSTATUS_SPEC>,
    #[doc = "0xfbc - MTB Device Architecture"]
    pub devarch: crate::Reg<devarch::DEVARCH_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0xfc8 - MTB Device Configuration"]
    pub devid: crate::Reg<devid::DEVID_SPEC>,
    #[doc = "0xfcc - MTB Device Type"]
    pub devtype: crate::Reg<devtype::DEVTYPE_SPEC>,
    #[doc = "0xfd0 - CoreSight"]
    pub pid4: crate::Reg<pid4::PID4_SPEC>,
    #[doc = "0xfd4 - CoreSight"]
    pub pid5: crate::Reg<pid5::PID5_SPEC>,
    #[doc = "0xfd8 - CoreSight"]
    pub pid6: crate::Reg<pid6::PID6_SPEC>,
    #[doc = "0xfdc - CoreSight"]
    pub pid7: crate::Reg<pid7::PID7_SPEC>,
    #[doc = "0xfe0 - CoreSight"]
    pub pid0: crate::Reg<pid0::PID0_SPEC>,
    #[doc = "0xfe4 - CoreSight"]
    pub pid1: crate::Reg<pid1::PID1_SPEC>,
    #[doc = "0xfe8 - CoreSight"]
    pub pid2: crate::Reg<pid2::PID2_SPEC>,
    #[doc = "0xfec - CoreSight"]
    pub pid3: crate::Reg<pid3::PID3_SPEC>,
    #[doc = "0xff0 - CoreSight"]
    pub cid0: crate::Reg<cid0::CID0_SPEC>,
    #[doc = "0xff4 - CoreSight"]
    pub cid1: crate::Reg<cid1::CID1_SPEC>,
    #[doc = "0xff8 - CoreSight"]
    pub cid2: crate::Reg<cid2::CID2_SPEC>,
    #[doc = "0xffc - CoreSight"]
    pub cid3: crate::Reg<cid3::CID3_SPEC>,
}
#[doc = "POSITION register accessor: an alias for `Reg<POSITION_SPEC>`"]
pub type POSITION = crate::Reg<position::POSITION_SPEC>;
#[doc = "MTB Position"]
pub mod position;
#[doc = "MASTER register accessor: an alias for `Reg<MASTER_SPEC>`"]
pub type MASTER = crate::Reg<master::MASTER_SPEC>;
#[doc = "MTB Master"]
pub mod master;
#[doc = "FLOW register accessor: an alias for `Reg<FLOW_SPEC>`"]
pub type FLOW = crate::Reg<flow::FLOW_SPEC>;
#[doc = "MTB Flow"]
pub mod flow;
#[doc = "BASE register accessor: an alias for `Reg<BASE_SPEC>`"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "MTB Base"]
pub mod base;
#[doc = "ITCTRL register accessor: an alias for `Reg<ITCTRL_SPEC>`"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "MTB Integration Mode Control"]
pub mod itctrl;
#[doc = "CLAIMSET register accessor: an alias for `Reg<CLAIMSET_SPEC>`"]
pub type CLAIMSET = crate::Reg<claimset::CLAIMSET_SPEC>;
#[doc = "MTB Claim Set"]
pub mod claimset;
#[doc = "CLAIMCLR register accessor: an alias for `Reg<CLAIMCLR_SPEC>`"]
pub type CLAIMCLR = crate::Reg<claimclr::CLAIMCLR_SPEC>;
#[doc = "MTB Claim Clear"]
pub mod claimclr;
#[doc = "LOCKACCESS register accessor: an alias for `Reg<LOCKACCESS_SPEC>`"]
pub type LOCKACCESS = crate::Reg<lockaccess::LOCKACCESS_SPEC>;
#[doc = "MTB Lock Access"]
pub mod lockaccess;
#[doc = "LOCKSTATUS register accessor: an alias for `Reg<LOCKSTATUS_SPEC>`"]
pub type LOCKSTATUS = crate::Reg<lockstatus::LOCKSTATUS_SPEC>;
#[doc = "MTB Lock Status"]
pub mod lockstatus;
#[doc = "AUTHSTATUS register accessor: an alias for `Reg<AUTHSTATUS_SPEC>`"]
pub type AUTHSTATUS = crate::Reg<authstatus::AUTHSTATUS_SPEC>;
#[doc = "MTB Authentication Status"]
pub mod authstatus;
#[doc = "DEVARCH register accessor: an alias for `Reg<DEVARCH_SPEC>`"]
pub type DEVARCH = crate::Reg<devarch::DEVARCH_SPEC>;
#[doc = "MTB Device Architecture"]
pub mod devarch;
#[doc = "DEVID register accessor: an alias for `Reg<DEVID_SPEC>`"]
pub type DEVID = crate::Reg<devid::DEVID_SPEC>;
#[doc = "MTB Device Configuration"]
pub mod devid;
#[doc = "DEVTYPE register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "MTB Device Type"]
pub mod devtype;
#[doc = "PID4 register accessor: an alias for `Reg<PID4_SPEC>`"]
pub type PID4 = crate::Reg<pid4::PID4_SPEC>;
#[doc = "CoreSight"]
pub mod pid4;
#[doc = "PID5 register accessor: an alias for `Reg<PID5_SPEC>`"]
pub type PID5 = crate::Reg<pid5::PID5_SPEC>;
#[doc = "CoreSight"]
pub mod pid5;
#[doc = "PID6 register accessor: an alias for `Reg<PID6_SPEC>`"]
pub type PID6 = crate::Reg<pid6::PID6_SPEC>;
#[doc = "CoreSight"]
pub mod pid6;
#[doc = "PID7 register accessor: an alias for `Reg<PID7_SPEC>`"]
pub type PID7 = crate::Reg<pid7::PID7_SPEC>;
#[doc = "CoreSight"]
pub mod pid7;
#[doc = "PID0 register accessor: an alias for `Reg<PID0_SPEC>`"]
pub type PID0 = crate::Reg<pid0::PID0_SPEC>;
#[doc = "CoreSight"]
pub mod pid0;
#[doc = "PID1 register accessor: an alias for `Reg<PID1_SPEC>`"]
pub type PID1 = crate::Reg<pid1::PID1_SPEC>;
#[doc = "CoreSight"]
pub mod pid1;
#[doc = "PID2 register accessor: an alias for `Reg<PID2_SPEC>`"]
pub type PID2 = crate::Reg<pid2::PID2_SPEC>;
#[doc = "CoreSight"]
pub mod pid2;
#[doc = "PID3 register accessor: an alias for `Reg<PID3_SPEC>`"]
pub type PID3 = crate::Reg<pid3::PID3_SPEC>;
#[doc = "CoreSight"]
pub mod pid3;
#[doc = "CID0 register accessor: an alias for `Reg<CID0_SPEC>`"]
pub type CID0 = crate::Reg<cid0::CID0_SPEC>;
#[doc = "CoreSight"]
pub mod cid0;
#[doc = "CID1 register accessor: an alias for `Reg<CID1_SPEC>`"]
pub type CID1 = crate::Reg<cid1::CID1_SPEC>;
#[doc = "CoreSight"]
pub mod cid1;
#[doc = "CID2 register accessor: an alias for `Reg<CID2_SPEC>`"]
pub type CID2 = crate::Reg<cid2::CID2_SPEC>;
#[doc = "CoreSight"]
pub mod cid2;
#[doc = "CID3 register accessor: an alias for `Reg<CID3_SPEC>`"]
pub type CID3 = crate::Reg<cid3::CID3_SPEC>;
#[doc = "CoreSight"]
pub mod cid3;
