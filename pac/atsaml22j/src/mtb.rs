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
    #[doc = "0xfd0 - Peripheral Identification 4"]
    pub pid4: PID4,
    #[doc = "0xfd4 - Peripheral Identification 5"]
    pub pid5: PID5,
    #[doc = "0xfd8 - Peripheral Identification 6"]
    pub pid6: PID6,
    #[doc = "0xfdc - Peripheral Identification 7"]
    pub pid7: PID7,
    #[doc = "0xfe0 - Peripheral Identification 0"]
    pub pid0: PID0,
    #[doc = "0xfe4 - Peripheral Identification 1"]
    pub pid1: PID1,
    #[doc = "0xfe8 - Peripheral Identification 2"]
    pub pid2: PID2,
    #[doc = "0xfec - Peripheral Identification 3"]
    pub pid3: PID3,
    #[doc = "0xff0 - Component Identification 0"]
    pub cid0: CID0,
    #[doc = "0xff4 - Component Identification 1"]
    pub cid1: CID1,
    #[doc = "0xff8 - Component Identification 2"]
    pub cid2: CID2,
    #[doc = "0xffc - Component Identification 3"]
    pub cid3: CID3,
}
#[doc = "POSITION (rw) register accessor: an alias for `Reg<POSITION_SPEC>`"]
pub type POSITION = crate::Reg<position::POSITION_SPEC>;
#[doc = "MTB Position"]
pub mod position;
#[doc = "MASTER (rw) register accessor: an alias for `Reg<MASTER_SPEC>`"]
pub type MASTER = crate::Reg<master::MASTER_SPEC>;
#[doc = "MTB Master"]
pub mod master;
#[doc = "FLOW (rw) register accessor: an alias for `Reg<FLOW_SPEC>`"]
pub type FLOW = crate::Reg<flow::FLOW_SPEC>;
#[doc = "MTB Flow"]
pub mod flow;
#[doc = "BASE (r) register accessor: an alias for `Reg<BASE_SPEC>`"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "MTB Base"]
pub mod base;
#[doc = "ITCTRL (rw) register accessor: an alias for `Reg<ITCTRL_SPEC>`"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "MTB Integration Mode Control"]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: an alias for `Reg<CLAIMSET_SPEC>`"]
pub type CLAIMSET = crate::Reg<claimset::CLAIMSET_SPEC>;
#[doc = "MTB Claim Set"]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: an alias for `Reg<CLAIMCLR_SPEC>`"]
pub type CLAIMCLR = crate::Reg<claimclr::CLAIMCLR_SPEC>;
#[doc = "MTB Claim Clear"]
pub mod claimclr;
#[doc = "LOCKACCESS (rw) register accessor: an alias for `Reg<LOCKACCESS_SPEC>`"]
pub type LOCKACCESS = crate::Reg<lockaccess::LOCKACCESS_SPEC>;
#[doc = "MTB Lock Access"]
pub mod lockaccess;
#[doc = "LOCKSTATUS (r) register accessor: an alias for `Reg<LOCKSTATUS_SPEC>`"]
pub type LOCKSTATUS = crate::Reg<lockstatus::LOCKSTATUS_SPEC>;
#[doc = "MTB Lock Status"]
pub mod lockstatus;
#[doc = "AUTHSTATUS (r) register accessor: an alias for `Reg<AUTHSTATUS_SPEC>`"]
pub type AUTHSTATUS = crate::Reg<authstatus::AUTHSTATUS_SPEC>;
#[doc = "MTB Authentication Status"]
pub mod authstatus;
#[doc = "DEVARCH (r) register accessor: an alias for `Reg<DEVARCH_SPEC>`"]
pub type DEVARCH = crate::Reg<devarch::DEVARCH_SPEC>;
#[doc = "MTB Device Architecture"]
pub mod devarch;
#[doc = "DEVID (r) register accessor: an alias for `Reg<DEVID_SPEC>`"]
pub type DEVID = crate::Reg<devid::DEVID_SPEC>;
#[doc = "MTB Device Configuration"]
pub mod devid;
#[doc = "DEVTYPE (r) register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "MTB Device Type"]
pub mod devtype;
#[doc = "PID4 (r) register accessor: an alias for `Reg<PID4_SPEC>`"]
pub type PID4 = crate::Reg<pid4::PID4_SPEC>;
#[doc = "Peripheral Identification 4"]
pub mod pid4;
#[doc = "PID5 (r) register accessor: an alias for `Reg<PID5_SPEC>`"]
pub type PID5 = crate::Reg<pid5::PID5_SPEC>;
#[doc = "Peripheral Identification 5"]
pub mod pid5;
#[doc = "PID6 (r) register accessor: an alias for `Reg<PID6_SPEC>`"]
pub type PID6 = crate::Reg<pid6::PID6_SPEC>;
#[doc = "Peripheral Identification 6"]
pub mod pid6;
#[doc = "PID7 (r) register accessor: an alias for `Reg<PID7_SPEC>`"]
pub type PID7 = crate::Reg<pid7::PID7_SPEC>;
#[doc = "Peripheral Identification 7"]
pub mod pid7;
#[doc = "PID0 (r) register accessor: an alias for `Reg<PID0_SPEC>`"]
pub type PID0 = crate::Reg<pid0::PID0_SPEC>;
#[doc = "Peripheral Identification 0"]
pub mod pid0;
#[doc = "PID1 (r) register accessor: an alias for `Reg<PID1_SPEC>`"]
pub type PID1 = crate::Reg<pid1::PID1_SPEC>;
#[doc = "Peripheral Identification 1"]
pub mod pid1;
#[doc = "PID2 (r) register accessor: an alias for `Reg<PID2_SPEC>`"]
pub type PID2 = crate::Reg<pid2::PID2_SPEC>;
#[doc = "Peripheral Identification 2"]
pub mod pid2;
#[doc = "PID3 (r) register accessor: an alias for `Reg<PID3_SPEC>`"]
pub type PID3 = crate::Reg<pid3::PID3_SPEC>;
#[doc = "Peripheral Identification 3"]
pub mod pid3;
#[doc = "CID0 (r) register accessor: an alias for `Reg<CID0_SPEC>`"]
pub type CID0 = crate::Reg<cid0::CID0_SPEC>;
#[doc = "Component Identification 0"]
pub mod cid0;
#[doc = "CID1 (r) register accessor: an alias for `Reg<CID1_SPEC>`"]
pub type CID1 = crate::Reg<cid1::CID1_SPEC>;
#[doc = "Component Identification 1"]
pub mod cid1;
#[doc = "CID2 (r) register accessor: an alias for `Reg<CID2_SPEC>`"]
pub type CID2 = crate::Reg<cid2::CID2_SPEC>;
#[doc = "Component Identification 2"]
pub mod cid2;
#[doc = "CID3 (r) register accessor: an alias for `Reg<CID3_SPEC>`"]
pub type CID3 = crate::Reg<cid3::CID3_SPEC>;
#[doc = "Component Identification 3"]
pub mod cid3;
