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
#[doc = "HOST Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u16, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "HOST Control B"]
pub mod ctrlb;
#[doc = "HOST Host Start Of Frame Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsofc](hsofc) module"]
pub type HSOFC = crate::Reg<u8, _HSOFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSOFC;
#[doc = "`read()` method returns [hsofc::R](hsofc::R) reader structure"]
impl crate::Readable for HSOFC {}
#[doc = "`write(|w| ..)` method takes [hsofc::W](hsofc::W) writer structure"]
impl crate::Writable for HSOFC {}
#[doc = "HOST Host Start Of Frame Control"]
pub mod hsofc;
#[doc = "HOST Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u8, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "HOST Status"]
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
#[doc = "HOST Host Frame Number\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnum](fnum) module"]
pub type FNUM = crate::Reg<u16, _FNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FNUM;
#[doc = "`read()` method returns [fnum::R](fnum::R) reader structure"]
impl crate::Readable for FNUM {}
#[doc = "`write(|w| ..)` method takes [fnum::W](fnum::W) writer structure"]
impl crate::Writable for FNUM {}
#[doc = "HOST Host Frame Number"]
pub mod fnum;
#[doc = "HOST Host Frame Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flenhigh](flenhigh) module"]
pub type FLENHIGH = crate::Reg<u8, _FLENHIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLENHIGH;
#[doc = "`read()` method returns [flenhigh::R](flenhigh::R) reader structure"]
impl crate::Readable for FLENHIGH {}
#[doc = "HOST Host Frame Length"]
pub mod flenhigh;
#[doc = "HOST Host Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u16, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "HOST Host Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "HOST Host Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u16, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "HOST Host Interrupt Enable Set"]
pub mod intenset;
#[doc = "HOST Host Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u16, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "HOST Host Interrupt Flag"]
pub mod intflag;
#[doc = "HOST Pipe Interrupt Summary\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsmry](pintsmry) module"]
pub type PINTSMRY = crate::Reg<u16, _PINTSMRY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTSMRY;
#[doc = "`read()` method returns [pintsmry::R](pintsmry::R) reader structure"]
impl crate::Readable for PINTSMRY {}
#[doc = "HOST Pipe Interrupt Summary"]
pub mod pintsmry;
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
#[doc = "HOST End Point Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg](pcfg) module"]
pub type PCFG = crate::Reg<u8, _PCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCFG;
#[doc = "`read()` method returns [pcfg::R](pcfg::R) reader structure"]
impl crate::Readable for PCFG {}
#[doc = "`write(|w| ..)` method takes [pcfg::W](pcfg::W) writer structure"]
impl crate::Writable for PCFG {}
#[doc = "HOST End Point Configuration"]
pub mod pcfg;
#[doc = "HOST Bus Access Period of Pipe\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [binterval](binterval) module"]
pub type BINTERVAL = crate::Reg<u8, _BINTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BINTERVAL;
#[doc = "`read()` method returns [binterval::R](binterval::R) reader structure"]
impl crate::Readable for BINTERVAL {}
#[doc = "`write(|w| ..)` method takes [binterval::W](binterval::W) writer structure"]
impl crate::Writable for BINTERVAL {}
#[doc = "HOST Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "HOST End Point Pipe Status Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstatusclr](pstatusclr) module"]
pub type PSTATUSCLR = crate::Reg<u8, _PSTATUSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSTATUSCLR;
#[doc = "`write(|w| ..)` method takes [pstatusclr::W](pstatusclr::W) writer structure"]
impl crate::Writable for PSTATUSCLR {}
#[doc = "HOST End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "HOST End Point Pipe Status Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstatusset](pstatusset) module"]
pub type PSTATUSSET = crate::Reg<u8, _PSTATUSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSTATUSSET;
#[doc = "`write(|w| ..)` method takes [pstatusset::W](pstatusset::W) writer structure"]
impl crate::Writable for PSTATUSSET {}
#[doc = "HOST End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "HOST End Point Pipe Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstatus](pstatus) module"]
pub type PSTATUS = crate::Reg<u8, _PSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSTATUS;
#[doc = "`read()` method returns [pstatus::R](pstatus::R) reader structure"]
impl crate::Readable for PSTATUS {}
#[doc = "HOST End Point Pipe Status"]
pub mod pstatus;
#[doc = "HOST Pipe Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintflag](pintflag) module"]
pub type PINTFLAG = crate::Reg<u8, _PINTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTFLAG;
#[doc = "`read()` method returns [pintflag::R](pintflag::R) reader structure"]
impl crate::Readable for PINTFLAG {}
#[doc = "`write(|w| ..)` method takes [pintflag::W](pintflag::W) writer structure"]
impl crate::Writable for PINTFLAG {}
#[doc = "HOST Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "HOST Pipe Interrupt Flag Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintenclr](pintenclr) module"]
pub type PINTENCLR = crate::Reg<u8, _PINTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTENCLR;
#[doc = "`read()` method returns [pintenclr::R](pintenclr::R) reader structure"]
impl crate::Readable for PINTENCLR {}
#[doc = "`write(|w| ..)` method takes [pintenclr::W](pintenclr::W) writer structure"]
impl crate::Writable for PINTENCLR {}
#[doc = "HOST Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "HOST Pipe Interrupt Flag Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintenset](pintenset) module"]
pub type PINTENSET = crate::Reg<u8, _PINTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTENSET;
#[doc = "`read()` method returns [pintenset::R](pintenset::R) reader structure"]
impl crate::Readable for PINTENSET {}
#[doc = "`write(|w| ..)` method takes [pintenset::W](pintenset::W) writer structure"]
impl crate::Writable for PINTENSET {}
#[doc = "HOST Pipe Interrupt Flag Set"]
pub mod pintenset;
