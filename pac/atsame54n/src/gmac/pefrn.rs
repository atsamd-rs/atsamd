#[doc = "Register `PEFRN` reader"]
pub type R = crate::R<PefrnSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Peer Event Frame Received Nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`pefrn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PefrnSpec;
impl crate::RegisterSpec for PefrnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pefrn::R`](R) reader structure"]
impl crate::Readable for PefrnSpec {}
#[doc = "`reset()` method sets PEFRN to value 0"]
impl crate::Resettable for PefrnSpec {
    const RESET_VALUE: u32 = 0;
}
