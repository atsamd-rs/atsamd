#[doc = "Register `GTBFT1518` reader"]
pub type R = crate::R<Gtbft1518Spec>;
#[doc = "Field `NFTX` reader - Greater than 1518 Byte Frames Transmitted without Error"]
pub type NftxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Greater than 1518 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NftxR {
        NftxR::new(self.bits)
    }
}
#[doc = "Greater Than 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtbft1518::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gtbft1518Spec;
impl crate::RegisterSpec for Gtbft1518Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtbft1518::R`](R) reader structure"]
impl crate::Readable for Gtbft1518Spec {}
#[doc = "`reset()` method sets GTBFT1518 to value 0"]
impl crate::Resettable for Gtbft1518Spec {
    const RESET_VALUE: u32 = 0;
}
