#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `LQOS`"]
pub type LQOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LQOS`"]
pub struct LQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> LQOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "DMA Trigger Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCCDMALEVEL_A {
    #[doc = "0: Trigger rises when DCC is empty"]
    EMPTY = 0,
    #[doc = "1: Trigger rises when DCC is full"]
    FULL = 1,
}
impl From<DCCDMALEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCCDMALEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCCDMALEVEL`"]
pub type DCCDMALEVEL_R = crate::R<u8, DCCDMALEVEL_A>;
impl DCCDMALEVEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DCCDMALEVEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DCCDMALEVEL_A::EMPTY),
            1 => Val(DCCDMALEVEL_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == DCCDMALEVEL_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DCCDMALEVEL_A::FULL
    }
}
#[doc = "Write proxy for field `DCCDMALEVEL`"]
pub struct DCCDMALEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCCDMALEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCCDMALEVEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trigger rises when DCC is empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(DCCDMALEVEL_A::EMPTY)
    }
    #[doc = "Trigger rises when DCC is full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(DCCDMALEVEL_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ETBRAMEN`"]
pub type ETBRAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETBRAMEN`"]
pub struct ETBRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETBRAMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    pub fn lqos(&self) -> LQOS_R {
        LQOS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    pub fn dccdmalevel(&self) -> DCCDMALEVEL_R {
        DCCDMALEVEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    pub fn etbramen(&self) -> ETBRAMEN_R {
        ETBRAMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    pub fn lqos(&mut self) -> LQOS_W {
        LQOS_W { w: self }
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    pub fn dccdmalevel(&mut self) -> DCCDMALEVEL_W {
        DCCDMALEVEL_W { w: self }
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    pub fn etbramen(&mut self) -> ETBRAMEN_W {
        ETBRAMEN_W { w: self }
    }
}
