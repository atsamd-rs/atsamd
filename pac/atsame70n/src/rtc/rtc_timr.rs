#[doc = "Register `RTC_TIMR` reader"]
pub struct R(crate::R<RTC_TIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIMR` writer"]
pub struct W(crate::W<RTC_TIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_TIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - Current Second"]
pub struct SEC_R(crate::FieldReader<u8, u8>);
impl SEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC` writer - Current Second"]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `MIN` reader - Current Minute"]
pub struct MIN_R(crate::FieldReader<u8, u8>);
impl MIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN` writer - Current Minute"]
pub struct MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `HOUR` reader - Current Hour"]
pub struct HOUR_R(crate::FieldReader<u8, u8>);
impl HOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR` writer - Current Hour"]
pub struct HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `AMPM` reader - Ante Meridiem Post Meridiem Indicator"]
pub struct AMPM_R(crate::FieldReader<bool, bool>);
impl AMPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMPM` writer - Ante Meridiem Post Meridiem Indicator"]
pub struct AMPM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Current Second"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Current Minute"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Current Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Ante Meridiem Post Meridiem Indicator"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current Second"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
    #[doc = "Bits 8:14 - Current Minute"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W {
        MIN_W { w: self }
    }
    #[doc = "Bits 16:21 - Current Hour"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W { w: self }
    }
    #[doc = "Bit 22 - Ante Meridiem Post Meridiem Indicator"]
    #[inline(always)]
    pub fn ampm(&mut self) -> AMPM_W {
        AMPM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timr](index.html) module"]
pub struct RTC_TIMR_SPEC;
impl crate::RegisterSpec for RTC_TIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_timr::R](R) reader structure"]
impl crate::Readable for RTC_TIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_timr::W](W) writer structure"]
impl crate::Writable for RTC_TIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIMR to value 0"]
impl crate::Resettable for RTC_TIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
