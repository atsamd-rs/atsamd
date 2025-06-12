#[doc = "Register `EFTN` reader"]
pub type R = crate::R<EftnSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Event Frame Transmitted Nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`eftn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EftnSpec;
impl crate::RegisterSpec for EftnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eftn::R`](R) reader structure"]
impl crate::Readable for EftnSpec {}
#[doc = "`reset()` method sets EFTN to value 0"]
impl crate::Resettable for EftnSpec {}
