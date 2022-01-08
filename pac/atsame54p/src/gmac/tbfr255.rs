#[doc = "Register `TBFR255` reader"]
pub struct R(crate::R<TBFR255_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBFR255_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBFR255_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBFR255_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NFRX` reader - 128 to 255 Byte Frames Received without Error"]
pub struct NFRX_R(crate::FieldReader<u32, u32>);
impl NFRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        NFRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFRX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - 128 to 255 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new(self.bits as u32)
    }
}
#[doc = "128 to 255 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr255](index.html) module"]
pub struct TBFR255_SPEC;
impl crate::RegisterSpec for TBFR255_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbfr255::R](R) reader structure"]
impl crate::Readable for TBFR255_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBFR255 to value 0"]
impl crate::Resettable for TBFR255_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
