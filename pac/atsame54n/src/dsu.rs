#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status A"]
    pub statusa: STATUSA,
    #[doc = "0x02 - Status B"]
    pub statusb: STATUSB,
    _reserved3: [u8; 1usize],
    #[doc = "0x04 - Address"]
    pub addr: ADDR,
    #[doc = "0x08 - Length"]
    pub length: LENGTH,
    #[doc = "0x0c - Data"]
    pub data: DATA,
    #[doc = "0x10 - Debug Communication Channel n"]
    pub dcc: [DCC; 2],
    #[doc = "0x18 - Device Identification"]
    pub did: DID,
    #[doc = "0x1c - Configuration"]
    pub cfg: CFG,
    _reserved9: [u8; 208usize],
    #[doc = "0xf0 - Device Configuration"]
    pub dcfg: [DCFG; 2],
    _reserved10: [u8; 3848usize],
    #[doc = "0x1000 - CoreSight ROM Table Entry 0"]
    pub entry0: ENTRY0,
    #[doc = "0x1004 - CoreSight ROM Table Entry 1"]
    pub entry1: ENTRY1,
    #[doc = "0x1008 - CoreSight ROM Table End"]
    pub end: END,
    _reserved13: [u8; 4032usize],
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
#[doc = "Control\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u8, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Status A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusa](statusa) module"]
pub type STATUSA = crate::Reg<u8, _STATUSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSA;
#[doc = "`read()` method returns [statusa::R](statusa::R) reader structure"]
impl crate::Readable for STATUSA {}
#[doc = "`write(|w| ..)` method takes [statusa::W](statusa::W) writer structure"]
impl crate::Writable for STATUSA {}
#[doc = "Status A"]
pub mod statusa;
#[doc = "Status B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusb](statusb) module"]
pub type STATUSB = crate::Reg<u8, _STATUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSB;
#[doc = "`read()` method returns [statusb::R](statusb::R) reader structure"]
impl crate::Readable for STATUSB {}
#[doc = "Status B"]
pub mod statusb;
#[doc = "Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Address"]
pub mod addr;
#[doc = "Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [length](length) module"]
pub type LENGTH = crate::Reg<u32, _LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LENGTH;
#[doc = "`read()` method returns [length::R](length::R) reader structure"]
impl crate::Readable for LENGTH {}
#[doc = "`write(|w| ..)` method takes [length::W](length::W) writer structure"]
impl crate::Writable for LENGTH {}
#[doc = "Length"]
pub mod length;
#[doc = "Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Data"]
pub mod data;
#[doc = "Debug Communication Channel n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcc](dcc) module"]
pub type DCC = crate::Reg<u32, _DCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCC;
#[doc = "`read()` method returns [dcc::R](dcc::R) reader structure"]
impl crate::Readable for DCC {}
#[doc = "`write(|w| ..)` method takes [dcc::W](dcc::W) writer structure"]
impl crate::Writable for DCC {}
#[doc = "Debug Communication Channel n"]
pub mod dcc;
#[doc = "Device Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did](did) module"]
pub type DID = crate::Reg<u32, _DID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DID;
#[doc = "`read()` method returns [did::R](did::R) reader structure"]
impl crate::Readable for DID {}
#[doc = "Device Identification"]
pub mod did;
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration"]
pub mod cfg;
#[doc = "Device Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](dcfg) module"]
pub type DCFG = crate::Reg<u32, _DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG;
#[doc = "`read()` method returns [dcfg::R](dcfg::R) reader structure"]
impl crate::Readable for DCFG {}
#[doc = "`write(|w| ..)` method takes [dcfg::W](dcfg::W) writer structure"]
impl crate::Writable for DCFG {}
#[doc = "Device Configuration"]
pub mod dcfg;
#[doc = "CoreSight ROM Table Entry 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [entry0](entry0) module"]
pub type ENTRY0 = crate::Reg<u32, _ENTRY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENTRY0;
#[doc = "`read()` method returns [entry0::R](entry0::R) reader structure"]
impl crate::Readable for ENTRY0 {}
#[doc = "CoreSight ROM Table Entry 0"]
pub mod entry0;
#[doc = "CoreSight ROM Table Entry 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [entry1](entry1) module"]
pub type ENTRY1 = crate::Reg<u32, _ENTRY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENTRY1;
#[doc = "`read()` method returns [entry1::R](entry1::R) reader structure"]
impl crate::Readable for ENTRY1 {}
#[doc = "CoreSight ROM Table Entry 1"]
pub mod entry1;
#[doc = "CoreSight ROM Table End\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [end](end) module"]
pub type END = crate::Reg<u32, _END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _END;
#[doc = "`read()` method returns [end::R](end::R) reader structure"]
impl crate::Readable for END {}
#[doc = "CoreSight ROM Table End"]
pub mod end;
#[doc = "CoreSight ROM Table Memory Type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memtype](memtype) module"]
pub type MEMTYPE = crate::Reg<u32, _MEMTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMTYPE;
#[doc = "`read()` method returns [memtype::R](memtype::R) reader structure"]
impl crate::Readable for MEMTYPE {}
#[doc = "CoreSight ROM Table Memory Type"]
pub mod memtype;
#[doc = "Peripheral Identification 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid4](pid4) module"]
pub type PID4 = crate::Reg<u32, _PID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID4;
#[doc = "`read()` method returns [pid4::R](pid4::R) reader structure"]
impl crate::Readable for PID4 {}
#[doc = "Peripheral Identification 4"]
pub mod pid4;
#[doc = "Peripheral Identification 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid5](pid5) module"]
pub type PID5 = crate::Reg<u32, _PID5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID5;
#[doc = "`read()` method returns [pid5::R](pid5::R) reader structure"]
impl crate::Readable for PID5 {}
#[doc = "Peripheral Identification 5"]
pub mod pid5;
#[doc = "Peripheral Identification 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid6](pid6) module"]
pub type PID6 = crate::Reg<u32, _PID6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID6;
#[doc = "`read()` method returns [pid6::R](pid6::R) reader structure"]
impl crate::Readable for PID6 {}
#[doc = "Peripheral Identification 6"]
pub mod pid6;
#[doc = "Peripheral Identification 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid7](pid7) module"]
pub type PID7 = crate::Reg<u32, _PID7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID7;
#[doc = "`read()` method returns [pid7::R](pid7::R) reader structure"]
impl crate::Readable for PID7 {}
#[doc = "Peripheral Identification 7"]
pub mod pid7;
#[doc = "Peripheral Identification 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid0](pid0) module"]
pub type PID0 = crate::Reg<u32, _PID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID0;
#[doc = "`read()` method returns [pid0::R](pid0::R) reader structure"]
impl crate::Readable for PID0 {}
#[doc = "Peripheral Identification 0"]
pub mod pid0;
#[doc = "Peripheral Identification 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid1](pid1) module"]
pub type PID1 = crate::Reg<u32, _PID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID1;
#[doc = "`read()` method returns [pid1::R](pid1::R) reader structure"]
impl crate::Readable for PID1 {}
#[doc = "Peripheral Identification 1"]
pub mod pid1;
#[doc = "Peripheral Identification 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid2](pid2) module"]
pub type PID2 = crate::Reg<u32, _PID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID2;
#[doc = "`read()` method returns [pid2::R](pid2::R) reader structure"]
impl crate::Readable for PID2 {}
#[doc = "Peripheral Identification 2"]
pub mod pid2;
#[doc = "Peripheral Identification 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid3](pid3) module"]
pub type PID3 = crate::Reg<u32, _PID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID3;
#[doc = "`read()` method returns [pid3::R](pid3::R) reader structure"]
impl crate::Readable for PID3 {}
#[doc = "Peripheral Identification 3"]
pub mod pid3;
#[doc = "Component Identification 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid0](cid0) module"]
pub type CID0 = crate::Reg<u32, _CID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID0;
#[doc = "`read()` method returns [cid0::R](cid0::R) reader structure"]
impl crate::Readable for CID0 {}
#[doc = "Component Identification 0"]
pub mod cid0;
#[doc = "Component Identification 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid1](cid1) module"]
pub type CID1 = crate::Reg<u32, _CID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID1;
#[doc = "`read()` method returns [cid1::R](cid1::R) reader structure"]
impl crate::Readable for CID1 {}
#[doc = "Component Identification 1"]
pub mod cid1;
#[doc = "Component Identification 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid2](cid2) module"]
pub type CID2 = crate::Reg<u32, _CID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID2;
#[doc = "`read()` method returns [cid2::R](cid2::R) reader structure"]
impl crate::Readable for CID2 {}
#[doc = "Component Identification 2"]
pub mod cid2;
#[doc = "Component Identification 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid3](cid3) module"]
pub type CID3 = crate::Reg<u32, _CID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID3;
#[doc = "`read()` method returns [cid3::R](cid3::R) reader structure"]
impl crate::Readable for CID3 {}
#[doc = "Component Identification 3"]
pub mod cid3;
