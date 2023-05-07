#[doc = "Register `SCF` reader"]
pub struct R(crate::R<SCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCOL` reader - Single Collision"]
pub type SCOL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Single Collision"]
    #[inline(always)]
    pub fn scol(&self) -> SCOL_R {
        SCOL_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Single Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scf](index.html) module"]
pub struct SCF_SPEC;
impl crate::RegisterSpec for SCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scf::R](R) reader structure"]
impl crate::Readable for SCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCF to value 0"]
impl crate::Resettable for SCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
