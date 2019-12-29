#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u8, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "USB Quality Of Service\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qosctrl](qosctrl) module"]
pub type QOSCTRL = crate::Reg<u8, _QOSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QOSCTRL;
#[doc = "`read()` method returns [qosctrl::R](qosctrl::R) reader structure"]
impl crate::Readable for QOSCTRL {}
#[doc = "`write(|w| ..)` method takes [qosctrl::W](qosctrl::W) writer structure"]
impl crate::Writable for QOSCTRL {}
#[doc = "USB Quality Of Service"]
pub mod qosctrl;
#[doc = "DEVICE Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u16, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "DEVICE Control B"]
pub mod ctrlb;
#[doc = "DEVICE Device Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dadd](dadd) module"]
pub type DADD = crate::Reg<u8, _DADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADD;
#[doc = "`read()` method returns [dadd::R](dadd::R) reader structure"]
impl crate::Readable for DADD {}
#[doc = "`write(|w| ..)` method takes [dadd::W](dadd::W) writer structure"]
impl crate::Writable for DADD {}
#[doc = "DEVICE Device Address"]
pub mod dadd;
#[doc = "DEVICE Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u8, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "DEVICE Status"]
pub mod status;
#[doc = "Finite State Machine Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsmstatus](fsmstatus) module"]
pub type FSMSTATUS = crate::Reg<u8, _FSMSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSMSTATUS;
#[doc = "`read()` method returns [fsmstatus::R](fsmstatus::R) reader structure"]
impl crate::Readable for FSMSTATUS {}
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "DEVICE Device Frame Number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnum](fnum) module"]
pub type FNUM = crate::Reg<u16, _FNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FNUM;
#[doc = "`read()` method returns [fnum::R](fnum::R) reader structure"]
impl crate::Readable for FNUM {}
#[doc = "DEVICE Device Frame Number"]
pub mod fnum;
#[doc = "DEVICE Device Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u16, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "DEVICE Device Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "DEVICE Device Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u16, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "DEVICE Device Interrupt Enable Set"]
pub mod intenset;
#[doc = "DEVICE Device Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u16, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "DEVICE Device Interrupt Flag"]
pub mod intflag;
#[doc = "DEVICE End Point Interrupt Summary\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintsmry](epintsmry) module"]
pub type EPINTSMRY = crate::Reg<u16, _EPINTSMRY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTSMRY;
#[doc = "`read()` method returns [epintsmry::R](epintsmry::R) reader structure"]
impl crate::Readable for EPINTSMRY {}
#[doc = "DEVICE End Point Interrupt Summary"]
pub mod epintsmry;
#[doc = "Descriptor Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descadd](descadd) module"]
pub type DESCADD = crate::Reg<u32, _DESCADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCADD;
#[doc = "`read()` method returns [descadd::R](descadd::R) reader structure"]
impl crate::Readable for DESCADD {}
#[doc = "`write(|w| ..)` method takes [descadd::W](descadd::W) writer structure"]
impl crate::Writable for DESCADD {}
#[doc = "Descriptor Address"]
pub mod descadd;
#[doc = "USB PAD Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcal](padcal) module"]
pub type PADCAL = crate::Reg<u16, _PADCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADCAL;
#[doc = "`read()` method returns [padcal::R](padcal::R) reader structure"]
impl crate::Readable for PADCAL {}
#[doc = "`write(|w| ..)` method takes [padcal::W](padcal::W) writer structure"]
impl crate::Writable for PADCAL {}
#[doc = "USB PAD Calibration"]
pub mod padcal;
#[doc = "DEVICE End Point Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcfg](epcfg) module"]
pub type EPCFG = crate::Reg<u8, _EPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPCFG;
#[doc = "`read()` method returns [epcfg::R](epcfg::R) reader structure"]
impl crate::Readable for EPCFG {}
#[doc = "`write(|w| ..)` method takes [epcfg::W](epcfg::W) writer structure"]
impl crate::Writable for EPCFG {}
#[doc = "DEVICE End Point Configuration"]
pub mod epcfg;
#[doc = "DEVICE End Point Pipe Status Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatusclr](epstatusclr) module"]
pub type EPSTATUSCLR = crate::Reg<u8, _EPSTATUSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSTATUSCLR;
#[doc = "`write(|w| ..)` method takes [epstatusclr::W](epstatusclr::W) writer structure"]
impl crate::Writable for EPSTATUSCLR {}
#[doc = "DEVICE End Point Pipe Status Clear"]
pub mod epstatusclr;
#[doc = "DEVICE End Point Pipe Status Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatusset](epstatusset) module"]
pub type EPSTATUSSET = crate::Reg<u8, _EPSTATUSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSTATUSSET;
#[doc = "`write(|w| ..)` method takes [epstatusset::W](epstatusset::W) writer structure"]
impl crate::Writable for EPSTATUSSET {}
#[doc = "DEVICE End Point Pipe Status Set"]
pub mod epstatusset;
#[doc = "DEVICE End Point Pipe Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatus](epstatus) module"]
pub type EPSTATUS = crate::Reg<u8, _EPSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSTATUS;
#[doc = "`read()` method returns [epstatus::R](epstatus::R) reader structure"]
impl crate::Readable for EPSTATUS {}
#[doc = "DEVICE End Point Pipe Status"]
pub mod epstatus;
#[doc = "DEVICE End Point Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintflag](epintflag) module"]
pub type EPINTFLAG = crate::Reg<u8, _EPINTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTFLAG;
#[doc = "`read()` method returns [epintflag::R](epintflag::R) reader structure"]
impl crate::Readable for EPINTFLAG {}
#[doc = "`write(|w| ..)` method takes [epintflag::W](epintflag::W) writer structure"]
impl crate::Writable for EPINTFLAG {}
#[doc = "DEVICE End Point Interrupt Flag"]
pub mod epintflag;
#[doc = "DEVICE End Point Interrupt Clear Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintenclr](epintenclr) module"]
pub type EPINTENCLR = crate::Reg<u8, _EPINTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTENCLR;
#[doc = "`read()` method returns [epintenclr::R](epintenclr::R) reader structure"]
impl crate::Readable for EPINTENCLR {}
#[doc = "`write(|w| ..)` method takes [epintenclr::W](epintenclr::W) writer structure"]
impl crate::Writable for EPINTENCLR {}
#[doc = "DEVICE End Point Interrupt Clear Flag"]
pub mod epintenclr;
#[doc = "DEVICE End Point Interrupt Set Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintenset](epintenset) module"]
pub type EPINTENSET = crate::Reg<u8, _EPINTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTENSET;
#[doc = "`read()` method returns [epintenset::R](epintenset::R) reader structure"]
impl crate::Readable for EPINTENSET {}
#[doc = "`write(|w| ..)` method takes [epintenset::W](epintenset::W) writer structure"]
impl crate::Writable for EPINTENSET {}
#[doc = "DEVICE End Point Interrupt Set Flag"]
pub mod epintenset;
