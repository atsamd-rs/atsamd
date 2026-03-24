#[doc = "Register `TBFT1023` reader"]
pub type R = crate::R<Tbft1023Spec>;
#[doc = "Field `NFTX` reader - 512 to 1023 Byte Frames Transmitted without Error"]
pub type NftxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 512 to 1023 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NftxR {
        NftxR::new(self.bits)
    }
}
#[doc = "512 to 1023 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft1023::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbft1023Spec;
impl crate::RegisterSpec for Tbft1023Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbft1023::R`](R) reader structure"]
impl crate::Readable for Tbft1023Spec {}
#[doc = "`reset()` method sets TBFT1023 to value 0"]
impl crate::Resettable for Tbft1023Spec {
    const RESET_VALUE: u32 = 0;
}
