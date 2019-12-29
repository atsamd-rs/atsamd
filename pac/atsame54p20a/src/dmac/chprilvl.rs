#[doc = "Reader of register CHPRILVL%s"]
pub type R = crate::R<u8, super::CHPRILVL>;
#[doc = "Writer for register CHPRILVL%s"]
pub type W = crate::W<u8, super::CHPRILVL>;
#[doc = "Register CHPRILVL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CHPRILVL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel Priority Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRILVL_A {
    #[doc = "0: Channel Priority Level 0 (Lowest Level)"]
    LVL0 = 0,
    #[doc = "1: Channel Priority Level 1"]
    LVL1 = 1,
    #[doc = "2: Channel Priority Level 2"]
    LVL2 = 2,
    #[doc = "3: Channel Priority Level 3"]
    LVL3 = 3,
    #[doc = "4: Channel Priority Level 4"]
    LVL4 = 4,
    #[doc = "5: Channel Priority Level 5"]
    LVL5 = 5,
    #[doc = "6: Channel Priority Level 6"]
    LVL6 = 6,
    #[doc = "7: Channel Priority Level 7 (Highest Level)"]
    LVL7 = 7,
}
impl From<PRILVL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRILVL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRILVL`"]
pub type PRILVL_R = crate::R<u8, PRILVL_A>;
impl PRILVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRILVL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRILVL_A::LVL0),
            1 => Val(PRILVL_A::LVL1),
            2 => Val(PRILVL_A::LVL2),
            3 => Val(PRILVL_A::LVL3),
            4 => Val(PRILVL_A::LVL4),
            5 => Val(PRILVL_A::LVL5),
            6 => Val(PRILVL_A::LVL6),
            7 => Val(PRILVL_A::LVL7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LVL0`"]
    #[inline(always)]
    pub fn is_lvl0(&self) -> bool {
        *self == PRILVL_A::LVL0
    }
    #[doc = "Checks if the value of the field is `LVL1`"]
    #[inline(always)]
    pub fn is_lvl1(&self) -> bool {
        *self == PRILVL_A::LVL1
    }
    #[doc = "Checks if the value of the field is `LVL2`"]
    #[inline(always)]
    pub fn is_lvl2(&self) -> bool {
        *self == PRILVL_A::LVL2
    }
    #[doc = "Checks if the value of the field is `LVL3`"]
    #[inline(always)]
    pub fn is_lvl3(&self) -> bool {
        *self == PRILVL_A::LVL3
    }
    #[doc = "Checks if the value of the field is `LVL4`"]
    #[inline(always)]
    pub fn is_lvl4(&self) -> bool {
        *self == PRILVL_A::LVL4
    }
    #[doc = "Checks if the value of the field is `LVL5`"]
    #[inline(always)]
    pub fn is_lvl5(&self) -> bool {
        *self == PRILVL_A::LVL5
    }
    #[doc = "Checks if the value of the field is `LVL6`"]
    #[inline(always)]
    pub fn is_lvl6(&self) -> bool {
        *self == PRILVL_A::LVL6
    }
    #[doc = "Checks if the value of the field is `LVL7`"]
    #[inline(always)]
    pub fn is_lvl7(&self) -> bool {
        *self == PRILVL_A::LVL7
    }
}
#[doc = "Write proxy for field `PRILVL`"]
pub struct PRILVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRILVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRILVL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    #[inline(always)]
    pub fn lvl0(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn lvl1(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn lvl2(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL2)
    }
    #[doc = "Channel Priority Level 3"]
    #[inline(always)]
    pub fn lvl3(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL3)
    }
    #[doc = "Channel Priority Level 4"]
    #[inline(always)]
    pub fn lvl4(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL4)
    }
    #[doc = "Channel Priority Level 5"]
    #[inline(always)]
    pub fn lvl5(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL5)
    }
    #[doc = "Channel Priority Level 6"]
    #[inline(always)]
    pub fn lvl6(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL6)
    }
    #[doc = "Channel Priority Level 7 (Highest Level)"]
    #[inline(always)]
    pub fn lvl7(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline(always)]
    pub fn prilvl(&self) -> PRILVL_R {
        PRILVL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline(always)]
    pub fn prilvl(&mut self) -> PRILVL_W {
        PRILVL_W { w: self }
    }
}
