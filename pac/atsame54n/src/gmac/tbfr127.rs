#[doc = "Register `TBFR127` reader"]
pub struct R(crate::R<TBFR127_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBFR127_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBFR127_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBFR127_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NFRX` reader - 65 to 127 Byte Frames Received without Error"]
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
    #[doc = "Bits 0:31 - 65 to 127 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new(self.bits as u32)
    }
}
#[doc = "65 to 127 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr127](index.html) module"]
pub struct TBFR127_SPEC;
impl crate::RegisterSpec for TBFR127_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbfr127::R](R) reader structure"]
impl crate::Readable for TBFR127_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBFR127 to value 0"]
impl crate::Resettable for TBFR127_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
