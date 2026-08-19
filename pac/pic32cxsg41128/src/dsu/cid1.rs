#[doc = "Register `CID1` reader"]
pub type R = crate::R<Cid1Spec>;
#[doc = "Field `PREAMBLE` reader - Preamble"]
pub type PreambleR = crate::FieldReader;
#[doc = "Field `CCLASS` reader - Component Class"]
pub type CclassR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Preamble"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component Class"]
    #[inline(always)]
    pub fn cclass(&self) -> CclassR {
        CclassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Component Identification 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid1Spec;
impl crate::RegisterSpec for Cid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid1::R`](R) reader structure"]
impl crate::Readable for Cid1Spec {}
#[doc = "`reset()` method sets CID1 to value 0x10"]
impl crate::Resettable for Cid1Spec {
    const RESET_VALUE: u32 = 0x10;
}
