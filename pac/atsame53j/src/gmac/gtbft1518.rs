#[doc = "Register `GTBFT1518` reader"]
pub struct R(crate::R<GTBFT1518_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTBFT1518_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTBFT1518_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTBFT1518_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NFTX` reader - Greater than 1518 Byte Frames Transmitted without Error"]
pub type NFTX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Greater than 1518 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits)
    }
}
#[doc = "Greater Than 1518 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtbft1518](index.html) module"]
pub struct GTBFT1518_SPEC;
impl crate::RegisterSpec for GTBFT1518_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtbft1518::R](R) reader structure"]
impl crate::Readable for GTBFT1518_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GTBFT1518 to value 0"]
impl crate::Resettable for GTBFT1518_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
