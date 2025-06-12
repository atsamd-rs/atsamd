#[doc = "Register `TBFT255` reader"]
pub type R = crate::R<Tbft255Spec>;
#[doc = "Field `NFTX` reader - 128 to 255 Byte Frames Transmitted without Error"]
pub type NftxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 128 to 255 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NftxR {
        NftxR::new(self.bits)
    }
}
#[doc = "128 to 255 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft255::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbft255Spec;
impl crate::RegisterSpec for Tbft255Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbft255::R`](R) reader structure"]
impl crate::Readable for Tbft255Spec {}
#[doc = "`reset()` method sets TBFT255 to value 0"]
impl crate::Resettable for Tbft255Spec {}
