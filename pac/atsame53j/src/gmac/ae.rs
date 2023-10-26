#[doc = "Register `AE` reader"]
pub type R = crate::R<AE_SPEC>;
#[doc = "Field `AER` reader - Alignment Errors"]
pub type AER_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Alignment Errors"]
    #[inline(always)]
    pub fn aer(&self) -> AER_R {
        AER_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Alignment Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ae::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AE_SPEC;
impl crate::RegisterSpec for AE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae::R`](R) reader structure"]
impl crate::Readable for AE_SPEC {}
#[doc = "`reset()` method sets AE to value 0"]
impl crate::Resettable for AE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
