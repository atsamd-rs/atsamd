#[doc = "Register `VALUE` reader"]
pub type R = crate::R<VALUE_SPEC>;
#[doc = "Field `VALUE` reader - Measurement Value"]
pub type VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Measurement Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Count Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VALUE_SPEC;
impl crate::RegisterSpec for VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for VALUE_SPEC {}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
