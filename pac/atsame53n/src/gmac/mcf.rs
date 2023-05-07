#[doc = "Register `MCF` reader"]
pub struct R(crate::R<MCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MCOL` reader - Multiple Collision"]
pub type MCOL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Multiple Collision"]
    #[inline(always)]
    pub fn mcol(&self) -> MCOL_R {
        MCOL_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Multiple Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcf](index.html) module"]
pub struct MCF_SPEC;
impl crate::RegisterSpec for MCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcf::R](R) reader structure"]
impl crate::Readable for MCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCF to value 0"]
impl crate::Resettable for MCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
