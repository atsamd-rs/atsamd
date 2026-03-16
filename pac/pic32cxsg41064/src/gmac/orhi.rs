#[doc = "Register `ORHI` reader"]
pub type R = crate::R<OrhiSpec>;
#[doc = "Field `RXO` reader - Received Octets"]
pub type RxoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Received Octets"]
    #[inline(always)]
    pub fn rxo(&self) -> RxoR {
        RxoR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Octets Received \\[47:32\\]
Received\n\nYou can [`read`](crate::Reg::read) this register and get [`orhi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrhiSpec;
impl crate::RegisterSpec for OrhiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`orhi::R`](R) reader structure"]
impl crate::Readable for OrhiSpec {}
#[doc = "`reset()` method sets ORHI to value 0"]
impl crate::Resettable for OrhiSpec {
    const RESET_VALUE: u32 = 0;
}
