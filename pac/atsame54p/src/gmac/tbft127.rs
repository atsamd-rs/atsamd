#[doc = "Register `TBFT127` reader"]
pub type R = crate::R<Tbft127Spec>;
#[doc = "Field `NFTX` reader - 65 to 127 Byte Frames Transmitted without Error"]
pub type NftxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 65 to 127 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NftxR {
        NftxR::new(self.bits)
    }
}
#[doc = "65 to 127 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft127::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbft127Spec;
impl crate::RegisterSpec for Tbft127Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbft127::R`](R) reader structure"]
impl crate::Readable for Tbft127Spec {}
#[doc = "`reset()` method sets TBFT127 to value 0"]
impl crate::Resettable for Tbft127Spec {}
