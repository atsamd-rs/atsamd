#[doc = "Priority A for Slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras](pras) module"]
pub type PRAS = crate::Reg<u32, _PRAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS;
#[doc = "`read()` method returns [pras::R](pras::R) reader structure"]
impl crate::Readable for PRAS {}
#[doc = "`write(|w| ..)` method takes [pras::W](pras::W) writer structure"]
impl crate::Writable for PRAS {}
#[doc = "Priority A for Slave"]
pub mod pras;
#[doc = "Priority B for Slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs](prbs) module"]
pub type PRBS = crate::Reg<u32, _PRBS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS;
#[doc = "`read()` method returns [prbs::R](prbs::R) reader structure"]
impl crate::Readable for PRBS {}
#[doc = "`write(|w| ..)` method takes [prbs::W](prbs::W) writer structure"]
impl crate::Writable for PRBS {}
#[doc = "Priority B for Slave"]
pub mod prbs;
