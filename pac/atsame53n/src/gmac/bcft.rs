#[doc = "Register `BCFT` reader"]
pub struct R(crate::R<BCFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BFTX` reader - Broadcast Frames Transmitted without Error"]
pub struct BFTX_R(crate::FieldReader<u32, u32>);
impl BFTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BFTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BFTX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Broadcast Frames Transmitted without Error"]
    #[inline(always)]
    pub fn bftx(&self) -> BFTX_R {
        BFTX_R::new(self.bits as u32)
    }
}
#[doc = "Broadcast Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcft](index.html) module"]
pub struct BCFT_SPEC;
impl crate::RegisterSpec for BCFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcft::R](R) reader structure"]
impl crate::Readable for BCFT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BCFT to value 0"]
impl crate::Resettable for BCFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
