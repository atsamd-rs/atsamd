#[doc = "Register `PCR` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDBPWR` reader - SD Bus Power"]
pub type SDBPWR_R = crate::BitReader<SDBPWRSELECT_A>;
#[doc = "SD Bus Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDBPWRSELECT_A {
    #[doc = "0: Power off"]
    OFF = 0,
    #[doc = "1: Power on"]
    ON = 1,
}
impl From<SDBPWRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SDBPWRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SDBPWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDBPWRSELECT_A {
        match self.bits {
            false => SDBPWRSELECT_A::OFF,
            true => SDBPWRSELECT_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SDBPWRSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == SDBPWRSELECT_A::ON
    }
}
#[doc = "Field `SDBPWR` writer - SD Bus Power"]
pub type SDBPWR_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCR_SPEC, SDBPWRSELECT_A, O>;
impl<'a, const O: u8> SDBPWR_W<'a, O> {
    #[doc = "Power off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SDBPWRSELECT_A::OFF)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SDBPWRSELECT_A::ON)
    }
}
#[doc = "Field `SDBVSEL` reader - SD Bus Voltage Select"]
pub type SDBVSEL_R = crate::FieldReader<u8, SDBVSELSELECT_A>;
#[doc = "SD Bus Voltage Select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDBVSELSELECT_A {
    #[doc = "5: 1.8V (Typ.)"]
    _1V8 = 5,
    #[doc = "6: 3.0V (Typ.)"]
    _3V0 = 6,
    #[doc = "7: 3.3V (Typ.)"]
    _3V3 = 7,
}
impl From<SDBVSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SDBVSELSELECT_A) -> Self {
        variant as _
    }
}
impl SDBVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDBVSELSELECT_A> {
        match self.bits {
            5 => Some(SDBVSELSELECT_A::_1V8),
            6 => Some(SDBVSELSELECT_A::_3V0),
            7 => Some(SDBVSELSELECT_A::_3V3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == SDBVSELSELECT_A::_1V8
    }
    #[doc = "Checks if the value of the field is `_3V0`"]
    #[inline(always)]
    pub fn is_3v0(&self) -> bool {
        *self == SDBVSELSELECT_A::_3V0
    }
    #[doc = "Checks if the value of the field is `_3V3`"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        *self == SDBVSELSELECT_A::_3V3
    }
}
#[doc = "Field `SDBVSEL` writer - SD Bus Voltage Select"]
pub type SDBVSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, PCR_SPEC, u8, SDBVSELSELECT_A, 3, O>;
impl<'a, const O: u8> SDBVSEL_W<'a, O> {
    #[doc = "1.8V (Typ.)"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(SDBVSELSELECT_A::_1V8)
    }
    #[doc = "3.0V (Typ.)"]
    #[inline(always)]
    pub fn _3v0(self) -> &'a mut W {
        self.variant(SDBVSELSELECT_A::_3V0)
    }
    #[doc = "3.3V (Typ.)"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut W {
        self.variant(SDBVSELSELECT_A::_3V3)
    }
}
impl R {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbpwr(&self) -> SDBPWR_R {
        SDBPWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbvsel(&self) -> SDBVSEL_R {
        SDBVSEL_R::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    #[must_use]
    pub fn sdbpwr(&mut self) -> SDBPWR_W<0> {
        SDBPWR_W::new(self)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdbvsel(&mut self) -> SDBVSEL_W<1> {
        SDBVSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR to value 0x0e"]
impl crate::Resettable for PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e;
}
