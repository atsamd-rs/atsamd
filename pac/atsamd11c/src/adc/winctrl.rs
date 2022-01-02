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
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WINMODE_A {
    #[doc = "0: No window mode (default)"]
    DISABLE = 0,
    #[doc = "1: Mode 1: RESULT > WINLT"]
    MODE1 = 1,
    #[doc = "2: Mode 2: RESULT < WINUT"]
    MODE2 = 2,
    #[doc = "3: Mode 3: WINLT < RESULT < WINUT"]
    MODE3 = 3,
    #[doc = "4: Mode 4: !(WINLT < RESULT < WINUT)"]
    MODE4 = 4,
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
            1 => Some(WINMODE_A::MODE1),
            2 => Some(WINMODE_A::MODE2),
            3 => Some(WINMODE_A::MODE3),
            4 => Some(WINMODE_A::MODE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WINMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        **self == WINMODE_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        **self == WINMODE_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        **self == WINMODE_A::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        **self == WINMODE_A::MODE4
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
    #[doc = "Mode 1: RESULT > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE1)
    }
    #[doc = "Mode 2: RESULT < WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE2)
    }
    #[doc = "Mode 3: WINLT < RESULT < WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE3)
    }
    #[doc = "Mode 4: !(WINLT < RESULT < WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W {
        WINMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Monitor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winctrl](index.html) module"]
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
