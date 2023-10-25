#[doc = "Register `END` reader"]
pub type R = crate::R<END_SPEC>;
#[doc = "Field `END` reader - End Marker"]
pub type END_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - End Marker"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(self.bits)
    }
}
#[doc = "CoreSight ROM Table End\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`end::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct END_SPEC;
impl crate::RegisterSpec for END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`end::R`](R) reader structure"]
impl crate::Readable for END_SPEC {}
#[doc = "`reset()` method sets END to value 0"]
impl crate::Resettable for END_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
