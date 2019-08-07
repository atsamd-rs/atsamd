#[doc = "Reader of register APBBSEL"]
pub type R = crate::R<u8, super::APBBSEL>;
#[doc = "Writer for register APBBSEL"]
pub type W = crate::W<u8, super::APBBSEL>;
#[doc = "Register APBBSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::APBBSEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `APBBDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APBBDIV_A {
    #[doc = "Divide by 1"]
    DIV1,
    #[doc = "Divide by 2"]
    DIV2,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 32"]
    DIV32,
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 128"]
    DIV128,
}
impl crate::ToBits<u8> for APBBDIV_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            APBBDIV_A::DIV1 => 0,
            APBBDIV_A::DIV2 => 1,
            APBBDIV_A::DIV4 => 2,
            APBBDIV_A::DIV8 => 3,
            APBBDIV_A::DIV16 => 4,
            APBBDIV_A::DIV32 => 5,
            APBBDIV_A::DIV64 => 6,
            APBBDIV_A::DIV128 => 7,
        }
    }
}
#[doc = "Reader of field `APBBDIV`"]
pub type APBBDIV_R = crate::R<u8, APBBDIV_A>;
impl APBBDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APBBDIV_A {
        match self.bits {
            0 => APBBDIV_A::DIV1,
            1 => APBBDIV_A::DIV2,
            2 => APBBDIV_A::DIV4,
            3 => APBBDIV_A::DIV8,
            4 => APBBDIV_A::DIV16,
            5 => APBBDIV_A::DIV32,
            6 => APBBDIV_A::DIV64,
            7 => APBBDIV_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == APBBDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == APBBDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == APBBDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == APBBDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == APBBDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == APBBDIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == APBBDIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == APBBDIV_A::DIV128
    }
}
#[doc = "Write proxy for field `APBBDIV`"]
pub struct APBBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APBBDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APBBDIV_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(APBBDIV_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(APBBDIV_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(APBBDIV_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(APBBDIV_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(APBBDIV_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(APBBDIV_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(APBBDIV_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(APBBDIV_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - APBB Prescaler Selection"]
    #[inline(always)]
    pub fn apbbdiv(&self) -> APBBDIV_R {
        APBBDIV_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - APBB Prescaler Selection"]
    #[inline(always)]
    pub fn apbbdiv(&mut self) -> APBBDIV_W {
        APBBDIV_W { w: self }
    }
}
