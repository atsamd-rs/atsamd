#[doc = "Register `WPSR` reader"]
pub struct R(crate::R<WPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WPVS` reader - Write Protection Violation Source"]
pub type WPVS_R = crate::BitReader<bool>;
#[doc = "Field `WPVSRC` reader - Write Protection Violation Status"]
pub type WPVSRC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](index.html) module"]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpsr::R](R) reader structure"]
impl crate::Readable for WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
