#[doc = "Register `RTC_CALR` reader"]
pub struct R(crate::R<RTC_CALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CALR` writer"]
pub struct W(crate::W<RTC_CALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CALR_SPEC>;
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
impl From<crate::W<RTC_CALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENT` reader - Current Century"]
pub struct CENT_R(crate::FieldReader<u8, u8>);
impl CENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CENT` writer - Current Century"]
pub struct CENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `YEAR` reader - Current Year"]
pub struct YEAR_R(crate::FieldReader<u8, u8>);
impl YEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        YEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YEAR` writer - Current Year"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `MONTH` reader - Current Month"]
pub struct MONTH_R(crate::FieldReader<u8, u8>);
impl MONTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MONTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH` writer - Current Month"]
pub struct MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `DAY` reader - Current Day in Current Week"]
pub struct DAY_R(crate::FieldReader<u8, u8>);
impl DAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY` writer - Current Day in Current Week"]
pub struct DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `DATE` reader - Current Day in Current Month"]
pub struct DATE_R(crate::FieldReader<u8, u8>);
impl DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATE` writer - Current Day in Current Month"]
pub struct DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Current Century"]
    #[inline(always)]
    pub fn cent(&self) -> CENT_R {
        CENT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Current Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Current Month"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Current Day in Current Week"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:29 - Current Day in Current Month"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current Century"]
    #[inline(always)]
    pub fn cent(&mut self) -> CENT_W {
        CENT_W { w: self }
    }
    #[doc = "Bits 8:15 - Current Year"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
    #[doc = "Bits 16:20 - Current Month"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W { w: self }
    }
    #[doc = "Bits 21:23 - Current Day in Current Week"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W { w: self }
    }
    #[doc = "Bits 24:29 - Current Day in Current Month"]
    #[inline(always)]
    pub fn date(&mut self) -> DATE_W {
        DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calendar Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_calr](index.html) module"]
pub struct RTC_CALR_SPEC;
impl crate::RegisterSpec for RTC_CALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_calr::R](R) reader structure"]
impl crate::Readable for RTC_CALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_calr::W](W) writer structure"]
impl crate::Writable for RTC_CALR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CALR to value 0"]
impl crate::Resettable for RTC_CALR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
