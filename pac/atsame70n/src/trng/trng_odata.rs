#[doc = "Register `TRNG_ODATA` reader"]
pub struct R(crate::R<TRNG_ODATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG_ODATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG_ODATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG_ODATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ODATA` reader - Output Data"]
pub struct ODATA_R(crate::FieldReader<u32, u32>);
impl ODATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ODATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Output Data"]
    #[inline(always)]
    pub fn odata(&self) -> ODATA_R {
        ODATA_R::new(self.bits as u32)
    }
}
#[doc = "Output Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng_odata](index.html) module"]
pub struct TRNG_ODATA_SPEC;
impl crate::RegisterSpec for TRNG_ODATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng_odata::R](R) reader structure"]
impl crate::Readable for TRNG_ODATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG_ODATA to value 0"]
impl crate::Resettable for TRNG_ODATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
