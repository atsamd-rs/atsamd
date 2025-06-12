#[doc = "Register `RRE` reader"]
pub type R = crate::R<RreSpec>;
#[doc = "Field `RXRER` reader - Receive Resource Errors"]
pub type RxrerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rxrer(&self) -> RxrerR {
        RxrerR::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Receive Resource Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rre::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RreSpec;
impl crate::RegisterSpec for RreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rre::R`](R) reader structure"]
impl crate::Readable for RreSpec {}
#[doc = "`reset()` method sets RRE to value 0"]
impl crate::Resettable for RreSpec {}
