#[doc = "Register `TBFT127` reader"]
pub struct R(crate::R<TBFT127_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBFT127_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBFT127_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBFT127_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NFTX` reader - 65 to 127 Byte Frames Transmitted without Error"]
pub type NFTX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 65 to 127 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits)
    }
}
#[doc = "65 to 127 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft127](index.html) module"]
pub struct TBFT127_SPEC;
impl crate::RegisterSpec for TBFT127_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbft127::R](R) reader structure"]
impl crate::Readable for TBFT127_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBFT127 to value 0"]
impl crate::Resettable for TBFT127_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
