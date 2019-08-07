#[doc = "Reader of register WINCTRL"]
pub type R = crate::R<u8, super::WINCTRL>;
#[doc = "Writer for register WINCTRL"]
pub type W = crate::W<u8, super::WINCTRL>;
#[doc = "Register WINCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WINCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WEN0`"]
pub type WEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WEN0`"]
pub struct WEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> WEN0_W<'a> {
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
#[doc = "Possible values of the field `WINTSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINTSEL0_A {
    #[doc = "Interrupt on signal above window"]
    ABOVE,
    #[doc = "Interrupt on signal inside window"]
    INSIDE,
    #[doc = "Interrupt on signal below window"]
    BELOW,
    #[doc = "Interrupt on signal outside window"]
    OUTSIDE,
}
impl crate::ToBits<u8> for WINTSEL0_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WINTSEL0_A::ABOVE => 0,
            WINTSEL0_A::INSIDE => 1,
            WINTSEL0_A::BELOW => 2,
            WINTSEL0_A::OUTSIDE => 3,
        }
    }
}
#[doc = "Reader of field `WINTSEL0`"]
pub type WINTSEL0_R = crate::R<u8, WINTSEL0_A>;
impl WINTSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WINTSEL0_A {
        match self.bits {
            0 => WINTSEL0_A::ABOVE,
            1 => WINTSEL0_A::INSIDE,
            2 => WINTSEL0_A::BELOW,
            3 => WINTSEL0_A::OUTSIDE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == WINTSEL0_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == WINTSEL0_A::INSIDE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == WINTSEL0_A::BELOW
    }
    #[doc = "Checks if the value of the field is `OUTSIDE`"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == WINTSEL0_A::OUTSIDE
    }
}
#[doc = "Write proxy for field `WINTSEL0`"]
pub struct WINTSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> WINTSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WINTSEL0_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt on signal above window"]
    #[inline(always)]
    pub fn above(self) -> &'a mut W {
        self.variant(WINTSEL0_A::ABOVE)
    }
    #[doc = "Interrupt on signal inside window"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut W {
        self.variant(WINTSEL0_A::INSIDE)
    }
    #[doc = "Interrupt on signal below window"]
    #[inline(always)]
    pub fn below(self) -> &'a mut W {
        self.variant(WINTSEL0_A::BELOW)
    }
    #[doc = "Interrupt on signal outside window"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut W {
        self.variant(WINTSEL0_A::OUTSIDE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u8) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline(always)]
    pub fn wen0(&self) -> WEN0_R {
        WEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline(always)]
    pub fn wintsel0(&self) -> WINTSEL0_R {
        WINTSEL0_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline(always)]
    pub fn wen0(&mut self) -> WEN0_W {
        WEN0_W { w: self }
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline(always)]
    pub fn wintsel0(&mut self) -> WINTSEL0_W {
        WINTSEL0_W { w: self }
    }
}
