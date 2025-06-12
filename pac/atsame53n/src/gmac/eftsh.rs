#[doc = "Register `EFTSH` reader"]
pub type R = crate::R<EftshSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eftsh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EftshSpec;
impl crate::RegisterSpec for EftshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eftsh::R`](R) reader structure"]
impl crate::Readable for EftshSpec {}
#[doc = "`reset()` method sets EFTSH to value 0"]
impl crate::Resettable for EftshSpec {}
