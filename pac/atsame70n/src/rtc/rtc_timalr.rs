#[doc = "Register `RTC_TIMALR` reader"]
pub struct R(crate::R<RTC_TIMALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIMALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIMALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIMALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIMALR` writer"]
pub struct W(crate::W<RTC_TIMALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIMALR_SPEC>;
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
impl From<crate::W<RTC_TIMALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIMALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - Second Alarm"]
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
#[doc = "Field `SEC` writer - Second Alarm"]
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
#[doc = "Field `SECEN` reader - Second Alarm Enable"]
pub struct SECEN_R(crate::FieldReader<bool, bool>);
impl SECEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SECEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECEN` writer - Second Alarm Enable"]
pub struct SECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SECEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `MIN` reader - Minute Alarm"]
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
#[doc = "Field `MIN` writer - Minute Alarm"]
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
#[doc = "Field `MINEN` reader - Minute Alarm Enable"]
pub struct MINEN_R(crate::FieldReader<bool, bool>);
impl MINEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MINEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINEN` writer - Minute Alarm Enable"]
pub struct MINEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MINEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `HOUR` reader - Hour Alarm"]
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
#[doc = "Field `HOUR` writer - Hour Alarm"]
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
#[doc = "Field `AMPM` reader - AM/PM Indicator"]
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
#[doc = "Field `AMPM` writer - AM/PM Indicator"]
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
#[doc = "Field `HOUREN` reader - Hour Alarm Enable"]
pub struct HOUREN_R(crate::FieldReader<bool, bool>);
impl HOUREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOUREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUREN` writer - Hour Alarm Enable"]
pub struct HOUREN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUREN_W<'a> {
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
impl R {
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    pub fn secen(&self) -> SECEN_R {
        SECEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    pub fn minen(&self) -> MINEN_R {
        MINEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    pub fn houren(&self) -> HOUREN_R {
        HOUREN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    pub fn secen(&mut self) -> SECEN_W {
        SECEN_W { w: self }
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W {
        MIN_W { w: self }
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    pub fn minen(&mut self) -> MINEN_W {
        MINEN_W { w: self }
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W { w: self }
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    pub fn ampm(&mut self) -> AMPM_W {
        AMPM_W { w: self }
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    pub fn houren(&mut self) -> HOUREN_W {
        HOUREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timalr](index.html) module"]
pub struct RTC_TIMALR_SPEC;
impl crate::RegisterSpec for RTC_TIMALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_timalr::R](R) reader structure"]
impl crate::Readable for RTC_TIMALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_timalr::W](W) writer structure"]
impl crate::Writable for RTC_TIMALR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIMALR to value 0"]
impl crate::Resettable for RTC_TIMALR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
