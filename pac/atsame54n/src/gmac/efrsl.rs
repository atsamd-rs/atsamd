#[doc = "Register `EFRSL` reader"]
pub type R = crate::R<EfrslSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(self.bits)
    }
}
#[doc = "PTP Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`efrsl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfrslSpec;
impl crate::RegisterSpec for EfrslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efrsl::R`](R) reader structure"]
impl crate::Readable for EfrslSpec {}
#[doc = "`reset()` method sets EFRSL to value 0"]
impl crate::Resettable for EfrslSpec {}
