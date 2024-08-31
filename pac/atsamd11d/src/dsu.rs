#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    statusa: Statusa,
    statusb: Statusb,
    _reserved3: [u8; 0x01],
    addr: Addr,
    length: Length,
    data: Data,
    dcc: [Dcc; 2],
    did: Did,
    _reserved8: [u8; 0xd4],
    dcfg: [Dcfg; 2],
    _reserved9: [u8; 0x0f08],
    entry0: Entry0,
    entry1: Entry1,
    end: End,
    _reserved12: [u8; 0x0fc0],
    memtype: Memtype,
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
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x01 - Status A"]
    #[inline(always)]
    pub const fn statusa(&self) -> &Statusa {
        &self.statusa
    }
    #[doc = "0x02 - Status B"]
    #[inline(always)]
    pub const fn statusb(&self) -> &Statusb {
        &self.statusb
    }
    #[doc = "0x04 - Address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x08 - Length"]
    #[inline(always)]
    pub const fn length(&self) -> &Length {
        &self.length
    }
    #[doc = "0x0c - Data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x10..0x18 - Debug Communication Channel n"]
    #[inline(always)]
    pub const fn dcc(&self, n: usize) -> &Dcc {
        &self.dcc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Debug Communication Channel n"]
    #[inline(always)]
    pub fn dcc_iter(&self) -> impl Iterator<Item = &Dcc> {
        self.dcc.iter()
    }
    #[doc = "0x18 - Device Identification"]
    #[inline(always)]
    pub const fn did(&self) -> &Did {
        &self.did
    }
    #[doc = "0xf0..0xf8 - Device Configuration"]
    #[inline(always)]
    pub const fn dcfg(&self, n: usize) -> &Dcfg {
        &self.dcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf0..0xf8 - Device Configuration"]
    #[inline(always)]
    pub fn dcfg_iter(&self) -> impl Iterator<Item = &Dcfg> {
        self.dcfg.iter()
    }
    #[doc = "0x1000 - CoreSight ROM Table Entry 0"]
    #[inline(always)]
    pub const fn entry0(&self) -> &Entry0 {
        &self.entry0
    }
    #[doc = "0x1004 - CoreSight ROM Table Entry 1"]
    #[inline(always)]
    pub const fn entry1(&self) -> &Entry1 {
        &self.entry1
    }
    #[doc = "0x1008 - CoreSight ROM Table End"]
    #[inline(always)]
    pub const fn end(&self) -> &End {
        &self.end
    }
    #[doc = "0x1fcc - CoreSight ROM Table Memory Type"]
    #[inline(always)]
    pub const fn memtype(&self) -> &Memtype {
        &self.memtype
    }
    #[doc = "0x1fd0 - Peripheral Identification 4"]
    #[inline(always)]
    pub const fn pid4(&self) -> &Pid4 {
        &self.pid4
    }
    #[doc = "0x1fd4 - Peripheral Identification 5"]
    #[inline(always)]
    pub const fn pid5(&self) -> &Pid5 {
        &self.pid5
    }
    #[doc = "0x1fd8 - Peripheral Identification 6"]
    #[inline(always)]
    pub const fn pid6(&self) -> &Pid6 {
        &self.pid6
    }
    #[doc = "0x1fdc - Peripheral Identification 7"]
    #[inline(always)]
    pub const fn pid7(&self) -> &Pid7 {
        &self.pid7
    }
    #[doc = "0x1fe0 - Peripheral Identification 0"]
    #[inline(always)]
    pub const fn pid0(&self) -> &Pid0 {
        &self.pid0
    }
    #[doc = "0x1fe4 - Peripheral Identification 1"]
    #[inline(always)]
    pub const fn pid1(&self) -> &Pid1 {
        &self.pid1
    }
    #[doc = "0x1fe8 - Peripheral Identification 2"]
    #[inline(always)]
    pub const fn pid2(&self) -> &Pid2 {
        &self.pid2
    }
    #[doc = "0x1fec - Peripheral Identification 3"]
    #[inline(always)]
    pub const fn pid3(&self) -> &Pid3 {
        &self.pid3
    }
    #[doc = "0x1ff0 - Component Identification 0"]
    #[inline(always)]
    pub const fn cid0(&self) -> &Cid0 {
        &self.cid0
    }
    #[doc = "0x1ff4 - Component Identification 1"]
    #[inline(always)]
    pub const fn cid1(&self) -> &Cid1 {
        &self.cid1
    }
    #[doc = "0x1ff8 - Component Identification 2"]
    #[inline(always)]
    pub const fn cid2(&self) -> &Cid2 {
        &self.cid2
    }
    #[doc = "0x1ffc - Component Identification 3"]
    #[inline(always)]
    pub const fn cid3(&self) -> &Cid3 {
        &self.cid3
    }
}
#[doc = "CTRL (w) register accessor: Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STATUSA (rw) register accessor: Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusa`]
module"]
#[doc(alias = "STATUSA")]
pub type Statusa = crate::Reg<statusa::StatusaSpec>;
#[doc = "Status A"]
pub mod statusa;
#[doc = "STATUSB (r) register accessor: Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusb`]
module"]
#[doc(alias = "STATUSB")]
pub type Statusb = crate::Reg<statusb::StatusbSpec>;
#[doc = "Status B"]
pub mod statusb;
#[doc = "ADDR (rw) register accessor: Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Address"]
pub mod addr;
#[doc = "LENGTH (rw) register accessor: Length\n\nYou can [`read`](crate::Reg::read) this register and get [`length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@length`]
module"]
#[doc(alias = "LENGTH")]
pub type Length = crate::Reg<length::LengthSpec>;
#[doc = "Length"]
pub mod length;
#[doc = "DATA (rw) register accessor: Data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data"]
pub mod data;
#[doc = "DCC (rw) register accessor: Debug Communication Channel n\n\nYou can [`read`](crate::Reg::read) this register and get [`dcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcc`]
module"]
#[doc(alias = "DCC")]
pub type Dcc = crate::Reg<dcc::DccSpec>;
#[doc = "Debug Communication Channel n"]
pub mod dcc;
#[doc = "DID (r) register accessor: Device Identification\n\nYou can [`read`](crate::Reg::read) this register and get [`did::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@did`]
module"]
#[doc(alias = "DID")]
pub type Did = crate::Reg<did::DidSpec>;
#[doc = "Device Identification"]
pub mod did;
#[doc = "DCFG (rw) register accessor: Device Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "Device Configuration"]
pub mod dcfg;
#[doc = "ENTRY0 (r) register accessor: CoreSight ROM Table Entry 0\n\nYou can [`read`](crate::Reg::read) this register and get [`entry0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry0`]
module"]
#[doc(alias = "ENTRY0")]
pub type Entry0 = crate::Reg<entry0::Entry0Spec>;
#[doc = "CoreSight ROM Table Entry 0"]
pub mod entry0;
#[doc = "ENTRY1 (r) register accessor: CoreSight ROM Table Entry 1\n\nYou can [`read`](crate::Reg::read) this register and get [`entry1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry1`]
module"]
#[doc(alias = "ENTRY1")]
pub type Entry1 = crate::Reg<entry1::Entry1Spec>;
#[doc = "CoreSight ROM Table Entry 1"]
pub mod entry1;
#[doc = "END (r) register accessor: CoreSight ROM Table End\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end`]
module"]
#[doc(alias = "END")]
pub type End = crate::Reg<end::EndSpec>;
#[doc = "CoreSight ROM Table End"]
pub mod end;
#[doc = "MEMTYPE (r) register accessor: CoreSight ROM Table Memory Type\n\nYou can [`read`](crate::Reg::read) this register and get [`memtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memtype`]
module"]
#[doc(alias = "MEMTYPE")]
pub type Memtype = crate::Reg<memtype::MemtypeSpec>;
#[doc = "CoreSight ROM Table Memory Type"]
pub mod memtype;
#[doc = "PID4 (r) register accessor: Peripheral Identification 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pid4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid4`]
module"]
#[doc(alias = "PID4")]
pub type Pid4 = crate::Reg<pid4::Pid4Spec>;
#[doc = "Peripheral Identification 4"]
pub mod pid4;
#[doc = "PID5 (r) register accessor: Peripheral Identification 5\n\nYou can [`read`](crate::Reg::read) this register and get [`pid5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid5`]
module"]
#[doc(alias = "PID5")]
pub type Pid5 = crate::Reg<pid5::Pid5Spec>;
#[doc = "Peripheral Identification 5"]
pub mod pid5;
#[doc = "PID6 (r) register accessor: Peripheral Identification 6\n\nYou can [`read`](crate::Reg::read) this register and get [`pid6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid6`]
module"]
#[doc(alias = "PID6")]
pub type Pid6 = crate::Reg<pid6::Pid6Spec>;
#[doc = "Peripheral Identification 6"]
pub mod pid6;
#[doc = "PID7 (r) register accessor: Peripheral Identification 7\n\nYou can [`read`](crate::Reg::read) this register and get [`pid7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid7`]
module"]
#[doc(alias = "PID7")]
pub type Pid7 = crate::Reg<pid7::Pid7Spec>;
#[doc = "Peripheral Identification 7"]
pub mod pid7;
#[doc = "PID0 (r) register accessor: Peripheral Identification 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid0`]
module"]
#[doc(alias = "PID0")]
pub type Pid0 = crate::Reg<pid0::Pid0Spec>;
#[doc = "Peripheral Identification 0"]
pub mod pid0;
#[doc = "PID1 (r) register accessor: Peripheral Identification 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid1`]
module"]
#[doc(alias = "PID1")]
pub type Pid1 = crate::Reg<pid1::Pid1Spec>;
#[doc = "Peripheral Identification 1"]
pub mod pid1;
#[doc = "PID2 (r) register accessor: Peripheral Identification 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid2`]
module"]
#[doc(alias = "PID2")]
pub type Pid2 = crate::Reg<pid2::Pid2Spec>;
#[doc = "Peripheral Identification 2"]
pub mod pid2;
#[doc = "PID3 (r) register accessor: Peripheral Identification 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid3`]
module"]
#[doc(alias = "PID3")]
pub type Pid3 = crate::Reg<pid3::Pid3Spec>;
#[doc = "Peripheral Identification 3"]
pub mod pid3;
#[doc = "CID0 (r) register accessor: Component Identification 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid0`]
module"]
#[doc(alias = "CID0")]
pub type Cid0 = crate::Reg<cid0::Cid0Spec>;
#[doc = "Component Identification 0"]
pub mod cid0;
#[doc = "CID1 (r) register accessor: Component Identification 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid1`]
module"]
#[doc(alias = "CID1")]
pub type Cid1 = crate::Reg<cid1::Cid1Spec>;
#[doc = "Component Identification 1"]
pub mod cid1;
#[doc = "CID2 (r) register accessor: Component Identification 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid2`]
module"]
#[doc(alias = "CID2")]
pub type Cid2 = crate::Reg<cid2::Cid2Spec>;
#[doc = "Component Identification 2"]
pub mod cid2;
#[doc = "CID3 (r) register accessor: Component Identification 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid3`]
module"]
#[doc(alias = "CID3")]
pub type Cid3 = crate::Reg<cid3::Cid3Spec>;
#[doc = "Component Identification 3"]
pub mod cid3;
