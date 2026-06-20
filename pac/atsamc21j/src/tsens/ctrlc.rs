#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WINMODE_A {
    #[doc = "0: No window mode (default)"]
    DISABLE = 0,
    #[doc = "1: VALUE greater than WINLT"]
    ABOVE = 1,
    #[doc = "2: VALUE less than WINUT"]
    BELOW = 2,
    #[doc = "3: VALUE greater than WINLT and VALUE less than WINUT"]
    INSIDE = 3,
    #[doc = "4: VALUE less than WINLT or VALUE greater than WINUT"]
    OUTSIDE = 4,
    #[doc = "5: VALUE greater than WINUT with hysteresis to WINLT"]
    HYST_ABOVE = 5,
    #[doc = "6: VALUE less than WINLST with hysteresis to WINUT"]
    HYST_BELOW = 6,
}
impl From<WINMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WINMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WINMODE` reader - Window Monitor Mode"]
pub struct WINMODE_R(crate::FieldReader<u8, WINMODE_A>);
impl WINMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WINMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINMODE_A> {
        match self.bits {
            0 => Some(WINMODE_A::DISABLE),
            1 => Some(WINMODE_A::ABOVE),
            2 => Some(WINMODE_A::BELOW),
            3 => Some(WINMODE_A::INSIDE),
            4 => Some(WINMODE_A::OUTSIDE),
            5 => Some(WINMODE_A::HYST_ABOVE),
            6 => Some(WINMODE_A::HYST_BELOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WINMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        **self == WINMODE_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        **self == WINMODE_A::BELOW
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        **self == WINMODE_A::INSIDE
    }
    #[doc = "Checks if the value of the field is `OUTSIDE`"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        **self == WINMODE_A::OUTSIDE
    }
    #[doc = "Checks if the value of the field is `HYST_ABOVE`"]
    #[inline(always)]
    pub fn is_hyst_above(&self) -> bool {
        **self == WINMODE_A::HYST_ABOVE
    }
    #[doc = "Checks if the value of the field is `HYST_BELOW`"]
    #[inline(always)]
    pub fn is_hyst_below(&self) -> bool {
        **self == WINMODE_A::HYST_BELOW
    }
}
impl core::ops::Deref for WINMODE_R {
    type Target = crate::FieldReader<u8, WINMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINMODE` writer - Window Monitor Mode"]
pub struct WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WINMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WINMODE_A::DISABLE)
    }
    #[doc = "VALUE greater than WINLT"]
    #[inline(always)]
    pub fn above(self) -> &'a mut W {
        self.variant(WINMODE_A::ABOVE)
    }
    #[doc = "VALUE less than WINUT"]
    #[inline(always)]
    pub fn below(self) -> &'a mut W {
        self.variant(WINMODE_A::BELOW)
    }
    #[doc = "VALUE greater than WINLT and VALUE less than WINUT"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut W {
        self.variant(WINMODE_A::INSIDE)
    }
    #[doc = "VALUE less than WINLT or VALUE greater than WINUT"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut W {
        self.variant(WINMODE_A::OUTSIDE)
    }
    #[doc = "VALUE greater than WINUT with hysteresis to WINLT"]
    #[inline(always)]
    pub fn hyst_above(self) -> &'a mut W {
        self.variant(WINMODE_A::HYST_ABOVE)
    }
    #[doc = "VALUE less than WINLST with hysteresis to WINUT"]
    #[inline(always)]
    pub fn hyst_below(self) -> &'a mut W {
        self.variant(WINMODE_A::HYST_BELOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "Field `FREERUN` reader - Free Running Measurement"]
pub struct FREERUN_R(crate::FieldReader<bool, bool>);
impl FREERUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREERUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREERUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREERUN` writer - Free Running Measurement"]
pub struct FREERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FREERUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Free Running Measurement"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W {
        WINMODE_W { w: self }
    }
    #[doc = "Bit 4 - Free Running Measurement"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FREERUN_W {
        FREERUN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
