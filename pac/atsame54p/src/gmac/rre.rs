#[doc = "Register `RRE` reader"]
pub type R = crate::R<RRE_SPEC>;
#[doc = "Field `RXRER` reader - Receive Resource Errors"]
pub type RXRER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rxrer(&self) -> RXRER_R {
        RXRER_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Receive Resource Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rre::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RRE_SPEC;
impl crate::RegisterSpec for RRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rre::R`](R) reader structure"]
impl crate::Readable for RRE_SPEC {}
#[doc = "`reset()` method sets RRE to value 0"]
impl crate::Resettable for RRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
