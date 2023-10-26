#[doc = "Register `RSE` reader"]
pub type R = crate::R<RSE_SPEC>;
#[doc = "Field `RXSE` reader - Receive Symbol Errors"]
pub type RXSE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rxse(&self) -> RXSE_R {
        RXSE_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receive Symbol Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rse::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSE_SPEC;
impl crate::RegisterSpec for RSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rse::R`](R) reader structure"]
impl crate::Readable for RSE_SPEC {}
#[doc = "`reset()` method sets RSE to value 0"]
impl crate::Resettable for RSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
