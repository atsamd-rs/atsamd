#[doc = "Register `PEFTSH` reader"]
pub type R = crate::R<PeftshSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`peftsh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeftshSpec;
impl crate::RegisterSpec for PeftshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peftsh::R`](R) reader structure"]
impl crate::Readable for PeftshSpec {}
#[doc = "`reset()` method sets PEFTSH to value 0"]
impl crate::Resettable for PeftshSpec {
    const RESET_VALUE: u32 = 0;
}
