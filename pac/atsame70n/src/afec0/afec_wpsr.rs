#[doc = "Register `AFEC_WPSR` reader"]
pub struct R(crate::R<AFEC_WPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_WPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_WPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_WPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WPVS` reader - Write Protect Violation Status"]
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
#[doc = "Field `WPVSRC` reader - Write Protect Violation Source"]
pub struct WPVSRC_R(crate::FieldReader<u16, u16>);
impl WPVSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WPVSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPVSRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:23 - Write Protect Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "AFEC Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_wpsr](index.html) module"]
pub struct AFEC_WPSR_SPEC;
impl crate::RegisterSpec for AFEC_WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_wpsr::R](R) reader structure"]
impl crate::Readable for AFEC_WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AFEC_WPSR to value 0"]
impl crate::Resettable for AFEC_WPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
