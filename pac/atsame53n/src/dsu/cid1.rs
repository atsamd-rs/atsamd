#[doc = "Register `CID1` reader"]
pub type R = crate::R<CID1_SPEC>;
#[doc = "Field `PREAMBLE` reader - Preamble"]
pub type PREAMBLE_R = crate::FieldReader;
#[doc = "Field `CCLASS` reader - Component Class"]
pub type CCLASS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Preamble"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component Class"]
    #[inline(always)]
    pub fn cclass(&self) -> CCLASS_R {
        CCLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Component Identification 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CID1_SPEC;
impl crate::RegisterSpec for CID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid1::R`](R) reader structure"]
impl crate::Readable for CID1_SPEC {}
#[doc = "`reset()` method sets CID1 to value 0x10"]
impl crate::Resettable for CID1_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
