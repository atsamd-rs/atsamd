#[doc = "Reader of register BKOUT"]
pub type R = crate::R<u32, super::BKOUT>;
#[doc = "Writer for register BKOUT"]
pub type W = crate::W<u32, super::BKOUT>;
#[doc = "Register BKOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::BKOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENOUT0`"]
pub type ENOUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENOUT0`"]
pub struct ENOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENOUT0_W<'a> {
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
#[doc = "Reader of field `ENOUT1`"]
pub type ENOUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENOUT1`"]
pub struct ENOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENOUT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CLROUT0`"]
pub type CLROUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLROUT0`"]
pub struct CLROUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLROUT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLROUT1`"]
pub type CLROUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLROUT1`"]
pub struct CLROUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLROUT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SETOUT0`"]
pub type SETOUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETOUT0`"]
pub struct SETOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SETOUT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SETOUT1`"]
pub type SETOUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETOUT1`"]
pub struct SETOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SETOUT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTCTGLOUT0`"]
pub type RTCTGLOUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTGLOUT0`"]
pub struct RTCTGLOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTGLOUT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RTCTGLOUT1`"]
pub type RTCTGLOUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTGLOUT1`"]
pub struct RTCTGLOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTGLOUT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable OUT0"]
    #[inline(always)]
    pub fn enout0(&self) -> ENOUT0_R {
        ENOUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable OUT1"]
    #[inline(always)]
    pub fn enout1(&self) -> ENOUT1_R {
        ENOUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clear OUT0"]
    #[inline(always)]
    pub fn clrout0(&self) -> CLROUT0_R {
        CLROUT0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear OUT1"]
    #[inline(always)]
    pub fn clrout1(&self) -> CLROUT1_R {
        CLROUT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set OUT0"]
    #[inline(always)]
    pub fn setout0(&self) -> SETOUT0_R {
        SETOUT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set OUT1"]
    #[inline(always)]
    pub fn setout1(&self) -> SETOUT1_R {
        SETOUT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RTC Toggle OUT0"]
    #[inline(always)]
    pub fn rtctglout0(&self) -> RTCTGLOUT0_R {
        RTCTGLOUT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RTC Toggle OUT1"]
    #[inline(always)]
    pub fn rtctglout1(&self) -> RTCTGLOUT1_R {
        RTCTGLOUT1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable OUT0"]
    #[inline(always)]
    pub fn enout0(&mut self) -> ENOUT0_W {
        ENOUT0_W { w: self }
    }
    #[doc = "Bit 1 - Enable OUT1"]
    #[inline(always)]
    pub fn enout1(&mut self) -> ENOUT1_W {
        ENOUT1_W { w: self }
    }
    #[doc = "Bit 8 - Clear OUT0"]
    #[inline(always)]
    pub fn clrout0(&mut self) -> CLROUT0_W {
        CLROUT0_W { w: self }
    }
    #[doc = "Bit 9 - Clear OUT1"]
    #[inline(always)]
    pub fn clrout1(&mut self) -> CLROUT1_W {
        CLROUT1_W { w: self }
    }
    #[doc = "Bit 16 - Set OUT0"]
    #[inline(always)]
    pub fn setout0(&mut self) -> SETOUT0_W {
        SETOUT0_W { w: self }
    }
    #[doc = "Bit 17 - Set OUT1"]
    #[inline(always)]
    pub fn setout1(&mut self) -> SETOUT1_W {
        SETOUT1_W { w: self }
    }
    #[doc = "Bit 24 - RTC Toggle OUT0"]
    #[inline(always)]
    pub fn rtctglout0(&mut self) -> RTCTGLOUT0_W {
        RTCTGLOUT0_W { w: self }
    }
    #[doc = "Bit 25 - RTC Toggle OUT1"]
    #[inline(always)]
    pub fn rtctglout1(&mut self) -> RTCTGLOUT1_W {
        RTCTGLOUT1_W { w: self }
    }
}
