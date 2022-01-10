#[doc = "Register `NMICTRL` reader"]
pub struct R(crate::R<NMICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMICTRL` writer"]
pub struct W(crate::W<NMICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMICTRL_SPEC>;
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
impl From<crate::W<NMICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Non-Maskable Interrupt Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NMISENSE_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising-edge detection"]
    RISE = 1,
    #[doc = "2: Falling-edge detection"]
    FALL = 2,
    #[doc = "3: Both-edges detection"]
    BOTH = 3,
    #[doc = "4: High-level detection"]
    HIGH = 4,
    #[doc = "5: Low-level detection"]
    LOW = 5,
}
impl From<NMISENSE_A> for u8 {
    #[inline(always)]
    fn from(variant: NMISENSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NMISENSE` reader - Non-Maskable Interrupt Sense"]
pub struct NMISENSE_R(crate::FieldReader<u8, NMISENSE_A>);
impl NMISENSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NMISENSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NMISENSE_A> {
        match self.bits {
            0 => Some(NMISENSE_A::NONE),
            1 => Some(NMISENSE_A::RISE),
            2 => Some(NMISENSE_A::FALL),
            3 => Some(NMISENSE_A::BOTH),
            4 => Some(NMISENSE_A::HIGH),
            5 => Some(NMISENSE_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == NMISENSE_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == NMISENSE_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == NMISENSE_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == NMISENSE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == NMISENSE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == NMISENSE_A::LOW
    }
}
impl core::ops::Deref for NMISENSE_R {
    type Target = crate::FieldReader<u8, NMISENSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMISENSE` writer - Non-Maskable Interrupt Sense"]
pub struct NMISENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NMISENSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMISENSE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(NMISENSE_A::NONE)
    }
    #[doc = "Rising-edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(NMISENSE_A::RISE)
    }
    #[doc = "Falling-edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(NMISENSE_A::FALL)
    }
    #[doc = "Both-edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(NMISENSE_A::BOTH)
    }
    #[doc = "High-level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(NMISENSE_A::HIGH)
    }
    #[doc = "Low-level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(NMISENSE_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "Field `NMIFILTEN` reader - Non-Maskable Interrupt Filter Enable"]
pub struct NMIFILTEN_R(crate::FieldReader<bool, bool>);
impl NMIFILTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMIFILTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMIFILTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIFILTEN` writer - Non-Maskable Interrupt Filter Enable"]
pub struct NMIFILTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIFILTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense"]
    #[inline(always)]
    pub fn nmisense(&self) -> NMISENSE_R {
        NMISENSE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline(always)]
    pub fn nmifilten(&self) -> NMIFILTEN_R {
        NMIFILTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense"]
    #[inline(always)]
    pub fn nmisense(&mut self) -> NMISENSE_W {
        NMISENSE_W { w: self }
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline(always)]
    pub fn nmifilten(&mut self) -> NMIFILTEN_W {
        NMIFILTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Maskable Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmictrl](index.html) module"]
pub struct NMICTRL_SPEC;
impl crate::RegisterSpec for NMICTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nmictrl::R](R) reader structure"]
impl crate::Readable for NMICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmictrl::W](W) writer structure"]
impl crate::Writable for NMICTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMICTRL to value 0"]
impl crate::Resettable for NMICTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
