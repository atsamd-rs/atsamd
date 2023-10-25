#[doc = "Register `CID2` reader"]
pub type R = crate::R<CID2_SPEC>;
#[doc = "Field `PREAMBLEB2` reader - Preamble Byte 2"]
pub type PREAMBLEB2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble Byte 2"]
    #[inline(always)]
    pub fn preambleb2(&self) -> PREAMBLEB2_R {
        PREAMBLEB2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component Identification 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CID2_SPEC;
impl crate::RegisterSpec for CID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid2::R`](R) reader structure"]
impl crate::Readable for CID2_SPEC {}
#[doc = "`reset()` method sets CID2 to value 0x05"]
impl crate::Resettable for CID2_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
