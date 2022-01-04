#[doc = "Register `PFT` reader"]
pub struct R(crate::R<PFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PFTX` reader - Pause Frames Transmitted Register"]
pub struct PFTX_R(crate::FieldReader<u16, u16>);
impl PFTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PFTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFTX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Pause Frames Transmitted Register"]
    #[inline(always)]
    pub fn pftx(&self) -> PFTX_R {
        PFTX_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pause Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pft](index.html) module"]
pub struct PFT_SPEC;
impl crate::RegisterSpec for PFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pft::R](R) reader structure"]
impl crate::Readable for PFT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PFT to value 0"]
impl crate::Resettable for PFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
