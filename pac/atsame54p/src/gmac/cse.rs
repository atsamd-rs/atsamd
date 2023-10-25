#[doc = "Register `CSE` reader"]
pub type R = crate::R<CSE_SPEC>;
#[doc = "Field `CSR` reader - Carrier Sense Error"]
pub type CSR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Carrier Sense Error"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Carrier Sense Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cse::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSE_SPEC;
impl crate::RegisterSpec for CSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cse::R`](R) reader structure"]
impl crate::Readable for CSE_SPEC {}
#[doc = "`reset()` method sets CSE to value 0"]
impl crate::Resettable for CSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
