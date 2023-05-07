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
#[doc = "Field `EVACT` reader - Event Action"]
pub type EVACT_R = crate::FieldReader<u8, EVACTSELECT_A>;
#[doc = "Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACTSELECT_A {
    #[doc = "0: Event action disabled"]
    OFF = 0,
    #[doc = "1: Start, restart or retrigger TC on event"]
    RETRIGGER = 1,
    #[doc = "2: Count on event"]
    COUNT = 2,
    #[doc = "3: Start TC on event"]
    START = 3,
    #[doc = "4: Time stamp capture"]
    STAMP = 4,
    #[doc = "5: Period catured in CC0, pulse width in CC1"]
    PPW = 5,
    #[doc = "6: Period catured in CC1, pulse width in CC0"]
    PWP = 6,
    #[doc = "7: Pulse width capture"]
    PW = 7,
}
impl From<EVACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACTSELECT_A) -> Self {
        variant as _
    }
}
impl EVACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVACTSELECT_A {
        match self.bits {
            0 => EVACTSELECT_A::OFF,
            1 => EVACTSELECT_A::RETRIGGER,
            2 => EVACTSELECT_A::COUNT,
            3 => EVACTSELECT_A::START,
            4 => EVACTSELECT_A::STAMP,
            5 => EVACTSELECT_A::PPW,
            6 => EVACTSELECT_A::PWP,
            7 => EVACTSELECT_A::PW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EVACTSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACTSELECT_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `COUNT`"]
    #[inline(always)]
    pub fn is_count(&self) -> bool {
        *self == EVACTSELECT_A::COUNT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == EVACTSELECT_A::START
    }
    #[doc = "Checks if the value of the field is `STAMP`"]
    #[inline(always)]
    pub fn is_stamp(&self) -> bool {
        *self == EVACTSELECT_A::STAMP
    }
    #[doc = "Checks if the value of the field is `PPW`"]
    #[inline(always)]
    pub fn is_ppw(&self) -> bool {
        *self == EVACTSELECT_A::PPW
    }
    #[doc = "Checks if the value of the field is `PWP`"]
    #[inline(always)]
    pub fn is_pwp(&self) -> bool {
        *self == EVACTSELECT_A::PWP
    }
    #[doc = "Checks if the value of the field is `PW`"]
    #[inline(always)]
    pub fn is_pw(&self) -> bool {
        *self == EVACTSELECT_A::PW
    }
}
#[doc = "Field `EVACT` writer - Event Action"]
pub type EVACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, EVCTRL_SPEC, u8, EVACTSELECT_A, 3, O>;
impl<'a, const O: u8> EVACT_W<'a, O> {
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EVACTSELECT_A::OFF)
    }
    #[doc = "Start, restart or retrigger TC on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACTSELECT_A::RETRIGGER)
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn count(self) -> &'a mut W {
        self.variant(EVACTSELECT_A::COUNT)
    }
    #[doc = "Start TC on event"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(EVACTSELECT_A::START)
    }
    #[doc = "Time stamp capture"]
    #[inline(always)]
    pub fn stamp(self) -> &'a mut W {
        self.variant(EVACTSELECT_A::STAMP)
    }
    #[doc = "Period catured in CC0, pulse width in CC1"]
    #[inline(always)]
    pub fn ppw(self) -> &'a mut W {
        self.variant(EVACTSELECT_A::PPW)
    }
    #[doc = "Period catured in CC1, pulse width in CC0"]
    #[inline(always)]
    pub fn pwp(self) -> &'a mut W {
        self.variant(EVACTSELECT_A::PWP)
    }
    #[doc = "Pulse width capture"]
    #[inline(always)]
    pub fn pw(self) -> &'a mut W {
        self.variant(EVACTSELECT_A::PW)
    }
}
#[doc = "Field `TCINV` reader - TC Event Input Polarity"]
pub type TCINV_R = crate::BitReader<bool>;
#[doc = "Field `TCINV` writer - TC Event Input Polarity"]
pub type TCINV_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `TCEI` reader - TC Event Enable"]
pub type TCEI_R = crate::BitReader<bool>;
#[doc = "Field `TCEI` writer - TC Event Enable"]
pub type TCEI_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `OVFEO` reader - Event Output Enable"]
pub type OVFEO_R = crate::BitReader<bool>;
#[doc = "Field `OVFEO` writer - Event Output Enable"]
pub type OVFEO_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEO0` reader - MC Event Output Enable 0"]
pub type MCEO0_R = crate::BitReader<bool>;
#[doc = "Field `MCEO0` writer - MC Event Output Enable 0"]
pub type MCEO0_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEO1` reader - MC Event Output Enable 1"]
pub type MCEO1_R = crate::BitReader<bool>;
#[doc = "Field `MCEO1` writer - MC Event Output Enable 1"]
pub type MCEO1_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Event Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - TC Event Input Polarity"]
    #[inline(always)]
    pub fn tcinv(&self) -> TCINV_R {
        TCINV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC Event Enable"]
    #[inline(always)]
    pub fn tcei(&self) -> TCEI_R {
        TCEI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - MC Event Output Enable 0"]
    #[inline(always)]
    pub fn mceo0(&self) -> MCEO0_R {
        MCEO0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MC Event Output Enable 1"]
    #[inline(always)]
    pub fn mceo1(&self) -> MCEO1_R {
        MCEO1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EVACT_W<0> {
        EVACT_W::new(self)
    }
    #[doc = "Bit 4 - TC Event Input Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tcinv(&mut self) -> TCINV_W<4> {
        TCINV_W::new(self)
    }
    #[doc = "Bit 5 - TC Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcei(&mut self) -> TCEI_W<5> {
        TCEI_W::new(self)
    }
    #[doc = "Bit 8 - Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfeo(&mut self) -> OVFEO_W<8> {
        OVFEO_W::new(self)
    }
    #[doc = "Bit 12 - MC Event Output Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn mceo0(&mut self) -> MCEO0_W<12> {
        MCEO0_W::new(self)
    }
    #[doc = "Bit 13 - MC Event Output Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn mceo1(&mut self) -> MCEO1_W<13> {
        MCEO1_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
