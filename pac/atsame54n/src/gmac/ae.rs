#[doc = "Register `AE` reader"]
pub struct R(crate::R<AE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AER` reader - Alignment Errors"]
pub struct AER_R(crate::FieldReader<u16, u16>);
impl AER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        AER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Alignment Errors"]
    #[inline(always)]
    pub fn aer(&self) -> AER_R {
        AER_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Alignment Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ae](index.html) module"]
pub struct AE_SPEC;
impl crate::RegisterSpec for AE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ae::R](R) reader structure"]
impl crate::Readable for AE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AE to value 0"]
impl crate::Resettable for AE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
