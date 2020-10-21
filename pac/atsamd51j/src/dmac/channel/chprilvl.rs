#[doc = "Reader of register CHPRILVL"]
pub type R = crate::R<u8, super::CHPRILVL>;
#[doc = "Writer for register CHPRILVL"]
pub type W = crate::W<u8, super::CHPRILVL>;
#[doc = "Register CHPRILVL `reset()`'s with value 0"]
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
    #[doc = "3: Channel Priority Level 3 (Highest Level)"]
    LVL3 = 3,
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
    pub fn variant(&self) -> PRILVL_A {
        match self.bits {
            0 => PRILVL_A::LVL0,
            1 => PRILVL_A::LVL1,
            2 => PRILVL_A::LVL2,
            3 => PRILVL_A::LVL3,
            _ => unreachable!(),
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
}
#[doc = "Write proxy for field `PRILVL`"]
pub struct PRILVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRILVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRILVL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
    #[doc = "Channel Priority Level 3 (Highest Level)"]
    #[inline(always)]
    pub fn lvl3(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
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
