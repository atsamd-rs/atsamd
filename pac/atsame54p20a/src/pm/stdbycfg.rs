#[doc = "Reader of register STDBYCFG"]
pub type R = crate::R<u8, super::STDBYCFG>;
#[doc = "Writer for register STDBYCFG"]
pub type W = crate::W<u8, super::STDBYCFG>;
#[doc = "Register STDBYCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::STDBYCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMCFG_A {
    #[doc = "0: All the RAMs are retained"]
    RET = 0,
    #[doc = "1: Only the first 32K bytes are retained"]
    PARTIAL = 1,
    #[doc = "2: All the RAMs are OFF"]
    OFF = 2,
}
impl From<RAMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAMCFG`"]
pub type RAMCFG_R = crate::R<u8, RAMCFG_A>;
impl RAMCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAMCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAMCFG_A::RET),
            1 => Val(RAMCFG_A::PARTIAL),
            2 => Val(RAMCFG_A::OFF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RET`"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == RAMCFG_A::RET
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == RAMCFG_A::PARTIAL
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RAMCFG_A::OFF
    }
}
#[doc = "Write proxy for field `RAMCFG`"]
pub struct RAMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All the RAMs are retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut W {
        self.variant(RAMCFG_A::RET)
    }
    #[doc = "Only the first 32K bytes are retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(RAMCFG_A::PARTIAL)
    }
    #[doc = "All the RAMs are OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RAMCFG_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FASTWKUP`"]
pub type FASTWKUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FASTWKUP`"]
pub struct FASTWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTWKUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&self) -> RAMCFG_R {
        RAMCFG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FASTWKUP_R {
        FASTWKUP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&mut self) -> RAMCFG_W {
        RAMCFG_W { w: self }
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&mut self) -> FASTWKUP_W {
        FASTWKUP_W { w: self }
    }
}
