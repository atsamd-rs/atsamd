#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEED_A {
    #[doc = "Full-speed mode"]
    FS,
    #[doc = "High-speed mode"]
    HS,
    #[doc = "Low-speed mode"]
    LS,
}
impl crate::ToBits<u8> for SPEED_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SPEED_A::FS => 0,
            SPEED_A::HS => 1,
            SPEED_A::LS => 2,
        }
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPEED_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPEED_A::FS),
            1 => Val(SPEED_A::HS),
            2 => Val(SPEED_A::LS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == SPEED_A::FS
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == SPEED_A::HS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == SPEED_A::LS
    }
}
#[doc = "Possible values of the field `LINESTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATE_A {
    #[doc = "SE0/RESET"]
    _0,
    #[doc = "FS-J or LS-K State"]
    _1,
    #[doc = "FS-K or LS-J State"]
    _2,
}
impl crate::ToBits<u8> for LINESTATE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            LINESTATE_A::_0 => 0,
            LINESTATE_A::_1 => 1,
            LINESTATE_A::_2 => 2,
        }
    }
}
#[doc = "Reader of field `LINESTATE`"]
pub type LINESTATE_R = crate::R<u8, LINESTATE_A>;
impl LINESTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LINESTATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LINESTATE_A::_0),
            1 => Val(LINESTATE_A::_1),
            2 => Val(LINESTATE_A::_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINESTATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINESTATE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == LINESTATE_A::_2
    }
}
impl R {
    #[doc = "Bits 2:3 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - USB Line State Status"]
    #[inline(always)]
    pub fn linestate(&self) -> LINESTATE_R {
        LINESTATE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
