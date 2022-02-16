#[doc = "Register `RTC_CR` reader"]
pub struct R(crate::R<RTC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CR` writer"]
pub struct W(crate::W<RTC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CR_SPEC>;
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
impl From<crate::W<RTC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDTIM` reader - Update Request Time Register"]
pub struct UPDTIM_R(crate::FieldReader<bool, bool>);
impl UPDTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPDTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDTIM` writer - Update Request Time Register"]
pub struct UPDTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `UPDCAL` reader - Update Request Calendar Register"]
pub struct UPDCAL_R(crate::FieldReader<bool, bool>);
impl UPDCAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPDCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDCAL` writer - Update Request Calendar Register"]
pub struct UPDCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDCAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Time Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEVSEL_A {
    #[doc = "0: Minute change"]
    MINUTE = 0,
    #[doc = "1: Hour change"]
    HOUR = 1,
    #[doc = "2: Every day at midnight"]
    MIDNIGHT = 2,
    #[doc = "3: Every day at noon"]
    NOON = 3,
}
impl From<TIMEVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMEVSEL` reader - Time Event Selection"]
pub struct TIMEVSEL_R(crate::FieldReader<u8, TIMEVSEL_A>);
impl TIMEVSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMEVSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEVSEL_A {
        match self.bits {
            0 => TIMEVSEL_A::MINUTE,
            1 => TIMEVSEL_A::HOUR,
            2 => TIMEVSEL_A::MIDNIGHT,
            3 => TIMEVSEL_A::NOON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MINUTE`"]
    #[inline(always)]
    pub fn is_minute(&self) -> bool {
        **self == TIMEVSEL_A::MINUTE
    }
    #[doc = "Checks if the value of the field is `HOUR`"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        **self == TIMEVSEL_A::HOUR
    }
    #[doc = "Checks if the value of the field is `MIDNIGHT`"]
    #[inline(always)]
    pub fn is_midnight(&self) -> bool {
        **self == TIMEVSEL_A::MIDNIGHT
    }
    #[doc = "Checks if the value of the field is `NOON`"]
    #[inline(always)]
    pub fn is_noon(&self) -> bool {
        **self == TIMEVSEL_A::NOON
    }
}
impl core::ops::Deref for TIMEVSEL_R {
    type Target = crate::FieldReader<u8, TIMEVSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEVSEL` writer - Time Event Selection"]
pub struct TIMEVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn minute(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::MINUTE)
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::HOUR)
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn midnight(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::MIDNIGHT)
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn noon(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::NOON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Calendar Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CALEVSEL_A {
    #[doc = "0: Week change (every Monday at time 00:00:00)"]
    WEEK = 0,
    #[doc = "1: Month change (every 01 of each month at time 00:00:00)"]
    MONTH = 1,
    #[doc = "2: Year change (every January 1 at time 00:00:00)"]
    YEAR = 2,
}
impl From<CALEVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CALEVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CALEVSEL` reader - Calendar Event Selection"]
pub struct CALEVSEL_R(crate::FieldReader<u8, CALEVSEL_A>);
impl CALEVSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CALEVSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CALEVSEL_A> {
        match self.bits {
            0 => Some(CALEVSEL_A::WEEK),
            1 => Some(CALEVSEL_A::MONTH),
            2 => Some(CALEVSEL_A::YEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WEEK`"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        **self == CALEVSEL_A::WEEK
    }
    #[doc = "Checks if the value of the field is `MONTH`"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        **self == CALEVSEL_A::MONTH
    }
    #[doc = "Checks if the value of the field is `YEAR`"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        **self == CALEVSEL_A::YEAR
    }
}
impl core::ops::Deref for CALEVSEL_R {
    type Target = crate::FieldReader<u8, CALEVSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALEVSEL` writer - Calendar Event Selection"]
pub struct CALEVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALEVSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn week(self) -> &'a mut W {
        self.variant(CALEVSEL_A::WEEK)
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn month(self) -> &'a mut W {
        self.variant(CALEVSEL_A::MONTH)
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn year(self) -> &'a mut W {
        self.variant(CALEVSEL_A::YEAR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&self) -> UPDTIM_R {
        UPDTIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&self) -> UPDCAL_R {
        UPDCAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&self) -> TIMEVSEL_R {
        TIMEVSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&self) -> CALEVSEL_R {
        CALEVSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&mut self) -> UPDTIM_W {
        UPDTIM_W { w: self }
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&mut self) -> UPDCAL_W {
        UPDCAL_W { w: self }
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&mut self) -> TIMEVSEL_W {
        TIMEVSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&mut self) -> CALEVSEL_W {
        CALEVSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cr](index.html) module"]
pub struct RTC_CR_SPEC;
impl crate::RegisterSpec for RTC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cr::R](R) reader structure"]
impl crate::Readable for RTC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cr::W](W) writer structure"]
impl crate::Writable for RTC_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CR to value 0"]
impl crate::Resettable for RTC_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
