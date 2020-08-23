#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Full-speed mode"]
    FS = 0,
    #[doc = "1: Low-speed mode"]
    LS = 1,
    #[doc = "2: High-speed mode"]
    HS = 2,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
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
            1 => Val(SPEED_A::LS),
            2 => Val(SPEED_A::HS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == SPEED_A::FS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == SPEED_A::LS
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == SPEED_A::HS
    }
}
#[doc = "USB Line State Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINESTATE_A {
    #[doc = "0: SE0/RESET"]
    _0 = 0,
    #[doc = "1: FS-J or LS-K State"]
    _1 = 1,
    #[doc = "2: FS-K or LS-J State"]
    _2 = 2,
}
impl From<LINESTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: LINESTATE_A) -> Self {
        variant as _
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
