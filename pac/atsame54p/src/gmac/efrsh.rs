#[doc = "Register `EFRSH` reader"]
pub type R = crate::R<EfrshSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Event Frame Received Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`efrsh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfrshSpec;
impl crate::RegisterSpec for EfrshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efrsh::R`](R) reader structure"]
impl crate::Readable for EfrshSpec {}
#[doc = "`reset()` method sets EFRSH to value 0"]
impl crate::Resettable for EfrshSpec {}
