#[doc = "Register `RTC_CALALR` reader"]
pub struct R(crate::R<RTC_CALALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CALALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CALALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CALALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CALALR` writer"]
pub struct W(crate::W<RTC_CALALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CALALR_SPEC>;
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
impl From<crate::W<RTC_CALALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CALALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MONTH` reader - Month Alarm"]
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
#[doc = "Field `MONTH` writer - Month Alarm"]
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
#[doc = "Field `MTHEN` reader - Month Alarm Enable"]
pub struct MTHEN_R(crate::FieldReader<bool, bool>);
impl MTHEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MTHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTHEN` writer - Month Alarm Enable"]
pub struct MTHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MTHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `DATE` reader - Date Alarm"]
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
#[doc = "Field `DATE` writer - Date Alarm"]
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
#[doc = "Field `DATEEN` reader - Date Alarm Enable"]
pub struct DATEEN_R(crate::FieldReader<bool, bool>);
impl DATEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATEEN` writer - Date Alarm Enable"]
pub struct DATEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:20 - Month Alarm"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Month Alarm Enable"]
    #[inline(always)]
    pub fn mthen(&self) -> MTHEN_R {
        MTHEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - Date Alarm"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Date Alarm Enable"]
    #[inline(always)]
    pub fn dateen(&self) -> DATEEN_R {
        DATEEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - Month Alarm"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W { w: self }
    }
    #[doc = "Bit 23 - Month Alarm Enable"]
    #[inline(always)]
    pub fn mthen(&mut self) -> MTHEN_W {
        MTHEN_W { w: self }
    }
    #[doc = "Bits 24:29 - Date Alarm"]
    #[inline(always)]
    pub fn date(&mut self) -> DATE_W {
        DATE_W { w: self }
    }
    #[doc = "Bit 31 - Date Alarm Enable"]
    #[inline(always)]
    pub fn dateen(&mut self) -> DATEEN_W {
        DATEEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calendar Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_calalr](index.html) module"]
pub struct RTC_CALALR_SPEC;
impl crate::RegisterSpec for RTC_CALALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_calalr::R](R) reader structure"]
impl crate::Readable for RTC_CALALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_calalr::W](W) writer structure"]
impl crate::Writable for RTC_CALALR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CALALR to value 0"]
impl crate::Resettable for RTC_CALALR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
