#[doc = "Register `CID3` reader"]
pub type R = crate::R<CID3_SPEC>;
#[doc = "Field `PREAMBLEB3` reader - Preamble Byte 3"]
pub type PREAMBLEB3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble Byte 3"]
    #[inline(always)]
    pub fn preambleb3(&self) -> PREAMBLEB3_R {
        PREAMBLEB3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component Identification 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CID3_SPEC;
impl crate::RegisterSpec for CID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid3::R`](R) reader structure"]
impl crate::Readable for CID3_SPEC {}
#[doc = "`reset()` method sets CID3 to value 0xb1"]
impl crate::Resettable for CID3_SPEC {
    const RESET_VALUE: Self::Ux = 0xb1;
}
