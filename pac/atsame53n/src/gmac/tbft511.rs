#[doc = "Register `TBFT511` reader"]
pub type R = crate::R<Tbft511Spec>;
#[doc = "Field `NFTX` reader - 256 to 511 Byte Frames Transmitted without Error"]
pub type NftxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 256 to 511 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NftxR {
        NftxR::new(self.bits)
    }
}
#[doc = "256 to 511 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft511::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbft511Spec;
impl crate::RegisterSpec for Tbft511Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbft511::R`](R) reader structure"]
impl crate::Readable for Tbft511Spec {}
#[doc = "`reset()` method sets TBFT511 to value 0"]
impl crate::Resettable for Tbft511Spec {}
