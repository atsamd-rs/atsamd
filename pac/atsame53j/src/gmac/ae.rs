#[doc = "Register `AE` reader"]
pub type R = crate::R<AeSpec>;
#[doc = "Field `AER` reader - Alignment Errors"]
pub type AerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Alignment Errors"]
    #[inline(always)]
    pub fn aer(&self) -> AerR {
        AerR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Alignment Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ae::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeSpec;
impl crate::RegisterSpec for AeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae::R`](R) reader structure"]
impl crate::Readable for AeSpec {}
#[doc = "`reset()` method sets AE to value 0"]
impl crate::Resettable for AeSpec {}
