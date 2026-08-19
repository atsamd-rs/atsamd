#[doc = "Register `CID2` reader"]
pub type R = crate::R<Cid2Spec>;
#[doc = "Field `PREAMBLEB2` reader - Preamble Byte 2"]
pub type Preambleb2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble Byte 2"]
    #[inline(always)]
    pub fn preambleb2(&self) -> Preambleb2R {
        Preambleb2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component Identification 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid2Spec;
impl crate::RegisterSpec for Cid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid2::R`](R) reader structure"]
impl crate::Readable for Cid2Spec {}
#[doc = "`reset()` method sets CID2 to value 0x05"]
impl crate::Resettable for Cid2Spec {
    const RESET_VALUE: u32 = 0x05;
}
