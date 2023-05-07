#[doc = "Register `OSC8M` reader"]
pub struct R(crate::R<OSC8M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC8M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC8M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC8M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC8M` writer"]
pub struct W(crate::W<OSC8M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC8M_SPEC>;
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
impl From<crate::W<OSC8M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC8M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC8M_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC8M_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC8M_SPEC, bool, O>;
#[doc = "Field `PRESC` reader - Oscillator Prescaler"]
pub type PRESC_R = crate::FieldReader<u8, PRESCSELECT_A>;
#[doc = "Oscillator Prescaler\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCSELECT_A {
    #[doc = "0: 1"]
    _0 = 0,
    #[doc = "1: 2"]
    _1 = 1,
    #[doc = "2: 4"]
    _2 = 2,
    #[doc = "3: 8"]
    _3 = 3,
}
impl From<PRESCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCSELECT_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCSELECT_A {
        match self.bits {
            0 => PRESCSELECT_A::_0,
            1 => PRESCSELECT_A::_1,
            2 => PRESCSELECT_A::_2,
            3 => PRESCSELECT_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRESCSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRESCSELECT_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PRESCSELECT_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == PRESCSELECT_A::_3
    }
}
#[doc = "Field `PRESC` writer - Oscillator Prescaler"]
pub type PRESC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OSC8M_SPEC, u8, PRESCSELECT_A, 2, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::_0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::_1)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::_2)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::_3)
    }
}
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub type CALIB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSC8M_SPEC, u16, u16, 12, O>;
#[doc = "Field `FRANGE` reader - Oscillator Frequency Range"]
pub type FRANGE_R = crate::FieldReader<u8, FRANGESELECT_A>;
#[doc = "Oscillator Frequency Range\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRANGESELECT_A {
    #[doc = "0: 4 to 6MHz"]
    _0 = 0,
    #[doc = "1: 6 to 8MHz"]
    _1 = 1,
    #[doc = "2: 8 to 11MHz"]
    _2 = 2,
    #[doc = "3: 11 to 15MHz"]
    _3 = 3,
}
impl From<FRANGESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FRANGESELECT_A) -> Self {
        variant as _
    }
}
impl FRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRANGESELECT_A {
        match self.bits {
            0 => FRANGESELECT_A::_0,
            1 => FRANGESELECT_A::_1,
            2 => FRANGESELECT_A::_2,
            3 => FRANGESELECT_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRANGESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRANGESELECT_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == FRANGESELECT_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == FRANGESELECT_A::_3
    }
}
#[doc = "Field `FRANGE` writer - Oscillator Frequency Range"]
pub type FRANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OSC8M_SPEC, u8, FRANGESELECT_A, 2, O>;
impl<'a, const O: u8> FRANGE_W<'a, O> {
    #[doc = "4 to 6MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRANGESELECT_A::_0)
    }
    #[doc = "6 to 8MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRANGESELECT_A::_1)
    }
    #[doc = "8 to 11MHz"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FRANGESELECT_A::_2)
    }
    #[doc = "11 to 15MHz"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(FRANGESELECT_A::_3)
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bits 8:9 - Oscillator Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:27 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - Oscillator Frequency Range"]
    #[inline(always)]
    pub fn frange(&self) -> FRANGE_R {
        FRANGE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
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
    #[doc = "Bits 8:9 - Oscillator Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<8> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 16:27 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<16> {
        CALIB_W::new(self)
    }
    #[doc = "Bits 30:31 - Oscillator Frequency Range"]
    #[inline(always)]
    #[must_use]
    pub fn frange(&mut self) -> FRANGE_W<30> {
        FRANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "8MHz Internal Oscillator (OSC8M) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc8m](index.html) module"]
pub struct OSC8M_SPEC;
impl crate::RegisterSpec for OSC8M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc8m::R](R) reader structure"]
impl crate::Readable for OSC8M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc8m::W](W) writer structure"]
impl crate::Writable for OSC8M_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC8M to value 0x8707_0382"]
impl crate::Resettable for OSC8M_SPEC {
    const RESET_VALUE: Self::Ux = 0x8707_0382;
}
