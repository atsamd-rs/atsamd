#[doc = "Register `GMAC_MCF` reader"]
pub struct R(crate::R<GMAC_MCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_MCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_MCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_MCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MCOL` reader - Multiple Collision"]
pub struct MCOL_R(crate::FieldReader<u32, u32>);
impl MCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCOL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - Multiple Collision"]
    #[inline(always)]
    pub fn mcol(&self) -> MCOL_R {
        MCOL_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "Multiple Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_mcf](index.html) module"]
pub struct GMAC_MCF_SPEC;
impl crate::RegisterSpec for GMAC_MCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_mcf::R](R) reader structure"]
impl crate::Readable for GMAC_MCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_MCF to value 0"]
impl crate::Resettable for GMAC_MCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
