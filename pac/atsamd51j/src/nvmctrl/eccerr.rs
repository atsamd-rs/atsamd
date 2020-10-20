#[doc = "Reader of register ECCERR"]
pub type R = crate::R<u32, super::ECCERR>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
#[doc = "Low Double-Word Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPEL_A {
    #[doc = "0: No Error Detected Since Last Read"]
    NONE = 0,
    #[doc = "1: At Least One Single Error Detected Since last Read"]
    SINGLE = 1,
    #[doc = "2: At Least One Dual Error Detected Since Last Read"]
    DUAL = 2,
}
impl From<TYPEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TYPEL`"]
pub type TYPEL_R = crate::R<u8, TYPEL_A>;
impl TYPEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TYPEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TYPEL_A::NONE),
            1 => Val(TYPEL_A::SINGLE),
            2 => Val(TYPEL_A::DUAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TYPEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TYPEL_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == TYPEL_A::DUAL
    }
}
#[doc = "High Double-Word Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPEH_A {
    #[doc = "0: No Error Detected Since Last Read"]
    NONE = 0,
    #[doc = "1: At Least One Single Error Detected Since last Read"]
    SINGLE = 1,
    #[doc = "2: At Least One Dual Error Detected Since Last Read"]
    DUAL = 2,
}
impl From<TYPEH_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPEH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TYPEH`"]
pub type TYPEH_R = crate::R<u8, TYPEH_A>;
impl TYPEH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TYPEH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TYPEH_A::NONE),
            1 => Val(TYPEH_A::SINGLE),
            2 => Val(TYPEH_A::DUAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TYPEH_A::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TYPEH_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == TYPEH_A::DUAL
    }
}
impl R {
    #[doc = "Bits 0:23 - Error Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 28:29 - Low Double-Word Error Type"]
    #[inline(always)]
    pub fn typel(&self) -> TYPEL_R {
        TYPEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - High Double-Word Error Type"]
    #[inline(always)]
    pub fn typeh(&self) -> TYPEH_R {
        TYPEH_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
