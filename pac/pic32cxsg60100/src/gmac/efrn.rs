#[doc = "Register `EFRN` reader"]
pub type R = crate::R<EfrnSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Event Frame Received Nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`efrn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfrnSpec;
impl crate::RegisterSpec for EfrnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efrn::R`](R) reader structure"]
impl crate::Readable for EfrnSpec {}
#[doc = "`reset()` method sets EFRN to value 0"]
impl crate::Resettable for EfrnSpec {
    const RESET_VALUE: u32 = 0;
}
