#[doc = "Register `ROE` reader"]
pub type R = crate::R<RoeSpec>;
#[doc = "Field `RXOVR` reader - Receive Overruns"]
pub type RxovrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Receive Overruns"]
    #[inline(always)]
    pub fn rxovr(&self) -> RxovrR {
        RxovrR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receive Overrun Register\n\nYou can [`read`](crate::Reg::read) this register and get [`roe::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RoeSpec;
impl crate::RegisterSpec for RoeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`roe::R`](R) reader structure"]
impl crate::Readable for RoeSpec {}
#[doc = "`reset()` method sets ROE to value 0"]
impl crate::Resettable for RoeSpec {}
