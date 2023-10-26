#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status A"]
    pub statusa: STATUSA,
    #[doc = "0x02 - Status B"]
    pub statusb: STATUSB,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - Address"]
    pub addr: ADDR,
    #[doc = "0x08 - Length"]
    pub length: LENGTH,
    #[doc = "0x0c - Data"]
    pub data: DATA,
    #[doc = "0x10..0x18 - Debug Communication Channel n"]
    pub dcc: [DCC; 2],
    #[doc = "0x18 - Device Identification"]
    pub did: DID,
    _reserved8: [u8; 0xd4],
    #[doc = "0xf0..0xf8 - Device Configuration"]
    pub dcfg: [DCFG; 2],
    _reserved9: [u8; 0x0f08],
    #[doc = "0x1000 - CoreSight ROM Table Entry 0"]
    pub entry0: ENTRY0,
    #[doc = "0x1004 - CoreSight ROM Table Entry 1"]
    pub entry1: ENTRY1,
    #[doc = "0x1008 - CoreSight ROM Table End"]
    pub end: END,
    _reserved12: [u8; 0x0fc0],
    #[doc = "0x1fcc - CoreSight ROM Table Memory Type"]
    pub memtype: MEMTYPE,
    #[doc = "0x1fd0 - Peripheral Identification 4"]
    pub pid4: PID4,
    #[doc = "0x1fd4 - Peripheral Identification 5"]
    pub pid5: PID5,
    #[doc = "0x1fd8 - Peripheral Identification 6"]
    pub pid6: PID6,
    #[doc = "0x1fdc - Peripheral Identification 7"]
    pub pid7: PID7,
    #[doc = "0x1fe0 - Peripheral Identification 0"]
    pub pid0: PID0,
    #[doc = "0x1fe4 - Peripheral Identification 1"]
    pub pid1: PID1,
    #[doc = "0x1fe8 - Peripheral Identification 2"]
    pub pid2: PID2,
    #[doc = "0x1fec - Peripheral Identification 3"]
    pub pid3: PID3,
    #[doc = "0x1ff0 - Component Identification 0"]
    pub cid0: CID0,
    #[doc = "0x1ff4 - Component Identification 1"]
    pub cid1: CID1,
    #[doc = "0x1ff8 - Component Identification 2"]
    pub cid2: CID2,
    #[doc = "0x1ffc - Component Identification 3"]
    pub cid3: CID3,
}
#[doc = "CTRL (w) register accessor: Control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STATUSA (rw) register accessor: Status A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statusa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusa`]
module"]
pub type STATUSA = crate::Reg<statusa::STATUSA_SPEC>;
#[doc = "Status A"]
pub mod statusa;
#[doc = "STATUSB (r) register accessor: Status B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusb`]
module"]
pub type STATUSB = crate::Reg<statusb::STATUSB_SPEC>;
#[doc = "Status B"]
pub mod statusb;
#[doc = "ADDR (rw) register accessor: Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address"]
pub mod addr;
#[doc = "LENGTH (rw) register accessor: Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@length`]
module"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "Length"]
pub mod length;
#[doc = "DATA (rw) register accessor: Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data"]
pub mod data;
#[doc = "DCC (rw) register accessor: Debug Communication Channel n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcc`]
module"]
pub type DCC = crate::Reg<dcc::DCC_SPEC>;
#[doc = "Debug Communication Channel n"]
pub mod dcc;
#[doc = "DID (r) register accessor: Device Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`did::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@did`]
module"]
pub type DID = crate::Reg<did::DID_SPEC>;
#[doc = "Device Identification"]
pub mod did;
#[doc = "DCFG (rw) register accessor: Device Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration"]
pub mod dcfg;
#[doc = "ENTRY0 (r) register accessor: CoreSight ROM Table Entry 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry0`]
module"]
pub type ENTRY0 = crate::Reg<entry0::ENTRY0_SPEC>;
#[doc = "CoreSight ROM Table Entry 0"]
pub mod entry0;
#[doc = "ENTRY1 (r) register accessor: CoreSight ROM Table Entry 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry1`]
module"]
pub type ENTRY1 = crate::Reg<entry1::ENTRY1_SPEC>;
#[doc = "CoreSight ROM Table Entry 1"]
pub mod entry1;
#[doc = "END (r) register accessor: CoreSight ROM Table End\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`end::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end`]
module"]
pub type END = crate::Reg<end::END_SPEC>;
#[doc = "CoreSight ROM Table End"]
pub mod end;
#[doc = "MEMTYPE (r) register accessor: CoreSight ROM Table Memory Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memtype::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memtype`]
module"]
pub type MEMTYPE = crate::Reg<memtype::MEMTYPE_SPEC>;
#[doc = "CoreSight ROM Table Memory Type"]
pub mod memtype;
#[doc = "PID4 (r) register accessor: Peripheral Identification 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid4`]
module"]
pub type PID4 = crate::Reg<pid4::PID4_SPEC>;
#[doc = "Peripheral Identification 4"]
pub mod pid4;
#[doc = "PID5 (r) register accessor: Peripheral Identification 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid5`]
module"]
pub type PID5 = crate::Reg<pid5::PID5_SPEC>;
#[doc = "Peripheral Identification 5"]
pub mod pid5;
#[doc = "PID6 (r) register accessor: Peripheral Identification 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid6`]
module"]
pub type PID6 = crate::Reg<pid6::PID6_SPEC>;
#[doc = "Peripheral Identification 6"]
pub mod pid6;
#[doc = "PID7 (r) register accessor: Peripheral Identification 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid7`]
module"]
pub type PID7 = crate::Reg<pid7::PID7_SPEC>;
#[doc = "Peripheral Identification 7"]
pub mod pid7;
#[doc = "PID0 (r) register accessor: Peripheral Identification 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid0`]
module"]
pub type PID0 = crate::Reg<pid0::PID0_SPEC>;
#[doc = "Peripheral Identification 0"]
pub mod pid0;
#[doc = "PID1 (r) register accessor: Peripheral Identification 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid1`]
module"]
pub type PID1 = crate::Reg<pid1::PID1_SPEC>;
#[doc = "Peripheral Identification 1"]
pub mod pid1;
#[doc = "PID2 (r) register accessor: Peripheral Identification 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid2`]
module"]
pub type PID2 = crate::Reg<pid2::PID2_SPEC>;
#[doc = "Peripheral Identification 2"]
pub mod pid2;
#[doc = "PID3 (r) register accessor: Peripheral Identification 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid3`]
module"]
pub type PID3 = crate::Reg<pid3::PID3_SPEC>;
#[doc = "Peripheral Identification 3"]
pub mod pid3;
#[doc = "CID0 (r) register accessor: Component Identification 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid0`]
module"]
pub type CID0 = crate::Reg<cid0::CID0_SPEC>;
#[doc = "Component Identification 0"]
pub mod cid0;
#[doc = "CID1 (r) register accessor: Component Identification 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid1`]
module"]
pub type CID1 = crate::Reg<cid1::CID1_SPEC>;
#[doc = "Component Identification 1"]
pub mod cid1;
#[doc = "CID2 (r) register accessor: Component Identification 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid2`]
module"]
pub type CID2 = crate::Reg<cid2::CID2_SPEC>;
#[doc = "Component Identification 2"]
pub mod cid2;
#[doc = "CID3 (r) register accessor: Component Identification 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid3`]
module"]
pub type CID3 = crate::Reg<cid3::CID3_SPEC>;
#[doc = "Component Identification 3"]
pub mod cid3;
