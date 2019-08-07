#[doc = "Reader of register MASK%s"]
pub type R = crate::R<u8, super::MASK>;
#[doc = "Writer for register MASK%s"]
pub type W = crate::W<u8, super::MASK>;
#[doc = "Register MASK%s `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "Alarm Disabled"]
    OFF,
    #[doc = "Match seconds only"]
    SS,
    #[doc = "Match seconds and minutes only"]
    MMSS,
    #[doc = "Match seconds, minutes, and hours only"]
    HHMMSS,
    #[doc = "Match seconds, minutes, hours, and days only"]
    DDHHMMSS,
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    MMDDHHMMSS,
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    YYMMDDHHMMSS,
}
impl crate::ToBits<u8> for SEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SEL_A::OFF => 0,
            SEL_A::SS => 1,
            SEL_A::MMSS => 2,
            SEL_A::HHMMSS => 3,
            SEL_A::DDHHMMSS => 4,
            SEL_A::MMDDHHMMSS => 5,
            SEL_A::YYMMDDHHMMSS => 6,
        }
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::OFF),
            1 => Val(SEL_A::SS),
            2 => Val(SEL_A::MMSS),
            3 => Val(SEL_A::HHMMSS),
            4 => Val(SEL_A::DDHHMMSS),
            5 => Val(SEL_A::MMDDHHMMSS),
            6 => Val(SEL_A::YYMMDDHHMMSS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `SS`"]
    #[inline(always)]
    pub fn is_ss(&self) -> bool {
        *self == SEL_A::SS
    }
    #[doc = "Checks if the value of the field is `MMSS`"]
    #[inline(always)]
    pub fn is_mmss(&self) -> bool {
        *self == SEL_A::MMSS
    }
    #[doc = "Checks if the value of the field is `HHMMSS`"]
    #[inline(always)]
    pub fn is_hhmmss(&self) -> bool {
        *self == SEL_A::HHMMSS
    }
    #[doc = "Checks if the value of the field is `DDHHMMSS`"]
    #[inline(always)]
    pub fn is_ddhhmmss(&self) -> bool {
        *self == SEL_A::DDHHMMSS
    }
    #[doc = "Checks if the value of the field is `MMDDHHMMSS`"]
    #[inline(always)]
    pub fn is_mmddhhmmss(&self) -> bool {
        *self == SEL_A::MMDDHHMMSS
    }
    #[doc = "Checks if the value of the field is `YYMMDDHHMMSS`"]
    #[inline(always)]
    pub fn is_yymmddhhmmss(&self) -> bool {
        *self == SEL_A::YYMMDDHHMMSS
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alarm Disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SEL_A::OFF)
    }
    #[doc = "Match seconds only"]
    #[inline(always)]
    pub fn ss(self) -> &'a mut W {
        self.variant(SEL_A::SS)
    }
    #[doc = "Match seconds and minutes only"]
    #[inline(always)]
    pub fn mmss(self) -> &'a mut W {
        self.variant(SEL_A::MMSS)
    }
    #[doc = "Match seconds, minutes, and hours only"]
    #[inline(always)]
    pub fn hhmmss(self) -> &'a mut W {
        self.variant(SEL_A::HHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, and days only"]
    #[inline(always)]
    pub fn ddhhmmss(self) -> &'a mut W {
        self.variant(SEL_A::DDHHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    #[inline(always)]
    pub fn mmddhhmmss(self) -> &'a mut W {
        self.variant(SEL_A::MMDDHHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    #[inline(always)]
    pub fn yymmddhhmmss(self) -> &'a mut W {
        self.variant(SEL_A::YYMMDDHHMMSS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
