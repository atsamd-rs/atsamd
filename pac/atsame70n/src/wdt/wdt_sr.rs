#[doc = "Register `WDT_SR` reader"]
pub struct R(crate::R<WDT_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDUNF` reader - Watchdog Underflow (cleared on read)"]
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
#[doc = "Field `WDERR` reader - Watchdog Error (cleared on read)"]
pub struct WDERR_R(crate::FieldReader<bool, bool>);
impl WDERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Underflow (cleared on read)"]
    #[inline(always)]
    pub fn wdunf(&self) -> WDUNF_R {
        WDUNF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog Error (cleared on read)"]
    #[inline(always)]
    pub fn wderr(&self) -> WDERR_R {
        WDERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_sr](index.html) module"]
pub struct WDT_SR_SPEC;
impl crate::RegisterSpec for WDT_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_sr::R](R) reader structure"]
impl crate::Readable for WDT_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDT_SR to value 0"]
impl crate::Resettable for WDT_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
