#[doc = "Register `IN%s` reader"]
pub type R = crate::R<IN_SPEC>;
#[doc = "Field `IN` reader - Port Data Input Value"]
pub type IN_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Input Value"]
    #[inline(always)]
    pub fn in_(&self) -> IN_R {
        IN_R::new(self.bits)
    }
}
#[doc = "Data Input Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for IN_SPEC {}
#[doc = "`reset()` method sets IN%s to value 0"]
impl crate::Resettable for IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
