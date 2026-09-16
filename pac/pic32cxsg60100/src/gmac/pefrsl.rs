#[doc = "Register `PEFRSL` reader"]
pub type R = crate::R<PefrslSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(self.bits)
    }
}
#[doc = "PTP Peer Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pefrsl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PefrslSpec;
impl crate::RegisterSpec for PefrslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pefrsl::R`](R) reader structure"]
impl crate::Readable for PefrslSpec {}
#[doc = "`reset()` method sets PEFRSL to value 0"]
impl crate::Resettable for PefrslSpec {
    const RESET_VALUE: u32 = 0;
}
