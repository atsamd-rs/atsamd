#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x01 - Status A"]
    pub statusa: crate::Reg<statusa::STATUSA_SPEC>,
    #[doc = "0x02 - Status B"]
    pub statusb: crate::Reg<statusb::STATUSB_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - Address"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x08 - Length"]
    pub length: crate::Reg<length::LENGTH_SPEC>,
    #[doc = "0x0c - Data"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x10..0x18 - Debug Communication Channel n"]
    pub dcc: [crate::Reg<dcc::DCC_SPEC>; 2],
    #[doc = "0x18 - Device Identification"]
    pub did: crate::Reg<did::DID_SPEC>,
    _reserved8: [u8; 0xd4],
    #[doc = "0xf0..0xf8 - Device Configuration"]
    pub dcfg: [crate::Reg<dcfg::DCFG_SPEC>; 2],
    _reserved9: [u8; 0x0f08],
    #[doc = "0x1000 - CoreSight ROM Table Entry 0"]
    pub entry0: crate::Reg<entry0::ENTRY0_SPEC>,
    #[doc = "0x1004 - CoreSight ROM Table Entry 1"]
    pub entry1: crate::Reg<entry1::ENTRY1_SPEC>,
    #[doc = "0x1008 - CoreSight ROM Table End"]
    pub end: crate::Reg<end::END_SPEC>,
    _reserved12: [u8; 0x0fc0],
    #[doc = "0x1fcc - CoreSight ROM Table Memory Type"]
    pub memtype: crate::Reg<memtype::MEMTYPE_SPEC>,
    #[doc = "0x1fd0 - Peripheral Identification 4"]
    pub pid4: crate::Reg<pid4::PID4_SPEC>,
    #[doc = "0x1fd4 - Peripheral Identification 5"]
    pub pid5: crate::Reg<pid5::PID5_SPEC>,
    #[doc = "0x1fd8 - Peripheral Identification 6"]
    pub pid6: crate::Reg<pid6::PID6_SPEC>,
    #[doc = "0x1fdc - Peripheral Identification 7"]
    pub pid7: crate::Reg<pid7::PID7_SPEC>,
    #[doc = "0x1fe0 - Peripheral Identification 0"]
    pub pid0: crate::Reg<pid0::PID0_SPEC>,
    #[doc = "0x1fe4 - Peripheral Identification 1"]
    pub pid1: crate::Reg<pid1::PID1_SPEC>,
    #[doc = "0x1fe8 - Peripheral Identification 2"]
    pub pid2: crate::Reg<pid2::PID2_SPEC>,
    #[doc = "0x1fec - Peripheral Identification 3"]
    pub pid3: crate::Reg<pid3::PID3_SPEC>,
    #[doc = "0x1ff0 - Component Identification 0"]
    pub cid0: crate::Reg<cid0::CID0_SPEC>,
    #[doc = "0x1ff4 - Component Identification 1"]
    pub cid1: crate::Reg<cid1::CID1_SPEC>,
    #[doc = "0x1ff8 - Component Identification 2"]
    pub cid2: crate::Reg<cid2::CID2_SPEC>,
    #[doc = "0x1ffc - Component Identification 3"]
    pub cid3: crate::Reg<cid3::CID3_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STATUSA register accessor: an alias for `Reg<STATUSA_SPEC>`"]
pub type STATUSA = crate::Reg<statusa::STATUSA_SPEC>;
#[doc = "Status A"]
pub mod statusa;
#[doc = "STATUSB register accessor: an alias for `Reg<STATUSB_SPEC>`"]
pub type STATUSB = crate::Reg<statusb::STATUSB_SPEC>;
#[doc = "Status B"]
pub mod statusb;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address"]
pub mod addr;
#[doc = "LENGTH register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "Length"]
pub mod length;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data"]
pub mod data;
#[doc = "DCC register accessor: an alias for `Reg<DCC_SPEC>`"]
pub type DCC = crate::Reg<dcc::DCC_SPEC>;
#[doc = "Debug Communication Channel n"]
pub mod dcc;
#[doc = "DID register accessor: an alias for `Reg<DID_SPEC>`"]
pub type DID = crate::Reg<did::DID_SPEC>;
#[doc = "Device Identification"]
pub mod did;
#[doc = "DCFG register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration"]
pub mod dcfg;
#[doc = "ENTRY0 register accessor: an alias for `Reg<ENTRY0_SPEC>`"]
pub type ENTRY0 = crate::Reg<entry0::ENTRY0_SPEC>;
#[doc = "CoreSight ROM Table Entry 0"]
pub mod entry0;
#[doc = "ENTRY1 register accessor: an alias for `Reg<ENTRY1_SPEC>`"]
pub type ENTRY1 = crate::Reg<entry1::ENTRY1_SPEC>;
#[doc = "CoreSight ROM Table Entry 1"]
pub mod entry1;
#[doc = "END register accessor: an alias for `Reg<END_SPEC>`"]
pub type END = crate::Reg<end::END_SPEC>;
#[doc = "CoreSight ROM Table End"]
pub mod end;
#[doc = "MEMTYPE register accessor: an alias for `Reg<MEMTYPE_SPEC>`"]
pub type MEMTYPE = crate::Reg<memtype::MEMTYPE_SPEC>;
#[doc = "CoreSight ROM Table Memory Type"]
pub mod memtype;
#[doc = "PID4 register accessor: an alias for `Reg<PID4_SPEC>`"]
pub type PID4 = crate::Reg<pid4::PID4_SPEC>;
#[doc = "Peripheral Identification 4"]
pub mod pid4;
#[doc = "PID5 register accessor: an alias for `Reg<PID5_SPEC>`"]
pub type PID5 = crate::Reg<pid5::PID5_SPEC>;
#[doc = "Peripheral Identification 5"]
pub mod pid5;
#[doc = "PID6 register accessor: an alias for `Reg<PID6_SPEC>`"]
pub type PID6 = crate::Reg<pid6::PID6_SPEC>;
#[doc = "Peripheral Identification 6"]
pub mod pid6;
#[doc = "PID7 register accessor: an alias for `Reg<PID7_SPEC>`"]
pub type PID7 = crate::Reg<pid7::PID7_SPEC>;
#[doc = "Peripheral Identification 7"]
pub mod pid7;
#[doc = "PID0 register accessor: an alias for `Reg<PID0_SPEC>`"]
pub type PID0 = crate::Reg<pid0::PID0_SPEC>;
#[doc = "Peripheral Identification 0"]
pub mod pid0;
#[doc = "PID1 register accessor: an alias for `Reg<PID1_SPEC>`"]
pub type PID1 = crate::Reg<pid1::PID1_SPEC>;
#[doc = "Peripheral Identification 1"]
pub mod pid1;
#[doc = "PID2 register accessor: an alias for `Reg<PID2_SPEC>`"]
pub type PID2 = crate::Reg<pid2::PID2_SPEC>;
#[doc = "Peripheral Identification 2"]
pub mod pid2;
#[doc = "PID3 register accessor: an alias for `Reg<PID3_SPEC>`"]
pub type PID3 = crate::Reg<pid3::PID3_SPEC>;
#[doc = "Peripheral Identification 3"]
pub mod pid3;
#[doc = "CID0 register accessor: an alias for `Reg<CID0_SPEC>`"]
pub type CID0 = crate::Reg<cid0::CID0_SPEC>;
#[doc = "Component Identification 0"]
pub mod cid0;
#[doc = "CID1 register accessor: an alias for `Reg<CID1_SPEC>`"]
pub type CID1 = crate::Reg<cid1::CID1_SPEC>;
#[doc = "Component Identification 1"]
pub mod cid1;
#[doc = "CID2 register accessor: an alias for `Reg<CID2_SPEC>`"]
pub type CID2 = crate::Reg<cid2::CID2_SPEC>;
#[doc = "Component Identification 2"]
pub mod cid2;
#[doc = "CID3 register accessor: an alias for `Reg<CID3_SPEC>`"]
pub type CID3 = crate::Reg<cid3::CID3_SPEC>;
#[doc = "Component Identification 3"]
pub mod cid3;
