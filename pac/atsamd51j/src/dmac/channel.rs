#[doc = "Channel n Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrla](chctrla) module"]
pub type CHCTRLA = crate::Reg<u32, _CHCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTRLA;
#[doc = "`read()` method returns [chctrla::R](chctrla::R) reader structure"]
impl crate::Readable for CHCTRLA {}
#[doc = "`write(|w| ..)` method takes [chctrla::W](chctrla::W) writer structure"]
impl crate::Writable for CHCTRLA {}
#[doc = "Channel n Control A"]
pub mod chctrla;
#[doc = "Channel n Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrlb](chctrlb) module"]
pub type CHCTRLB = crate::Reg<u8, _CHCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTRLB;
#[doc = "`read()` method returns [chctrlb::R](chctrlb::R) reader structure"]
impl crate::Readable for CHCTRLB {}
#[doc = "`write(|w| ..)` method takes [chctrlb::W](chctrlb::W) writer structure"]
impl crate::Writable for CHCTRLB {}
#[doc = "Channel n Control B"]
pub mod chctrlb;
#[doc = "Channel n Priority Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chprilvl](chprilvl) module"]
pub type CHPRILVL = crate::Reg<u8, _CHPRILVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPRILVL;
#[doc = "`read()` method returns [chprilvl::R](chprilvl::R) reader structure"]
impl crate::Readable for CHPRILVL {}
#[doc = "`write(|w| ..)` method takes [chprilvl::W](chprilvl::W) writer structure"]
impl crate::Writable for CHPRILVL {}
#[doc = "Channel n Priority Level"]
pub mod chprilvl;
#[doc = "Channel n Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chevctrl](chevctrl) module"]
pub type CHEVCTRL = crate::Reg<u8, _CHEVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEVCTRL;
#[doc = "`read()` method returns [chevctrl::R](chevctrl::R) reader structure"]
impl crate::Readable for CHEVCTRL {}
#[doc = "`write(|w| ..)` method takes [chevctrl::W](chevctrl::W) writer structure"]
impl crate::Writable for CHEVCTRL {}
#[doc = "Channel n Event Control"]
pub mod chevctrl;
#[doc = "Channel n Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintenclr](chintenclr) module"]
pub type CHINTENCLR = crate::Reg<u8, _CHINTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHINTENCLR;
#[doc = "`read()` method returns [chintenclr::R](chintenclr::R) reader structure"]
impl crate::Readable for CHINTENCLR {}
#[doc = "`write(|w| ..)` method takes [chintenclr::W](chintenclr::W) writer structure"]
impl crate::Writable for CHINTENCLR {}
#[doc = "Channel n Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "Channel n Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintenset](chintenset) module"]
pub type CHINTENSET = crate::Reg<u8, _CHINTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHINTENSET;
#[doc = "`read()` method returns [chintenset::R](chintenset::R) reader structure"]
impl crate::Readable for CHINTENSET {}
#[doc = "`write(|w| ..)` method takes [chintenset::W](chintenset::W) writer structure"]
impl crate::Writable for CHINTENSET {}
#[doc = "Channel n Interrupt Enable Set"]
pub mod chintenset;
#[doc = "Channel n Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintflag](chintflag) module"]
pub type CHINTFLAG = crate::Reg<u8, _CHINTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHINTFLAG;
#[doc = "`read()` method returns [chintflag::R](chintflag::R) reader structure"]
impl crate::Readable for CHINTFLAG {}
#[doc = "`write(|w| ..)` method takes [chintflag::W](chintflag::W) writer structure"]
impl crate::Writable for CHINTFLAG {}
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "Channel n Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chstatus](chstatus) module"]
pub type CHSTATUS = crate::Reg<u8, _CHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSTATUS;
#[doc = "`read()` method returns [chstatus::R](chstatus::R) reader structure"]
impl crate::Readable for CHSTATUS {}
#[doc = "`write(|w| ..)` method takes [chstatus::W](chstatus::W) writer structure"]
impl crate::Writable for CHSTATUS {}
#[doc = "Channel n Status"]
pub mod chstatus;
