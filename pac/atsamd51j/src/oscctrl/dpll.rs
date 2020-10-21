#[doc = "DPLL Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrla](dpllctrla) module"]
pub type DPLLCTRLA = crate::Reg<u8, _DPLLCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRLA;
#[doc = "`read()` method returns [dpllctrla::R](dpllctrla::R) reader structure"]
impl crate::Readable for DPLLCTRLA {}
#[doc = "`write(|w| ..)` method takes [dpllctrla::W](dpllctrla::W) writer structure"]
impl crate::Writable for DPLLCTRLA {}
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLL Ratio Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllratio](dpllratio) module"]
pub type DPLLRATIO = crate::Reg<u32, _DPLLRATIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLRATIO;
#[doc = "`read()` method returns [dpllratio::R](dpllratio::R) reader structure"]
impl crate::Readable for DPLLRATIO {}
#[doc = "`write(|w| ..)` method takes [dpllratio::W](dpllratio::W) writer structure"]
impl crate::Writable for DPLLRATIO {}
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLL Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrlb](dpllctrlb) module"]
pub type DPLLCTRLB = crate::Reg<u32, _DPLLCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRLB;
#[doc = "`read()` method returns [dpllctrlb::R](dpllctrlb::R) reader structure"]
impl crate::Readable for DPLLCTRLB {}
#[doc = "`write(|w| ..)` method takes [dpllctrlb::W](dpllctrlb::W) writer structure"]
impl crate::Writable for DPLLCTRLB {}
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLL Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllsyncbusy](dpllsyncbusy) module"]
pub type DPLLSYNCBUSY = crate::Reg<u32, _DPLLSYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLSYNCBUSY;
#[doc = "`read()` method returns [dpllsyncbusy::R](dpllsyncbusy::R) reader structure"]
impl crate::Readable for DPLLSYNCBUSY {}
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllstatus](dpllstatus) module"]
pub type DPLLSTATUS = crate::Reg<u32, _DPLLSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLSTATUS;
#[doc = "`read()` method returns [dpllstatus::R](dpllstatus::R) reader structure"]
impl crate::Readable for DPLLSTATUS {}
#[doc = "DPLL Status"]
pub mod dpllstatus;
