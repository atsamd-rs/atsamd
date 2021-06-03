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
    _reserved4: [u8; 3824usize],
    #[doc = "0xf00 - MTB Integration Mode Control"]
    pub itctrl: ITCTRL,
    _reserved5: [u8; 156usize],
    #[doc = "0xfa0 - MTB Claim Set"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - MTB Claim Clear"]
    pub claimclr: CLAIMCLR,
    _reserved7: [u8; 8usize],
    #[doc = "0xfb0 - MTB Lock Access"]
    pub lockaccess: LOCKACCESS,
    #[doc = "0xfb4 - MTB Lock Status"]
    pub lockstatus: LOCKSTATUS,
    #[doc = "0xfb8 - MTB Authentication Status"]
    pub authstatus: AUTHSTATUS,
    #[doc = "0xfbc - MTB Device Architecture"]
    pub devarch: DEVARCH,
    _reserved11: [u8; 8usize],
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
#[doc = "MTB Position\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [position](position) module"]
pub type POSITION = crate::Reg<u32, _POSITION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POSITION;
#[doc = "`read()` method returns [position::R](position::R) reader structure"]
impl crate::Readable for POSITION {}
#[doc = "`write(|w| ..)` method takes [position::W](position::W) writer structure"]
impl crate::Writable for POSITION {}
#[doc = "MTB Position"]
pub mod position;
#[doc = "MTB Master\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master](master) module"]
pub type MASTER = crate::Reg<u32, _MASTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASTER;
#[doc = "`read()` method returns [master::R](master::R) reader structure"]
impl crate::Readable for MASTER {}
#[doc = "`write(|w| ..)` method takes [master::W](master::W) writer structure"]
impl crate::Writable for MASTER {}
#[doc = "MTB Master"]
pub mod master;
#[doc = "MTB Flow\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow](flow) module"]
pub type FLOW = crate::Reg<u32, _FLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOW;
#[doc = "`read()` method returns [flow::R](flow::R) reader structure"]
impl crate::Readable for FLOW {}
#[doc = "`write(|w| ..)` method takes [flow::W](flow::W) writer structure"]
impl crate::Writable for FLOW {}
#[doc = "MTB Flow"]
pub mod flow;
#[doc = "MTB Base\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base](base) module"]
pub type BASE = crate::Reg<u32, _BASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE;
#[doc = "`read()` method returns [base::R](base::R) reader structure"]
impl crate::Readable for BASE {}
#[doc = "MTB Base"]
pub mod base;
#[doc = "MTB Integration Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itctrl](itctrl) module"]
pub type ITCTRL = crate::Reg<u32, _ITCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITCTRL;
#[doc = "`read()` method returns [itctrl::R](itctrl::R) reader structure"]
impl crate::Readable for ITCTRL {}
#[doc = "`write(|w| ..)` method takes [itctrl::W](itctrl::W) writer structure"]
impl crate::Writable for ITCTRL {}
#[doc = "MTB Integration Mode Control"]
pub mod itctrl;
#[doc = "MTB Claim Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimset](claimset) module"]
pub type CLAIMSET = crate::Reg<u32, _CLAIMSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMSET;
#[doc = "`read()` method returns [claimset::R](claimset::R) reader structure"]
impl crate::Readable for CLAIMSET {}
#[doc = "`write(|w| ..)` method takes [claimset::W](claimset::W) writer structure"]
impl crate::Writable for CLAIMSET {}
#[doc = "MTB Claim Set"]
pub mod claimset;
#[doc = "MTB Claim Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimclr](claimclr) module"]
pub type CLAIMCLR = crate::Reg<u32, _CLAIMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMCLR;
#[doc = "`read()` method returns [claimclr::R](claimclr::R) reader structure"]
impl crate::Readable for CLAIMCLR {}
#[doc = "`write(|w| ..)` method takes [claimclr::W](claimclr::W) writer structure"]
impl crate::Writable for CLAIMCLR {}
#[doc = "MTB Claim Clear"]
pub mod claimclr;
#[doc = "MTB Lock Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockaccess](lockaccess) module"]
pub type LOCKACCESS = crate::Reg<u32, _LOCKACCESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKACCESS;
#[doc = "`read()` method returns [lockaccess::R](lockaccess::R) reader structure"]
impl crate::Readable for LOCKACCESS {}
#[doc = "`write(|w| ..)` method takes [lockaccess::W](lockaccess::W) writer structure"]
impl crate::Writable for LOCKACCESS {}
#[doc = "MTB Lock Access"]
pub mod lockaccess;
#[doc = "MTB Lock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstatus](lockstatus) module"]
pub type LOCKSTATUS = crate::Reg<u32, _LOCKSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKSTATUS;
#[doc = "`read()` method returns [lockstatus::R](lockstatus::R) reader structure"]
impl crate::Readable for LOCKSTATUS {}
#[doc = "MTB Lock Status"]
pub mod lockstatus;
#[doc = "MTB Authentication Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [authstatus](authstatus) module"]
pub type AUTHSTATUS = crate::Reg<u32, _AUTHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTHSTATUS;
#[doc = "`read()` method returns [authstatus::R](authstatus::R) reader structure"]
impl crate::Readable for AUTHSTATUS {}
#[doc = "MTB Authentication Status"]
pub mod authstatus;
#[doc = "MTB Device Architecture\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devarch](devarch) module"]
pub type DEVARCH = crate::Reg<u32, _DEVARCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVARCH;
#[doc = "`read()` method returns [devarch::R](devarch::R) reader structure"]
impl crate::Readable for DEVARCH {}
#[doc = "MTB Device Architecture"]
pub mod devarch;
#[doc = "MTB Device Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devid](devid) module"]
pub type DEVID = crate::Reg<u32, _DEVID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVID;
#[doc = "`read()` method returns [devid::R](devid::R) reader structure"]
impl crate::Readable for DEVID {}
#[doc = "MTB Device Configuration"]
pub mod devid;
#[doc = "MTB Device Type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devtype](devtype) module"]
pub type DEVTYPE = crate::Reg<u32, _DEVTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVTYPE;
#[doc = "`read()` method returns [devtype::R](devtype::R) reader structure"]
impl crate::Readable for DEVTYPE {}
#[doc = "MTB Device Type"]
pub mod devtype;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid4](pid4) module"]
pub type PID4 = crate::Reg<u32, _PID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID4;
#[doc = "`read()` method returns [pid4::R](pid4::R) reader structure"]
impl crate::Readable for PID4 {}
#[doc = "CoreSight"]
pub mod pid4;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid5](pid5) module"]
pub type PID5 = crate::Reg<u32, _PID5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID5;
#[doc = "`read()` method returns [pid5::R](pid5::R) reader structure"]
impl crate::Readable for PID5 {}
#[doc = "CoreSight"]
pub mod pid5;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid6](pid6) module"]
pub type PID6 = crate::Reg<u32, _PID6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID6;
#[doc = "`read()` method returns [pid6::R](pid6::R) reader structure"]
impl crate::Readable for PID6 {}
#[doc = "CoreSight"]
pub mod pid6;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid7](pid7) module"]
pub type PID7 = crate::Reg<u32, _PID7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID7;
#[doc = "`read()` method returns [pid7::R](pid7::R) reader structure"]
impl crate::Readable for PID7 {}
#[doc = "CoreSight"]
pub mod pid7;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid0](pid0) module"]
pub type PID0 = crate::Reg<u32, _PID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID0;
#[doc = "`read()` method returns [pid0::R](pid0::R) reader structure"]
impl crate::Readable for PID0 {}
#[doc = "CoreSight"]
pub mod pid0;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid1](pid1) module"]
pub type PID1 = crate::Reg<u32, _PID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID1;
#[doc = "`read()` method returns [pid1::R](pid1::R) reader structure"]
impl crate::Readable for PID1 {}
#[doc = "CoreSight"]
pub mod pid1;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid2](pid2) module"]
pub type PID2 = crate::Reg<u32, _PID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID2;
#[doc = "`read()` method returns [pid2::R](pid2::R) reader structure"]
impl crate::Readable for PID2 {}
#[doc = "CoreSight"]
pub mod pid2;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid3](pid3) module"]
pub type PID3 = crate::Reg<u32, _PID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID3;
#[doc = "`read()` method returns [pid3::R](pid3::R) reader structure"]
impl crate::Readable for PID3 {}
#[doc = "CoreSight"]
pub mod pid3;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid0](cid0) module"]
pub type CID0 = crate::Reg<u32, _CID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID0;
#[doc = "`read()` method returns [cid0::R](cid0::R) reader structure"]
impl crate::Readable for CID0 {}
#[doc = "CoreSight"]
pub mod cid0;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid1](cid1) module"]
pub type CID1 = crate::Reg<u32, _CID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID1;
#[doc = "`read()` method returns [cid1::R](cid1::R) reader structure"]
impl crate::Readable for CID1 {}
#[doc = "CoreSight"]
pub mod cid1;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid2](cid2) module"]
pub type CID2 = crate::Reg<u32, _CID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID2;
#[doc = "`read()` method returns [cid2::R](cid2::R) reader structure"]
impl crate::Readable for CID2 {}
#[doc = "CoreSight"]
pub mod cid2;
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid3](cid3) module"]
pub type CID3 = crate::Reg<u32, _CID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID3;
#[doc = "`read()` method returns [cid3::R](cid3::R) reader structure"]
impl crate::Readable for CID3 {}
#[doc = "CoreSight"]
pub mod cid3;
