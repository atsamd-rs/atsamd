#[doc = "Register `PEFTSL` reader"]
pub type R = crate::R<PeftslSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(self.bits)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`peftsl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeftslSpec;
impl crate::RegisterSpec for PeftslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peftsl::R`](R) reader structure"]
impl crate::Readable for PeftslSpec {}
#[doc = "`reset()` method sets PEFTSL to value 0"]
impl crate::Resettable for PeftslSpec {
    const RESET_VALUE: u32 = 0;
}
