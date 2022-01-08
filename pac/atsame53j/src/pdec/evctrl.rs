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
    #[doc = "1: Start, restart or retrigger on event"]
    RETRIGGER = 1,
    #[doc = "2: Count on event"]
    COUNT = 2,
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
    #[doc = "Start, restart or retrigger on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACT_A::RETRIGGER)
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn count(self) -> &'a mut W {
        self.variant(EVACT_A::COUNT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u16 & 0x03);
        self.w
    }
}
#[doc = "Field `EVINV` reader - Inverted Event Input Enable"]
pub struct EVINV_R(crate::FieldReader<u8, u8>);
impl EVINV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVINV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVINV` writer - Inverted Event Input Enable"]
pub struct EVINV_W<'a> {
    w: &'a mut W,
}
impl<'a> EVINV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u16 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `EVEI` reader - Event Input Enable"]
pub struct EVEI_R(crate::FieldReader<u8, u8>);
impl EVEI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVEI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVEI` writer - Event Input Enable"]
pub struct EVEI_W<'a> {
    w: &'a mut W,
}
impl<'a> EVEI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u16 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `OVFEO` reader - Overflow/Underflow Output Event Enable"]
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
#[doc = "Field `OVFEO` writer - Overflow/Underflow Output Event Enable"]
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
#[doc = "Field `ERREO` reader - Error Output Event Enable"]
pub struct ERREO_R(crate::FieldReader<bool, bool>);
impl ERREO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERREO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERREO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERREO` writer - Error Output Event Enable"]
pub struct ERREO_W<'a> {
    w: &'a mut W,
}
impl<'a> ERREO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `DIREO` reader - Direction Output Event Enable"]
pub struct DIREO_R(crate::FieldReader<bool, bool>);
impl DIREO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIREO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIREO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIREO` writer - Direction Output Event Enable"]
pub struct DIREO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIREO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `VLCEO` reader - Velocity Output Event Enable"]
pub struct VLCEO_R(crate::FieldReader<bool, bool>);
impl VLCEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VLCEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLCEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLCEO` writer - Velocity Output Event Enable"]
pub struct VLCEO_W<'a> {
    w: &'a mut W,
}
impl<'a> VLCEO_W<'a> {
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
#[doc = "Field `MCEO0` reader - Match Channel 0 Event Output Enable"]
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
#[doc = "Field `MCEO0` writer - Match Channel 0 Event Output Enable"]
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
#[doc = "Field `MCEO1` reader - Match Channel 1 Event Output Enable"]
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
#[doc = "Field `MCEO1` writer - Match Channel 1 Event Output Enable"]
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
    #[doc = "Bits 0:1 - Event Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Inverted Event Input Enable"]
    #[inline(always)]
    pub fn evinv(&self) -> EVINV_R {
        EVINV_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - Event Input Enable"]
    #[inline(always)]
    pub fn evei(&self) -> EVEI_R {
        EVEI_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Error Output Event Enable"]
    #[inline(always)]
    pub fn erreo(&self) -> ERREO_R {
        ERREO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Direction Output Event Enable"]
    #[inline(always)]
    pub fn direo(&self) -> DIREO_R {
        DIREO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Velocity Output Event Enable"]
    #[inline(always)]
    pub fn vlceo(&self) -> VLCEO_R {
        VLCEO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Match Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&self) -> MCEO0_R {
        MCEO0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Match Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&self) -> MCEO1_R {
        MCEO1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Event Action"]
    #[inline(always)]
    pub fn evact(&mut self) -> EVACT_W {
        EVACT_W { w: self }
    }
    #[doc = "Bits 2:4 - Inverted Event Input Enable"]
    #[inline(always)]
    pub fn evinv(&mut self) -> EVINV_W {
        EVINV_W { w: self }
    }
    #[doc = "Bits 5:7 - Event Input Enable"]
    #[inline(always)]
    pub fn evei(&mut self) -> EVEI_W {
        EVEI_W { w: self }
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    pub fn ovfeo(&mut self) -> OVFEO_W {
        OVFEO_W { w: self }
    }
    #[doc = "Bit 9 - Error Output Event Enable"]
    #[inline(always)]
    pub fn erreo(&mut self) -> ERREO_W {
        ERREO_W { w: self }
    }
    #[doc = "Bit 10 - Direction Output Event Enable"]
    #[inline(always)]
    pub fn direo(&mut self) -> DIREO_W {
        DIREO_W { w: self }
    }
    #[doc = "Bit 11 - Velocity Output Event Enable"]
    #[inline(always)]
    pub fn vlceo(&mut self) -> VLCEO_W {
        VLCEO_W { w: self }
    }
    #[doc = "Bit 12 - Match Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&mut self) -> MCEO0_W {
        MCEO0_W { w: self }
    }
    #[doc = "Bit 13 - Match Channel 1 Event Output Enable"]
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
