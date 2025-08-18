#[doc = "Register `OSC16MCTRL` reader"]
pub struct R(crate::R<OSC16MCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC16MCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC16MCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC16MCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC16MCTRL` writer"]
pub struct W(crate::W<OSC16MCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC16MCTRL_SPEC>;
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
impl From<crate::W<OSC16MCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC16MCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSC16MCTRL_SPEC, bool, O>;
#[doc = "Field `FSEL` reader - Oscillator Frequency Select"]
pub type FSEL_R = crate::FieldReader<u8, FSELSELECT_A>;
#[doc = "Oscillator Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSELSELECT_A {
    #[doc = "0: 4MHz"]
    _4 = 0,
    #[doc = "1: 8MHz"]
    _8 = 1,
    #[doc = "2: 12MHz"]
    _12 = 2,
    #[doc = "3: 16MHz"]
    _16 = 3,
}
impl From<FSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FSELSELECT_A) -> Self {
        variant as _
    }
}
impl FSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSELSELECT_A {
        match self.bits {
            0 => FSELSELECT_A::_4,
            1 => FSELSELECT_A::_8,
            2 => FSELSELECT_A::_12,
            3 => FSELSELECT_A::_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == FSELSELECT_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == FSELSELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == FSELSELECT_A::_12
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == FSELSELECT_A::_16
    }
}
#[doc = "Field `FSEL` writer - Oscillator Frequency Select"]
pub type FSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, OSC16MCTRL_SPEC, u8, FSELSELECT_A, 2, O>;
impl<'a, const O: u8> FSEL_W<'a, O> {
    #[doc = "4MHz"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FSELSELECT_A::_4)
    }
    #[doc = "8MHz"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(FSELSELECT_A::_8)
    }
    #[doc = "12MHz"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(FSELSELECT_A::_12)
    }
    #[doc = "16MHz"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(FSELSELECT_A::_16)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSC16MCTRL_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSC16MCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Oscillator Frequency Select"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Oscillator Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<2> {
        FSEL_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "16MHz Internal Oscillator (OSC16M) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc16mctrl](index.html) module"]
pub struct OSC16MCTRL_SPEC;
impl crate::RegisterSpec for OSC16MCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc16mctrl::R](R) reader structure"]
impl crate::Readable for OSC16MCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc16mctrl::W](W) writer structure"]
impl crate::Writable for OSC16MCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC16MCTRL to value 0x82"]
impl crate::Resettable for OSC16MCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x82;
}
