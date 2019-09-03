#[doc = "Reader of register INTFLAG"]
pub type R = crate::R<u8, super::INTFLAG>;
#[doc = "Writer for register INTFLAG"]
pub type W = crate::W<u8, super::INTFLAG>;
#[doc = "Register INTFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UNDERRUN`"]
pub type UNDERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDERRUN`"]
pub struct UNDERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERRUN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMPTY`"]
pub struct EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SYNCRDY`"]
pub type SYNCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCRDY`"]
pub struct SYNCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRDY_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Underrun"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SYNCRDY_R {
        SYNCRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underrun"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UNDERRUN_W {
        UNDERRUN_W { w: self }
    }
    #[doc = "Bit 1 - Data Buffer Empty"]
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W {
        EMPTY_W { w: self }
    }
    #[doc = "Bit 2 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&mut self) -> SYNCRDY_W {
        SYNCRDY_W { w: self }
    }
}
