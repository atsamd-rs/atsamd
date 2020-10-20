#[doc = "Reader of register MCCAR"]
pub type R = crate::R<u32, super::MCCAR>;
#[doc = "Maximum Current for 3.3V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAXCUR33V_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
    #[doc = "1: 4mA"]
    _4MA = 1,
    #[doc = "2: 8mA"]
    _8MA = 2,
    #[doc = "3: 12mA"]
    _12MA = 3,
}
impl From<MAXCUR33V_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXCUR33V_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAXCUR33V`"]
pub type MAXCUR33V_R = crate::R<u8, MAXCUR33V_A>;
impl MAXCUR33V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAXCUR33V_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAXCUR33V_A::OTHER),
            1 => Val(MAXCUR33V_A::_4MA),
            2 => Val(MAXCUR33V_A::_8MA),
            3 => Val(MAXCUR33V_A::_12MA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == MAXCUR33V_A::OTHER
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == MAXCUR33V_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline(always)]
    pub fn is_8ma(&self) -> bool {
        *self == MAXCUR33V_A::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline(always)]
    pub fn is_12ma(&self) -> bool {
        *self == MAXCUR33V_A::_12MA
    }
}
#[doc = "Maximum Current for 3.0V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAXCUR30V_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
    #[doc = "1: 4mA"]
    _4MA = 1,
    #[doc = "2: 8mA"]
    _8MA = 2,
    #[doc = "3: 12mA"]
    _12MA = 3,
}
impl From<MAXCUR30V_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXCUR30V_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAXCUR30V`"]
pub type MAXCUR30V_R = crate::R<u8, MAXCUR30V_A>;
impl MAXCUR30V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAXCUR30V_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAXCUR30V_A::OTHER),
            1 => Val(MAXCUR30V_A::_4MA),
            2 => Val(MAXCUR30V_A::_8MA),
            3 => Val(MAXCUR30V_A::_12MA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == MAXCUR30V_A::OTHER
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == MAXCUR30V_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline(always)]
    pub fn is_8ma(&self) -> bool {
        *self == MAXCUR30V_A::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline(always)]
    pub fn is_12ma(&self) -> bool {
        *self == MAXCUR30V_A::_12MA
    }
}
#[doc = "Maximum Current for 1.8V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAXCUR18V_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
    #[doc = "1: 4mA"]
    _4MA = 1,
    #[doc = "2: 8mA"]
    _8MA = 2,
    #[doc = "3: 12mA"]
    _12MA = 3,
}
impl From<MAXCUR18V_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXCUR18V_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAXCUR18V`"]
pub type MAXCUR18V_R = crate::R<u8, MAXCUR18V_A>;
impl MAXCUR18V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAXCUR18V_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAXCUR18V_A::OTHER),
            1 => Val(MAXCUR18V_A::_4MA),
            2 => Val(MAXCUR18V_A::_8MA),
            3 => Val(MAXCUR18V_A::_12MA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == MAXCUR18V_A::OTHER
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == MAXCUR18V_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline(always)]
    pub fn is_8ma(&self) -> bool {
        *self == MAXCUR18V_A::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline(always)]
    pub fn is_12ma(&self) -> bool {
        *self == MAXCUR18V_A::_12MA
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn maxcur33v(&self) -> MAXCUR33V_R {
        MAXCUR33V_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V"]
    #[inline(always)]
    pub fn maxcur30v(&self) -> MAXCUR30V_R {
        MAXCUR30V_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V"]
    #[inline(always)]
    pub fn maxcur18v(&self) -> MAXCUR18V_R {
        MAXCUR18V_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
