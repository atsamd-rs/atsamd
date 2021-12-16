#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write Protection Clear"]
    pub wpclr: WPCLR,
    #[doc = "0x04 - Write Protection Set"]
    pub wpset: WPSET,
}
#[doc = "Write Protection Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpclr](wpclr) module"]
pub type WPCLR = crate::Reg<u32, _WPCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPCLR;
#[doc = "`read()` method returns [wpclr::R](wpclr::R) reader structure"]
impl crate::Readable for WPCLR {}
#[doc = "`write(|w| ..)` method takes [wpclr::W](wpclr::W) writer structure"]
impl crate::Writable for WPCLR {}
#[doc = "Write Protection Clear"]
pub mod wpclr;
#[doc = "Write Protection Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpset](wpset) module"]
pub type WPSET = crate::Reg<u32, _WPSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSET;
#[doc = "`read()` method returns [wpset::R](wpset::R) reader structure"]
impl crate::Readable for WPSET {}
#[doc = "`write(|w| ..)` method takes [wpset::W](wpset::W) writer structure"]
impl crate::Writable for WPSET {}
#[doc = "Write Protection Set"]
pub mod wpset;
