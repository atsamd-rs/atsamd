#[doc = "Register `XOSC` reader"]
pub struct R(crate::R<XOSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSC` writer"]
pub struct W(crate::W<XOSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSC_SPEC>;
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
impl From<crate::W<XOSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub struct XTALEN_R(crate::FieldReader<bool, bool>);
impl XTALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub struct XTALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub struct RUNSTDBY_R(crate::FieldReader<bool, bool>);
impl RUNSTDBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNSTDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNSTDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub struct ONDEMAND_R(crate::FieldReader<bool, bool>);
impl ONDEMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONDEMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONDEMAND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub struct ONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEMAND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Oscillator Gain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN_A {
    #[doc = "0: 2MHz"]
    _0 = 0,
    #[doc = "1: 4MHz"]
    _1 = 1,
    #[doc = "2: 8MHz"]
    _2 = 2,
    #[doc = "3: 16MHz"]
    _3 = 3,
    #[doc = "4: 30MHz"]
    _4 = 4,
}
impl From<GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GAIN` reader - Oscillator Gain"]
pub struct GAIN_R(crate::FieldReader<u8, GAIN_A>);
impl GAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GAIN_A> {
        match self.bits {
            0 => Some(GAIN_A::_0),
            1 => Some(GAIN_A::_1),
            2 => Some(GAIN_A::_2),
            3 => Some(GAIN_A::_3),
            4 => Some(GAIN_A::_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GAIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GAIN_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == GAIN_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == GAIN_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == GAIN_A::_4
    }
}
impl core::ops::Deref for GAIN_R {
    type Target = crate::FieldReader<u8, GAIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN` writer - Oscillator Gain"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "2MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAIN_A::_0)
    }
    #[doc = "4MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAIN_A::_1)
    }
    #[doc = "8MHz"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(GAIN_A::_2)
    }
    #[doc = "16MHz"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(GAIN_A::_3)
    }
    #[doc = "30MHz"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(GAIN_A::_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u16 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `AMPGC` reader - Automatic Amplitude Gain Control"]
pub struct AMPGC_R(crate::FieldReader<bool, bool>);
impl AMPGC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMPGC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMPGC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMPGC` writer - Automatic Amplitude Gain Control"]
pub struct AMPGC_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPGC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub struct STARTUP_R(crate::FieldReader<u8, u8>);
impl STARTUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STARTUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u16 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&self) -> AMPGC_R {
        AMPGC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&mut self) -> XTALEN_W {
        XTALEN_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&mut self) -> AMPGC_W {
        AMPGC_W { w: self }
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc](index.html) module"]
pub struct XOSC_SPEC;
impl crate::RegisterSpec for XOSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xosc::R](R) reader structure"]
impl crate::Readable for XOSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xosc::W](W) writer structure"]
impl crate::Writable for XOSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XOSC to value 0x80"]
impl crate::Resettable for XOSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
