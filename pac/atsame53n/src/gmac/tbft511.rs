#[doc = "Register `TBFT511` reader"]
pub struct R(crate::R<TBFT511_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBFT511_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBFT511_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBFT511_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NFTX` reader - 256 to 511 Byte Frames Transmitted without Error"]
pub struct NFTX_R(crate::FieldReader<u32, u32>);
impl NFTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        NFTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFTX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - 256 to 511 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits as u32)
    }
}
#[doc = "256 to 511 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft511](index.html) module"]
pub struct TBFT511_SPEC;
impl crate::RegisterSpec for TBFT511_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbft511::R](R) reader structure"]
impl crate::Readable for TBFT511_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBFT511 to value 0"]
impl crate::Resettable for TBFT511_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
