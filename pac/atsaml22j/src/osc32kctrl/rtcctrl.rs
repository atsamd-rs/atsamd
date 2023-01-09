#[doc = "Register `RTCCTRL` reader"]
pub struct R(crate::R<RTCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTRL` writer"]
pub struct W(crate::W<RTCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTRL_SPEC>;
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
impl From<crate::W<RTCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCSEL` reader - RTC Clock Selection"]
pub type RTCSEL_R = crate::FieldReader<u8, RTCSELSELECT_A>;
#[doc = "RTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSELSELECT_A {
    #[doc = "0: 1.024kHz from 32kHz internal ULP oscillator"]
    ULP1K = 0,
    #[doc = "1: 32.768kHz from 32kHz internal ULP oscillator"]
    ULP32K = 1,
    #[doc = "4: 1.024kHz from 32.768kHz external oscillator"]
    XOSC1K = 4,
    #[doc = "5: 32.768kHz from 32.768kHz external crystal oscillator"]
    XOSC32K = 5,
}
impl From<RTCSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSELSELECT_A) -> Self {
        variant as _
    }
}
impl RTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCSELSELECT_A> {
        match self.bits {
            0 => Some(RTCSELSELECT_A::ULP1K),
            1 => Some(RTCSELSELECT_A::ULP32K),
            4 => Some(RTCSELSELECT_A::XOSC1K),
            5 => Some(RTCSELSELECT_A::XOSC32K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ULP1K`"]
    #[inline(always)]
    pub fn is_ulp1k(&self) -> bool {
        *self == RTCSELSELECT_A::ULP1K
    }
    #[doc = "Checks if the value of the field is `ULP32K`"]
    #[inline(always)]
    pub fn is_ulp32k(&self) -> bool {
        *self == RTCSELSELECT_A::ULP32K
    }
    #[doc = "Checks if the value of the field is `XOSC1K`"]
    #[inline(always)]
    pub fn is_xosc1k(&self) -> bool {
        *self == RTCSELSELECT_A::XOSC1K
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == RTCSELSELECT_A::XOSC32K
    }
}
#[doc = "Field `RTCSEL` writer - RTC Clock Selection"]
pub type RTCSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, RTCCTRL_SPEC, u8, RTCSELSELECT_A, 3, O>;
impl<'a, const O: u8> RTCSEL_W<'a, O> {
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp1k(self) -> &'a mut W {
        self.variant(RTCSELSELECT_A::ULP1K)
    }
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp32k(self) -> &'a mut W {
        self.variant(RTCSELSELECT_A::ULP32K)
    }
    #[doc = "1.024kHz from 32.768kHz external oscillator"]
    #[inline(always)]
    pub fn xosc1k(self) -> &'a mut W {
        self.variant(RTCSELSELECT_A::XOSC1K)
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(RTCSELSELECT_A::XOSC32K)
    }
}
impl R {
    #[doc = "Bits 0:2 - RTC Clock Selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - RTC Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<0> {
        RTCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Clock Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctrl](index.html) module"]
pub struct RTCCTRL_SPEC;
impl crate::RegisterSpec for RTCCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcctrl::R](R) reader structure"]
impl crate::Readable for RTCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctrl::W](W) writer structure"]
impl crate::Writable for RTCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCTRL to value 0"]
impl crate::Resettable for RTCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
