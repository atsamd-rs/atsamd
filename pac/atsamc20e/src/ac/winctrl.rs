#[doc = "Register `WINCTRL` reader"]
pub struct R(crate::R<WINCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINCTRL` writer"]
pub struct W(crate::W<WINCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINCTRL_SPEC>;
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
impl From<crate::W<WINCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEN0` reader - Window 0 Mode Enable"]
pub struct WEN0_R(crate::FieldReader<bool, bool>);
impl WEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEN0` writer - Window 0 Mode Enable"]
pub struct WEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> WEN0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Window 0 Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WINTSEL0_A {
    #[doc = "0: Interrupt on signal above window"]
    ABOVE = 0,
    #[doc = "1: Interrupt on signal inside window"]
    INSIDE = 1,
    #[doc = "2: Interrupt on signal below window"]
    BELOW = 2,
    #[doc = "3: Interrupt on signal outside window"]
    OUTSIDE = 3,
}
impl From<WINTSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: WINTSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WINTSEL0` reader - Window 0 Interrupt Selection"]
pub struct WINTSEL0_R(crate::FieldReader<u8, WINTSEL0_A>);
impl WINTSEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WINTSEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WINTSEL0_A {
        match self.bits {
            0 => WINTSEL0_A::ABOVE,
            1 => WINTSEL0_A::INSIDE,
            2 => WINTSEL0_A::BELOW,
            3 => WINTSEL0_A::OUTSIDE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        **self == WINTSEL0_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        **self == WINTSEL0_A::INSIDE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        **self == WINTSEL0_A::BELOW
    }
    #[doc = "Checks if the value of the field is `OUTSIDE`"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        **self == WINTSEL0_A::OUTSIDE
    }
}
impl core::ops::Deref for WINTSEL0_R {
    type Target = crate::FieldReader<u8, WINTSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINTSEL0` writer - Window 0 Interrupt Selection"]
pub struct WINTSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> WINTSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WINTSEL0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Interrupt on signal above window"]
    #[inline(always)]
    pub fn above(self) -> &'a mut W {
        self.variant(WINTSEL0_A::ABOVE)
    }
    #[doc = "Interrupt on signal inside window"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut W {
        self.variant(WINTSEL0_A::INSIDE)
    }
    #[doc = "Interrupt on signal below window"]
    #[inline(always)]
    pub fn below(self) -> &'a mut W {
        self.variant(WINTSEL0_A::BELOW)
    }
    #[doc = "Interrupt on signal outside window"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut W {
        self.variant(WINTSEL0_A::OUTSIDE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u8 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline(always)]
    pub fn wen0(&self) -> WEN0_R {
        WEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline(always)]
    pub fn wintsel0(&self) -> WINTSEL0_R {
        WINTSEL0_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline(always)]
    pub fn wen0(&mut self) -> WEN0_W {
        WEN0_W { w: self }
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline(always)]
    pub fn wintsel0(&mut self) -> WINTSEL0_W {
        WINTSEL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winctrl](index.html) module"]
pub struct WINCTRL_SPEC;
impl crate::RegisterSpec for WINCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [winctrl::R](R) reader structure"]
impl crate::Readable for WINCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winctrl::W](W) writer structure"]
impl crate::Writable for WINCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WINCTRL to value 0"]
impl crate::Resettable for WINCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
