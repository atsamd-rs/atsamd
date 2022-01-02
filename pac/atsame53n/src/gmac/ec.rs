#[doc = "Register `EC` reader"]
pub struct R(crate::R<EC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `XCOL` reader - Excessive Collisions"]
pub struct XCOL_R(crate::FieldReader<u16, u16>);
impl XCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        XCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XCOL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Excessive Collisions"]
    #[inline(always)]
    pub fn xcol(&self) -> XCOL_R {
        XCOL_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Excessive Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec](index.html) module"]
pub struct EC_SPEC;
impl crate::RegisterSpec for EC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec::R](R) reader structure"]
impl crate::Readable for EC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EC to value 0"]
impl crate::Resettable for EC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
