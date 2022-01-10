#[doc = "Register `FT` reader"]
pub struct R(crate::R<FT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FTX` reader - Frames Transmitted without Error"]
pub struct FTX_R(crate::FieldReader<u32, u32>);
impl FTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Frames Transmitted without Error"]
    #[inline(always)]
    pub fn ftx(&self) -> FTX_R {
        FTX_R::new(self.bits as u32)
    }
}
#[doc = "Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ft](index.html) module"]
pub struct FT_SPEC;
impl crate::RegisterSpec for FT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ft::R](R) reader structure"]
impl crate::Readable for FT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FT to value 0"]
impl crate::Resettable for FT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
