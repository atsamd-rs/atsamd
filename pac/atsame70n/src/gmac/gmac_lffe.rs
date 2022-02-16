#[doc = "Register `GMAC_LFFE` reader"]
pub struct R(crate::R<GMAC_LFFE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_LFFE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_LFFE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_LFFE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LFER` reader - Length Field Frame Errors"]
pub struct LFER_R(crate::FieldReader<u16, u16>);
impl LFER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LFER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Length Field Frame Errors"]
    #[inline(always)]
    pub fn lfer(&self) -> LFER_R {
        LFER_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Length Field Frame Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_lffe](index.html) module"]
pub struct GMAC_LFFE_SPEC;
impl crate::RegisterSpec for GMAC_LFFE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_lffe::R](R) reader structure"]
impl crate::Readable for GMAC_LFFE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_LFFE to value 0"]
impl crate::Resettable for GMAC_LFFE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
