#[doc = "Register `GTBFT1518` reader"]
pub type R = crate::R<GTBFT1518_SPEC>;
#[doc = "Field `NFTX` reader - Greater than 1518 Byte Frames Transmitted without Error"]
pub type NFTX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Greater than 1518 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits)
    }
}
#[doc = "Greater Than 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtbft1518::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTBFT1518_SPEC;
impl crate::RegisterSpec for GTBFT1518_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtbft1518::R`](R) reader structure"]
impl crate::Readable for GTBFT1518_SPEC {}
#[doc = "`reset()` method sets GTBFT1518 to value 0"]
impl crate::Resettable for GTBFT1518_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
