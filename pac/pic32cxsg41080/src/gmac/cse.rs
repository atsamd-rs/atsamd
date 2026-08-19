#[doc = "Register `CSE` reader"]
pub type R = crate::R<CseSpec>;
#[doc = "Field `CSR` reader - Carrier Sense Error"]
pub type CsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Carrier Sense Error"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Carrier Sense Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cse::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CseSpec;
impl crate::RegisterSpec for CseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cse::R`](R) reader structure"]
impl crate::Readable for CseSpec {}
#[doc = "`reset()` method sets CSE to value 0"]
impl crate::Resettable for CseSpec {
    const RESET_VALUE: u32 = 0;
}
