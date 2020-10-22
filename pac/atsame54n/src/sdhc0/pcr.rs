#[doc = "Reader of register PCR"]
pub type R = crate::R<u8, super::PCR>;
#[doc = "Writer for register PCR"]
pub type W = crate::W<u8, super::PCR>;
#[doc = "Register PCR `reset()`'s with value 0x0e"]
impl crate::ResetValue for super::PCR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0e
    }
}
#[doc = "SD Bus Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDBPWR_A {
    #[doc = "0: Power off"]
    OFF = 0,
    #[doc = "1: Power on"]
    ON = 1,
}
impl From<SDBPWR_A> for bool {
    #[inline(always)]
    fn from(variant: SDBPWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDBPWR`"]
pub type SDBPWR_R = crate::R<bool, SDBPWR_A>;
impl SDBPWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDBPWR_A {
        match self.bits {
            false => SDBPWR_A::OFF,
            true => SDBPWR_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SDBPWR_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == SDBPWR_A::ON
    }
}
#[doc = "Write proxy for field `SDBPWR`"]
pub struct SDBPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SDBPWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDBPWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SDBPWR_A::OFF)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SDBPWR_A::ON)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "SD Bus Voltage Select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDBVSEL_A {
    #[doc = "5: 1.8V (Typ.)"]
    _1V8 = 5,
    #[doc = "6: 3.0V (Typ.)"]
    _3V0 = 6,
    #[doc = "7: 3.3V (Typ.)"]
    _3V3 = 7,
}
impl From<SDBVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDBVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SDBVSEL`"]
pub type SDBVSEL_R = crate::R<u8, SDBVSEL_A>;
impl SDBVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDBVSEL_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(SDBVSEL_A::_1V8),
            6 => Val(SDBVSEL_A::_3V0),
            7 => Val(SDBVSEL_A::_3V3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == SDBVSEL_A::_1V8
    }
    #[doc = "Checks if the value of the field is `_3V0`"]
    #[inline(always)]
    pub fn is_3v0(&self) -> bool {
        *self == SDBVSEL_A::_3V0
    }
    #[doc = "Checks if the value of the field is `_3V3`"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        *self == SDBVSEL_A::_3V3
    }
}
#[doc = "Write proxy for field `SDBVSEL`"]
pub struct SDBVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDBVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDBVSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1.8V (Typ.)"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(SDBVSEL_A::_1V8)
    }
    #[doc = "3.0V (Typ.)"]
    #[inline(always)]
    pub fn _3v0(self) -> &'a mut W {
        self.variant(SDBVSEL_A::_3V0)
    }
    #[doc = "3.3V (Typ.)"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut W {
        self.variant(SDBVSEL_A::_3V3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u8) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbpwr(&self) -> SDBPWR_R {
        SDBPWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbvsel(&self) -> SDBVSEL_R {
        SDBVSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbpwr(&mut self) -> SDBPWR_W {
        SDBPWR_W { w: self }
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbvsel(&mut self) -> SDBVSEL_W {
        SDBVSEL_W { w: self }
    }
}
