#[doc = "Register `TWIHS_WPSR` reader"]
pub struct R(crate::R<TWIHS_WPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_WPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_WPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_WPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WPVS` reader - Write Protection Violation Status"]
pub struct WPVS_R(crate::FieldReader<bool, bool>);
impl WPVS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPVS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPVSRC` reader - Write Protection Violation Source"]
pub struct WPVSRC_R(crate::FieldReader<u32, u32>);
impl WPVSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WPVSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPVSRC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_wpsr](index.html) module"]
pub struct TWIHS_WPSR_SPEC;
impl crate::RegisterSpec for TWIHS_WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_wpsr::R](R) reader structure"]
impl crate::Readable for TWIHS_WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TWIHS_WPSR to value 0"]
impl crate::Resettable for TWIHS_WPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
