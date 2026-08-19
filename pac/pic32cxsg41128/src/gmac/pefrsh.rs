#[doc = "Register `PEFRSH` reader"]
pub type R = crate::R<PefrshSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Peer Event Frame Received Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pefrsh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PefrshSpec;
impl crate::RegisterSpec for PefrshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pefrsh::R`](R) reader structure"]
impl crate::Readable for PefrshSpec {}
#[doc = "`reset()` method sets PEFRSH to value 0"]
impl crate::Resettable for PefrshSpec {
    const RESET_VALUE: u32 = 0;
}
