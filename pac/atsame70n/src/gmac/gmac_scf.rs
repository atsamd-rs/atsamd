#[doc = "Register `GMAC_SCF` reader"]
pub struct R(crate::R<GMAC_SCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_SCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_SCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_SCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCOL` reader - Single Collision"]
pub struct SCOL_R(crate::FieldReader<u32, u32>);
impl SCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCOL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - Single Collision"]
    #[inline(always)]
    pub fn scol(&self) -> SCOL_R {
        SCOL_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "Single Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_scf](index.html) module"]
pub struct GMAC_SCF_SPEC;
impl crate::RegisterSpec for GMAC_SCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_scf::R](R) reader structure"]
impl crate::Readable for GMAC_SCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_SCF to value 0"]
impl crate::Resettable for GMAC_SCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
