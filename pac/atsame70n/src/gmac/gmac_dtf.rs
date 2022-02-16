#[doc = "Register `GMAC_DTF` reader"]
pub struct R(crate::R<GMAC_DTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_DTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_DTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_DTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEFT` reader - Deferred Transmission"]
pub struct DEFT_R(crate::FieldReader<u32, u32>);
impl DEFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DEFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEFT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - Deferred Transmission"]
    #[inline(always)]
    pub fn deft(&self) -> DEFT_R {
        DEFT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "Deferred Transmission Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_dtf](index.html) module"]
pub struct GMAC_DTF_SPEC;
impl crate::RegisterSpec for GMAC_DTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_dtf::R](R) reader structure"]
impl crate::Readable for GMAC_DTF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_DTF to value 0"]
impl crate::Resettable for GMAC_DTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
