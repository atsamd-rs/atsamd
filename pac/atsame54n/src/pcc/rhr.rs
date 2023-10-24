#[doc = "Register `RHR` reader"]
pub type R = crate::R<RHR_SPEC>;
#[doc = "Field `RDATA` reader - Reception Data"]
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reception Data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
#[doc = "Reception Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RHR_SPEC;
impl crate::RegisterSpec for RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rhr::R`](R) reader structure"]
impl crate::Readable for RHR_SPEC {}
#[doc = "`reset()` method sets RHR to value 0"]
impl crate::Resettable for RHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
