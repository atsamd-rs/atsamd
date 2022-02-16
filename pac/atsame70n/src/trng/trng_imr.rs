#[doc = "Register `TRNG_IMR` reader"]
pub struct R(crate::R<TRNG_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATRDY` reader - Data Ready Interrupt Mask"]
pub struct DATRDY_R(crate::FieldReader<bool, bool>);
impl DATRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng_imr](index.html) module"]
pub struct TRNG_IMR_SPEC;
impl crate::RegisterSpec for TRNG_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng_imr::R](R) reader structure"]
impl crate::Readable for TRNG_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG_IMR to value 0"]
impl crate::Resettable for TRNG_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
