#[doc = "Register `BFT64` reader"]
pub type R = crate::R<Bft64Spec>;
#[doc = "Field `NFTX` reader - 64 Byte Frames Transmitted without Error"]
pub type NftxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 64 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NftxR {
        NftxR::new(self.bits)
    }
}
#[doc = "64 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bft64::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bft64Spec;
impl crate::RegisterSpec for Bft64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bft64::R`](R) reader structure"]
impl crate::Readable for Bft64Spec {}
#[doc = "`reset()` method sets BFT64 to value 0"]
impl crate::Resettable for Bft64Spec {}
