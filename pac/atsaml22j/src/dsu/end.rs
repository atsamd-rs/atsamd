#[doc = "Register `END` reader"]
pub struct R(crate::R<END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `END` reader - End Marker"]
pub type END_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - End Marker"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(self.bits)
    }
}
#[doc = "CoreSight ROM Table End\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [end](index.html) module"]
pub struct END_SPEC;
impl crate::RegisterSpec for END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [end::R](R) reader structure"]
impl crate::Readable for END_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets END to value 0"]
impl crate::Resettable for END_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
