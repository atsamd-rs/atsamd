#[doc = "Reader of register TOCC"]
pub type R = crate::R<u32, super::TOCC>;
#[doc = "Writer for register TOCC"]
pub type W = crate::W<u32, super::TOCC>;
#[doc = "Register TOCC `reset()`'s with value 0xffff_0000"]
impl crate::ResetValue for super::TOCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
#[doc = "Reader of field `ETOC`"]
pub type ETOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETOC`"]
pub struct ETOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETOC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Timeout Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOS_A {
    #[doc = "0: Continuout operation"]
    CONT = 0,
    #[doc = "1: Timeout controlled by TX Event FIFO"]
    TXEF = 1,
    #[doc = "2: Timeout controlled by Rx FIFO 0"]
    RXF0 = 2,
    #[doc = "3: Timeout controlled by Rx FIFO 1"]
    RXF1 = 3,
}
impl From<TOS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TOS`"]
pub type TOS_R = crate::R<u8, TOS_A>;
impl TOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS_A {
        match self.bits {
            0 => TOS_A::CONT,
            1 => TOS_A::TXEF,
            2 => TOS_A::RXF0,
            3 => TOS_A::RXF1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == TOS_A::CONT
    }
    #[doc = "Checks if the value of the field is `TXEF`"]
    #[inline(always)]
    pub fn is_txef(&self) -> bool {
        *self == TOS_A::TXEF
    }
    #[doc = "Checks if the value of the field is `RXF0`"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == TOS_A::RXF0
    }
    #[doc = "Checks if the value of the field is `RXF1`"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == TOS_A::RXF1
    }
}
#[doc = "Write proxy for field `TOS`"]
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Continuout operation"]
    #[inline(always)]
    pub fn cont(self) -> &'a mut W {
        self.variant(TOS_A::CONT)
    }
    #[doc = "Timeout controlled by TX Event FIFO"]
    #[inline(always)]
    pub fn txef(self) -> &'a mut W {
        self.variant(TOS_A::TXEF)
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut W {
        self.variant(TOS_A::RXF0)
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut W {
        self.variant(TOS_A::RXF1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOP`"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W {
        ETOC_W { w: self }
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
}
