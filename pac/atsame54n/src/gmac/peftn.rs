#[doc = "Register `PEFTN` reader"]
pub type R = crate::R<PeftnSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`peftn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeftnSpec;
impl crate::RegisterSpec for PeftnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peftn::R`](R) reader structure"]
impl crate::Readable for PeftnSpec {}
#[doc = "`reset()` method sets PEFTN to value 0"]
impl crate::Resettable for PeftnSpec {}
