#[doc = "Reader of register STATUSA"]
pub type R = crate::R<u8, super::STATUSA>;
#[doc = "Reader of field `STATE0`"]
pub type STATE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `STATE1`"]
pub type STATE1_R = crate::R<bool, bool>;
#[doc = "Possible values of the field `WSTATE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSTATE0_A {
    #[doc = "Signal is above window"]
    ABOVE,
    #[doc = "Signal is inside window"]
    INSIDE,
    #[doc = "Signal is below window"]
    BELOW,
}
impl crate::ToBits<u8> for WSTATE0_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WSTATE0_A::ABOVE => 0,
            WSTATE0_A::INSIDE => 1,
            WSTATE0_A::BELOW => 2,
        }
    }
}
#[doc = "Reader of field `WSTATE0`"]
pub type WSTATE0_R = crate::R<u8, WSTATE0_A>;
impl WSTATE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WSTATE0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WSTATE0_A::ABOVE),
            1 => Val(WSTATE0_A::INSIDE),
            2 => Val(WSTATE0_A::BELOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == WSTATE0_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == WSTATE0_A::INSIDE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == WSTATE0_A::BELOW
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 Current State"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Current State"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Window 0 Current State"]
    #[inline(always)]
    pub fn wstate0(&self) -> WSTATE0_R {
        WSTATE0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
