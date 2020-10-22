#[doc = "Reader of register RTCCTRL"]
pub type R = crate::R<u8, super::RTCCTRL>;
#[doc = "Writer for register RTCCTRL"]
pub type W = crate::W<u8, super::RTCCTRL>;
#[doc = "Register RTCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSEL_A {
    #[doc = "0: 1.024kHz from 32kHz internal ULP oscillator"]
    ULP1K = 0,
    #[doc = "1: 32.768kHz from 32kHz internal ULP oscillator"]
    ULP32K = 1,
    #[doc = "4: 1.024kHz from 32.768kHz internal oscillator"]
    XOSC1K = 4,
    #[doc = "5: 32.768kHz from 32.768kHz external crystal oscillator"]
    XOSC32K = 5,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCSEL`"]
pub type RTCSEL_R = crate::R<u8, RTCSEL_A>;
impl RTCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RTCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RTCSEL_A::ULP1K),
            1 => Val(RTCSEL_A::ULP32K),
            4 => Val(RTCSEL_A::XOSC1K),
            5 => Val(RTCSEL_A::XOSC32K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ULP1K`"]
    #[inline(always)]
    pub fn is_ulp1k(&self) -> bool {
        *self == RTCSEL_A::ULP1K
    }
    #[doc = "Checks if the value of the field is `ULP32K`"]
    #[inline(always)]
    pub fn is_ulp32k(&self) -> bool {
        *self == RTCSEL_A::ULP32K
    }
    #[doc = "Checks if the value of the field is `XOSC1K`"]
    #[inline(always)]
    pub fn is_xosc1k(&self) -> bool {
        *self == RTCSEL_A::XOSC1K
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == RTCSEL_A::XOSC32K
    }
}
#[doc = "Write proxy for field `RTCSEL`"]
pub struct RTCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp1k(self) -> &'a mut W {
        self.variant(RTCSEL_A::ULP1K)
    }
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp32k(self) -> &'a mut W {
        self.variant(RTCSEL_A::ULP32K)
    }
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn xosc1k(self) -> &'a mut W {
        self.variant(RTCSEL_A::XOSC1K)
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(RTCSEL_A::XOSC32K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - RTC Clock Selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RTC Clock Selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W {
        RTCSEL_W { w: self }
    }
}
