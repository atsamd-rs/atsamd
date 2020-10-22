#[doc = "DEVICE_ENDPOINT End Point Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcfg](epcfg) module"]
pub type EPCFG = crate::Reg<u8, _EPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPCFG;
#[doc = "`read()` method returns [epcfg::R](epcfg::R) reader structure"]
impl crate::Readable for EPCFG {}
#[doc = "`write(|w| ..)` method takes [epcfg::W](epcfg::W) writer structure"]
impl crate::Writable for EPCFG {}
#[doc = "DEVICE_ENDPOINT End Point Configuration"]
pub mod epcfg;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatusclr](epstatusclr) module"]
pub type EPSTATUSCLR = crate::Reg<u8, _EPSTATUSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSTATUSCLR;
#[doc = "`write(|w| ..)` method takes [epstatusclr::W](epstatusclr::W) writer structure"]
impl crate::Writable for EPSTATUSCLR {}
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Clear"]
pub mod epstatusclr;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatusset](epstatusset) module"]
pub type EPSTATUSSET = crate::Reg<u8, _EPSTATUSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSTATUSSET;
#[doc = "`write(|w| ..)` method takes [epstatusset::W](epstatusset::W) writer structure"]
impl crate::Writable for EPSTATUSSET {}
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Set"]
pub mod epstatusset;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatus](epstatus) module"]
pub type EPSTATUS = crate::Reg<u8, _EPSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSTATUS;
#[doc = "`read()` method returns [epstatus::R](epstatus::R) reader structure"]
impl crate::Readable for EPSTATUS {}
#[doc = "DEVICE_ENDPOINT End Point Pipe Status"]
pub mod epstatus;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintflag](epintflag) module"]
pub type EPINTFLAG = crate::Reg<u8, _EPINTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTFLAG;
#[doc = "`read()` method returns [epintflag::R](epintflag::R) reader structure"]
impl crate::Readable for EPINTFLAG {}
#[doc = "`write(|w| ..)` method takes [epintflag::W](epintflag::W) writer structure"]
impl crate::Writable for EPINTFLAG {}
#[doc = "DEVICE_ENDPOINT End Point Interrupt Flag"]
pub mod epintflag;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Clear Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintenclr](epintenclr) module"]
pub type EPINTENCLR = crate::Reg<u8, _EPINTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTENCLR;
#[doc = "`read()` method returns [epintenclr::R](epintenclr::R) reader structure"]
impl crate::Readable for EPINTENCLR {}
#[doc = "`write(|w| ..)` method takes [epintenclr::W](epintenclr::W) writer structure"]
impl crate::Writable for EPINTENCLR {}
#[doc = "DEVICE_ENDPOINT End Point Interrupt Clear Flag"]
pub mod epintenclr;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Set Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintenset](epintenset) module"]
pub type EPINTENSET = crate::Reg<u8, _EPINTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTENSET;
#[doc = "`read()` method returns [epintenset::R](epintenset::R) reader structure"]
impl crate::Readable for EPINTENSET {}
#[doc = "`write(|w| ..)` method takes [epintenset::W](epintenset::W) writer structure"]
impl crate::Writable for EPINTENSET {}
#[doc = "DEVICE_ENDPOINT End Point Interrupt Set Flag"]
pub mod epintenset;
