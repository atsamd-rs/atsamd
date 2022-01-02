#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVACT_A {
    #[doc = "0: Event action disabled"]
    OFF = 0,
    #[doc = "1: Start, restart or retrigger TC on event"]
    RETRIGGER = 1,
    #[doc = "2: Count on event"]
    COUNT = 2,
    #[doc = "3: Start TC on event"]
    START = 3,
    #[doc = "5: Period captured in CC0, pulse width in CC1"]
    PPW = 5,
    #[doc = "6: Period captured in CC1, pulse width in CC0"]
    PWP = 6,
}
impl From<EVACT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EVACT` reader - Event Action"]
pub struct EVACT_R(crate::FieldReader<u8, EVACT_A>);
impl EVACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EVACT_A> {
        match self.bits {
            0 => Some(EVACT_A::OFF),
            1 => Some(EVACT_A::RETRIGGER),
            2 => Some(EVACT_A::COUNT),
            3 => Some(EVACT_A::START),
            5 => Some(EVACT_A::PPW),
            6 => Some(EVACT_A::PWP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == EVACT_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        **self == EVACT_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `COUNT`"]
    #[inline(always)]
    pub fn is_count(&self) -> bool {
        **self == EVACT_A::COUNT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == EVACT_A::START
    }
    #[doc = "Checks if the value of the field is `PPW`"]
    #[inline(always)]
    pub fn is_ppw(&self) -> bool {
        **self == EVACT_A::PPW
    }
    #[doc = "Checks if the value of the field is `PWP`"]
    #[inline(always)]
    pub fn is_pwp(&self) -> bool {
        **self == EVACT_A::PWP
    }
}
impl core::ops::Deref for EVACT_R {
    type Target = crate::FieldReader<u8, EVACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVACT` writer - Event Action"]
pub struct EVACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVACT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EVACT_A::OFF)
    }
    #[doc = "Start, restart or retrigger TC on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACT_A::RETRIGGER)
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn count(self) -> &'a mut W {
        self.variant(EVACT_A::COUNT)
    }
    #[doc = "Start TC on event"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(EVACT_A::START)
    }
    #[doc = "Period captured in CC0, pulse width in CC1"]
    #[inline(always)]
    pub fn ppw(self) -> &'a mut W {
        self.variant(EVACT_A::PPW)
    }
    #[doc = "Period captured in CC1, pulse width in CC0"]
    #[inline(always)]
    pub fn pwp(self) -> &'a mut W {
        self.variant(EVACT_A::PWP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
#[doc = "Field `TCINV` reader - TC Inverted Event Input"]
pub struct TCINV_R(crate::FieldReader<bool, bool>);
impl TCINV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCINV` writer - TC Inverted Event Input"]
pub struct TCINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TCINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TCEI` reader - TC Event Input"]
pub struct TCEI_R(crate::FieldReader<bool, bool>);
impl TCEI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCEI` writer - TC Event Input"]
pub struct TCEI_W<'a> {
    w: &'a mut W,
}
impl<'a> TCEI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `OVFEO` reader - Overflow/Underflow Event Output Enable"]
pub struct OVFEO_R(crate::FieldReader<bool, bool>);
impl OVFEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVFEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFEO` writer - Overflow/Underflow Event Output Enable"]
pub struct OVFEO_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `MCEO0` reader - Match or Capture Channel 0 Event Output Enable"]
pub struct MCEO0_R(crate::FieldReader<bool, bool>);
impl MCEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEO0` writer - Match or Capture Channel 0 Event Output Enable"]
pub struct MCEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `MCEO1` reader - Match or Capture Channel 1 Event Output Enable"]
pub struct MCEO1_R(crate::FieldReader<bool, bool>);
impl MCEO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEO1` writer - Match or Capture Channel 1 Event Output Enable"]
pub struct MCEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Event Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - TC Inverted Event Input"]
    #[inline(always)]
    pub fn tcinv(&self) -> TCINV_R {
        TCINV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TC Event Input"]
    #[inline(always)]
    pub fn tcei(&self) -> TCEI_R {
        TCEI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Overflow/Underflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&self) -> MCEO0_R {
        MCEO0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&self) -> MCEO1_R {
        MCEO1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Action"]
    #[inline(always)]
    pub fn evact(&mut self) -> EVACT_W {
        EVACT_W { w: self }
    }
    #[doc = "Bit 4 - TC Inverted Event Input"]
    #[inline(always)]
    pub fn tcinv(&mut self) -> TCINV_W {
        TCINV_W { w: self }
    }
    #[doc = "Bit 5 - TC Event Input"]
    #[inline(always)]
    pub fn tcei(&mut self) -> TCEI_W {
        TCEI_W { w: self }
    }
    #[doc = "Bit 8 - Overflow/Underflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&mut self) -> OVFEO_W {
        OVFEO_W { w: self }
    }
    #[doc = "Bit 12 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&mut self) -> MCEO0_W {
        MCEO0_W { w: self }
    }
    #[doc = "Bit 13 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&mut self) -> MCEO1_W {
        MCEO1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
