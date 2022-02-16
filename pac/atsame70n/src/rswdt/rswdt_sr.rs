#[doc = "Register `RSWDT_SR` reader"]
pub struct R(crate::R<RSWDT_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSWDT_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSWDT_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSWDT_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDUNF` reader - Watchdog Underflow"]
pub struct WDUNF_R(crate::FieldReader<bool, bool>);
impl WDUNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDUNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDUNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Underflow"]
    #[inline(always)]
    pub fn wdunf(&self) -> WDUNF_R {
        WDUNF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswdt_sr](index.html) module"]
pub struct RSWDT_SR_SPEC;
impl crate::RegisterSpec for RSWDT_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rswdt_sr::R](R) reader structure"]
impl crate::Readable for RSWDT_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSWDT_SR to value 0"]
impl crate::Resettable for RSWDT_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
