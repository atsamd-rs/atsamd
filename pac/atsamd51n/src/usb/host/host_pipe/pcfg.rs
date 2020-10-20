#[doc = "Reader of register PCFG"]
pub type R = crate::R<u8, super::PCFG>;
#[doc = "Writer for register PCFG"]
pub type W = crate::W<u8, super::PCFG>;
#[doc = "Register PCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PTOKEN`"]
pub type PTOKEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PTOKEN`"]
pub struct PTOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTOKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `BK`"]
pub type BK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BK`"]
pub struct BK_W<'a> {
    w: &'a mut W,
}
impl<'a> BK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PTYPE`"]
pub type PTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PTYPE`"]
pub struct PTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u8) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    pub fn bk(&self) -> BK_R {
        BK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&mut self) -> PTOKEN_W {
        PTOKEN_W { w: self }
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    pub fn bk(&mut self) -> BK_W {
        BK_W { w: self }
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PTYPE_W {
        PTYPE_W { w: self }
    }
}
