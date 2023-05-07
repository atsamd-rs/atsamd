#[doc = "Register `IN%s` reader"]
pub struct R(crate::R<IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN` reader - Port Data Input Value"]
pub type IN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Input Value"]
    #[inline(always)]
    pub fn in_(&self) -> IN_R {
        IN_R::new(self.bits)
    }
}
#[doc = "Data Input Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_](index.html) module"]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_::R](R) reader structure"]
impl crate::Readable for IN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN%s to value 0"]
impl crate::Resettable for IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
