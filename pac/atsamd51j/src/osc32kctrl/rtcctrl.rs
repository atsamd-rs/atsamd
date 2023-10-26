#[doc = "Register `RTCCTRL` reader"]
pub type R = crate::R<RTCCTRL_SPEC>;
#[doc = "Register `RTCCTRL` writer"]
pub type W = crate::W<RTCCTRL_SPEC>;
#[doc = "Field `RTCSEL` reader - RTC Clock Selection"]
pub type RTCSEL_R = crate::FieldReader<RTCSELSELECT_A>;
#[doc = "RTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSELSELECT_A {
    #[doc = "0: 1.024kHz from 32kHz internal ULP oscillator"]
    ULP1K = 0,
    #[doc = "1: 32.768kHz from 32kHz internal ULP oscillator"]
    ULP32K = 1,
    #[doc = "4: 1.024kHz from 32.768kHz internal oscillator"]
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
impl crate::FieldSpec for RTCSELSELECT_A {
    type Ux = u8;
}
impl RTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTCSELSELECT_A> {
        match self.bits {
            0 => Some(RTCSELSELECT_A::ULP1K),
            1 => Some(RTCSELSELECT_A::ULP32K),
            4 => Some(RTCSELSELECT_A::XOSC1K),
            5 => Some(RTCSELSELECT_A::XOSC32K),
            _ => None,
        }
    }
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn is_ulp1k(&self) -> bool {
        *self == RTCSELSELECT_A::ULP1K
    }
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn is_ulp32k(&self) -> bool {
        *self == RTCSELSELECT_A::ULP32K
    }
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn is_xosc1k(&self) -> bool {
        *self == RTCSELSELECT_A::XOSC1K
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == RTCSELSELECT_A::XOSC32K
    }
}
#[doc = "Field `RTCSEL` writer - RTC Clock Selection"]
pub type RTCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, RTCSELSELECT_A>;
impl<'a, REG, const O: u8> RTCSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp1k(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSELSELECT_A::ULP1K)
    }
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp32k(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSELSELECT_A::ULP32K)
    }
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn xosc1k(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSELSELECT_A::XOSC1K)
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
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
    pub fn rtcsel(&mut self) -> RTCSEL_W<RTCCTRL_SPEC, 0> {
        RTCSEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC Clock Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCTRL_SPEC;
impl crate::RegisterSpec for RTCCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcctrl::R`](R) reader structure"]
impl crate::Readable for RTCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcctrl::W`](W) writer structure"]
impl crate::Writable for RTCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCTRL to value 0"]
impl crate::Resettable for RTCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
