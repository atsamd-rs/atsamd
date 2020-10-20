#[doc = "Reader of register LENGTH"]
pub type R = crate::R<u16, super::LENGTH>;
#[doc = "Writer for register LENGTH"]
pub type W = crate::W<u16, super::LENGTH>;
#[doc = "Register LENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::LENGTH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEN`"]
pub type LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEN`"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `LENEN`"]
pub type LENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LENEN`"]
pub struct LENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LENEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Data Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LENEN_R {
        LENEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Bit 8 - Data Length Enable"]
    #[inline(always)]
    pub fn lenen(&mut self) -> LENEN_W {
        LENEN_W { w: self }
    }
}
