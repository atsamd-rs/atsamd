#[doc = "Register `BFT64` reader"]
pub type R = crate::R<BFT64_SPEC>;
#[doc = "Field `NFTX` reader - 64 Byte Frames Transmitted without Error"]
pub type NFTX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 64 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits)
    }
}
#[doc = "64 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bft64::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BFT64_SPEC;
impl crate::RegisterSpec for BFT64_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bft64::R`](R) reader structure"]
impl crate::Readable for BFT64_SPEC {}
#[doc = "`reset()` method sets BFT64 to value 0"]
impl crate::Resettable for BFT64_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
