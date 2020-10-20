#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u8, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u8, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLUSHEI`"]
pub type FLUSHEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLUSHEI`"]
pub struct FLUSHEI_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHEI_W<'a> {
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
#[doc = "Reader of field `STARTEI`"]
pub type STARTEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTEI`"]
pub struct STARTEI_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTEI_W<'a> {
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
#[doc = "Reader of field `FLUSHINV`"]
pub type FLUSHINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLUSHINV`"]
pub struct FLUSHINV_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHINV_W<'a> {
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
#[doc = "Reader of field `STARTINV`"]
pub type STARTINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTINV`"]
pub struct STARTINV_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RESRDYEO`"]
pub type RESRDYEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESRDYEO`"]
pub struct RESRDYEO_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRDYEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `WINMONEO`"]
pub type WINMONEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WINMONEO`"]
pub struct WINMONEO_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMONEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flush Event Input Enable"]
    #[inline(always)]
    pub fn flushei(&self) -> FLUSHEI_R {
        FLUSHEI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Input Enable"]
    #[inline(always)]
    pub fn startei(&self) -> STARTEI_R {
        STARTEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flush Event Invert Enable"]
    #[inline(always)]
    pub fn flushinv(&self) -> FLUSHINV_R {
        FLUSHINV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    pub fn startinv(&self) -> STARTINV_R {
        STARTINV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&self) -> RESRDYEO_R {
        RESRDYEO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&self) -> WINMONEO_R {
        WINMONEO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Event Input Enable"]
    #[inline(always)]
    pub fn flushei(&mut self) -> FLUSHEI_W {
        FLUSHEI_W { w: self }
    }
    #[doc = "Bit 1 - Start Conversion Event Input Enable"]
    #[inline(always)]
    pub fn startei(&mut self) -> STARTEI_W {
        STARTEI_W { w: self }
    }
    #[doc = "Bit 2 - Flush Event Invert Enable"]
    #[inline(always)]
    pub fn flushinv(&mut self) -> FLUSHINV_W {
        FLUSHINV_W { w: self }
    }
    #[doc = "Bit 3 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    pub fn startinv(&mut self) -> STARTINV_W {
        STARTINV_W { w: self }
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&mut self) -> RESRDYEO_W {
        RESRDYEO_W { w: self }
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&mut self) -> WINMONEO_W {
        WINMONEO_W { w: self }
    }
}
