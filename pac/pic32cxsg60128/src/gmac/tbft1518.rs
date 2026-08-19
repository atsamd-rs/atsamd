#[doc = "Register `TBFT1518` reader"]
pub type R = crate::R<Tbft1518Spec>;
#[doc = "Field `NFTX` reader - 1024 to 1518 Byte Frames Transmitted without Error"]
pub type NftxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 1024 to 1518 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NftxR {
        NftxR::new(self.bits)
    }
}
#[doc = "1024 to 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft1518::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbft1518Spec;
impl crate::RegisterSpec for Tbft1518Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbft1518::R`](R) reader structure"]
impl crate::Readable for Tbft1518Spec {}
#[doc = "`reset()` method sets TBFT1518 to value 0"]
impl crate::Resettable for Tbft1518Spec {
    const RESET_VALUE: u32 = 0;
}
