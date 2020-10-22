#[doc = "HOST_PIPE End Point Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg](pcfg) module"]
pub type PCFG = crate::Reg<u8, _PCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCFG;
#[doc = "`read()` method returns [pcfg::R](pcfg::R) reader structure"]
impl crate::Readable for PCFG {}
#[doc = "`write(|w| ..)` method takes [pcfg::W](pcfg::W) writer structure"]
impl crate::Writable for PCFG {}
#[doc = "HOST_PIPE End Point Configuration"]
pub mod pcfg;
#[doc = "HOST_PIPE Bus Access Period of Pipe\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [binterval](binterval) module"]
pub type BINTERVAL = crate::Reg<u8, _BINTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BINTERVAL;
#[doc = "`read()` method returns [binterval::R](binterval::R) reader structure"]
impl crate::Readable for BINTERVAL {}
#[doc = "`write(|w| ..)` method takes [binterval::W](binterval::W) writer structure"]
impl crate::Writable for BINTERVAL {}
#[doc = "HOST_PIPE Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "HOST_PIPE End Point Pipe Status Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstatusclr](pstatusclr) module"]
pub type PSTATUSCLR = crate::Reg<u8, _PSTATUSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSTATUSCLR;
#[doc = "`write(|w| ..)` method takes [pstatusclr::W](pstatusclr::W) writer structure"]
impl crate::Writable for PSTATUSCLR {}
#[doc = "HOST_PIPE End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "HOST_PIPE End Point Pipe Status Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstatusset](pstatusset) module"]
pub type PSTATUSSET = crate::Reg<u8, _PSTATUSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSTATUSSET;
#[doc = "`write(|w| ..)` method takes [pstatusset::W](pstatusset::W) writer structure"]
impl crate::Writable for PSTATUSSET {}
#[doc = "HOST_PIPE End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "HOST_PIPE End Point Pipe Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstatus](pstatus) module"]
pub type PSTATUS = crate::Reg<u8, _PSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSTATUS;
#[doc = "`read()` method returns [pstatus::R](pstatus::R) reader structure"]
impl crate::Readable for PSTATUS {}
#[doc = "HOST_PIPE End Point Pipe Status"]
pub mod pstatus;
#[doc = "HOST_PIPE Pipe Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintflag](pintflag) module"]
pub type PINTFLAG = crate::Reg<u8, _PINTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTFLAG;
#[doc = "`read()` method returns [pintflag::R](pintflag::R) reader structure"]
impl crate::Readable for PINTFLAG {}
#[doc = "`write(|w| ..)` method takes [pintflag::W](pintflag::W) writer structure"]
impl crate::Writable for PINTFLAG {}
#[doc = "HOST_PIPE Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "HOST_PIPE Pipe Interrupt Flag Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintenclr](pintenclr) module"]
pub type PINTENCLR = crate::Reg<u8, _PINTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTENCLR;
#[doc = "`read()` method returns [pintenclr::R](pintenclr::R) reader structure"]
impl crate::Readable for PINTENCLR {}
#[doc = "`write(|w| ..)` method takes [pintenclr::W](pintenclr::W) writer structure"]
impl crate::Writable for PINTENCLR {}
#[doc = "HOST_PIPE Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "HOST_PIPE Pipe Interrupt Flag Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintenset](pintenset) module"]
pub type PINTENSET = crate::Reg<u8, _PINTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTENSET;
#[doc = "`read()` method returns [pintenset::R](pintenset::R) reader structure"]
impl crate::Readable for PINTENSET {}
#[doc = "`write(|w| ..)` method takes [pintenset::W](pintenset::W) writer structure"]
impl crate::Writable for PINTENSET {}
#[doc = "HOST_PIPE Pipe Interrupt Flag Set"]
pub mod pintenset;
