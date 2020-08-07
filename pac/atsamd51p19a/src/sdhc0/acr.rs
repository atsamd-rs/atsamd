#[doc = "Reader of register ACR"]
pub type R = crate::R<u32, super::ACR>;
#[doc = "Writer for register ACR"]
pub type W = crate::W<u32, super::ACR>;
#[doc = "Register ACR `reset()`'s with value 0"]
impl crate::ResetValue for super::ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AHB Maximum Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BMAX_A {
    #[doc = "0: `0`"]
    INCR16 = 0,
    #[doc = "1: `1`"]
    INCR8 = 1,
    #[doc = "2: `10`"]
    INCR4 = 2,
    #[doc = "3: `11`"]
    SINGLE = 3,
}
impl From<BMAX_A> for u8 {
    #[inline(always)]
    fn from(variant: BMAX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BMAX`"]
pub type BMAX_R = crate::R<u8, BMAX_A>;
impl BMAX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMAX_A {
        match self.bits {
            0 => BMAX_A::INCR16,
            1 => BMAX_A::INCR8,
            2 => BMAX_A::INCR4,
            3 => BMAX_A::SINGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == BMAX_A::INCR16
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == BMAX_A::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == BMAX_A::INCR4
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == BMAX_A::SINGLE
    }
}
#[doc = "Write proxy for field `BMAX`"]
pub struct BMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> BMAX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMAX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(BMAX_A::INCR16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(BMAX_A::INCR8)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(BMAX_A::INCR4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(BMAX_A::SINGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    pub fn bmax(&self) -> BMAX_R {
        BMAX_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    pub fn bmax(&mut self) -> BMAX_W {
        BMAX_W { w: self }
    }
}
