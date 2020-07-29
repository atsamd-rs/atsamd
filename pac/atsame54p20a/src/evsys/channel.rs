#[doc = "Channel n Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel](channel) module"]
pub type CHANNEL = crate::Reg<u32, _CHANNEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNEL;
#[doc = "`read()` method returns [channel::R](channel::R) reader structure"]
impl crate::Readable for CHANNEL {}
#[doc = "`write(|w| ..)` method takes [channel::W](channel::W) writer structure"]
impl crate::Writable for CHANNEL {}
#[doc = "Channel n Control"]
pub mod channel;
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
#[doc = "Channel n Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chstatus](chstatus) module"]
pub type CHSTATUS = crate::Reg<u8, _CHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSTATUS;
#[doc = "`read()` method returns [chstatus::R](chstatus::R) reader structure"]
impl crate::Readable for CHSTATUS {}
#[doc = "Channel n Status"]
pub mod chstatus;
