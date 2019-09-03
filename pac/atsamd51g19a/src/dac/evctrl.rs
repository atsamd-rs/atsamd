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
#[doc = "Reader of field `STARTEI0`"]
pub type STARTEI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTEI0`"]
pub struct STARTEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTEI0_W<'a> {
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
#[doc = "Reader of field `STARTEI1`"]
pub type STARTEI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTEI1`"]
pub struct STARTEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTEI1_W<'a> {
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
#[doc = "Reader of field `EMPTYEO0`"]
pub type EMPTYEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMPTYEO0`"]
pub struct EMPTYEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYEO0_W<'a> {
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
#[doc = "Reader of field `EMPTYEO1`"]
pub type EMPTYEO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMPTYEO1`"]
pub struct EMPTYEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYEO1_W<'a> {
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
#[doc = "Reader of field `INVEI0`"]
pub type INVEI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEI0`"]
pub struct INVEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI0_W<'a> {
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
#[doc = "Reader of field `INVEI1`"]
pub type INVEI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEI1`"]
pub struct INVEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI1_W<'a> {
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
#[doc = "Reader of field `RESRDYEO0`"]
pub type RESRDYEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESRDYEO0`"]
pub struct RESRDYEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRDYEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RESRDYEO1`"]
pub type RESRDYEO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESRDYEO1`"]
pub struct RESRDYEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRDYEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start Conversion Event Input DAC 0"]
    #[inline(always)]
    pub fn startei0(&self) -> STARTEI0_R {
        STARTEI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Input DAC 1"]
    #[inline(always)]
    pub fn startei1(&self) -> STARTEI1_R {
        STARTEI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Buffer Empty Event Output DAC 0"]
    #[inline(always)]
    pub fn emptyeo0(&self) -> EMPTYEO0_R {
        EMPTYEO0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Buffer Empty Event Output DAC 1"]
    #[inline(always)]
    pub fn emptyeo1(&self) -> EMPTYEO1_R {
        EMPTYEO1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Invertion of DAC 0 input event"]
    #[inline(always)]
    pub fn invei0(&self) -> INVEI0_R {
        INVEI0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Invertion of DAC 1 input event"]
    #[inline(always)]
    pub fn invei1(&self) -> INVEI1_R {
        INVEI1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Result Ready Event Output 0"]
    #[inline(always)]
    pub fn resrdyeo0(&self) -> RESRDYEO0_R {
        RESRDYEO0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Result Ready Event Output 1"]
    #[inline(always)]
    pub fn resrdyeo1(&self) -> RESRDYEO1_R {
        RESRDYEO1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event Input DAC 0"]
    #[inline(always)]
    pub fn startei0(&mut self) -> STARTEI0_W {
        STARTEI0_W { w: self }
    }
    #[doc = "Bit 1 - Start Conversion Event Input DAC 1"]
    #[inline(always)]
    pub fn startei1(&mut self) -> STARTEI1_W {
        STARTEI1_W { w: self }
    }
    #[doc = "Bit 2 - Data Buffer Empty Event Output DAC 0"]
    #[inline(always)]
    pub fn emptyeo0(&mut self) -> EMPTYEO0_W {
        EMPTYEO0_W { w: self }
    }
    #[doc = "Bit 3 - Data Buffer Empty Event Output DAC 1"]
    #[inline(always)]
    pub fn emptyeo1(&mut self) -> EMPTYEO1_W {
        EMPTYEO1_W { w: self }
    }
    #[doc = "Bit 4 - Enable Invertion of DAC 0 input event"]
    #[inline(always)]
    pub fn invei0(&mut self) -> INVEI0_W {
        INVEI0_W { w: self }
    }
    #[doc = "Bit 5 - Enable Invertion of DAC 1 input event"]
    #[inline(always)]
    pub fn invei1(&mut self) -> INVEI1_W {
        INVEI1_W { w: self }
    }
    #[doc = "Bit 6 - Result Ready Event Output 0"]
    #[inline(always)]
    pub fn resrdyeo0(&mut self) -> RESRDYEO0_W {
        RESRDYEO0_W { w: self }
    }
    #[doc = "Bit 7 - Result Ready Event Output 1"]
    #[inline(always)]
    pub fn resrdyeo1(&mut self) -> RESRDYEO1_W {
        RESRDYEO1_W { w: self }
    }
}
