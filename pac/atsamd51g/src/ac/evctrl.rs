#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u16, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u16, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPEO0`"]
pub type COMPEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEO0`"]
pub struct COMPEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `COMPEO1`"]
pub type COMPEO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEO1`"]
pub struct COMPEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `WINEO0`"]
pub type WINEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WINEO0`"]
pub struct WINEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> WINEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `COMPEI0`"]
pub type COMPEI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEI0`"]
pub struct COMPEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI0_W<'a> {
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
#[doc = "Reader of field `COMPEI1`"]
pub type COMPEI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEI1`"]
pub struct COMPEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&self) -> COMPEO0_R {
        COMPEO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&self) -> COMPEO1_R {
        COMPEO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&self) -> WINEO0_R {
        WINEO0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&self) -> COMPEI0_R {
        COMPEI0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&self) -> COMPEI1_R {
        COMPEI1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei0(&self) -> INVEI0_R {
        INVEI0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei1(&self) -> INVEI1_R {
        INVEI1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&mut self) -> COMPEO0_W {
        COMPEO0_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&mut self) -> COMPEO1_W {
        COMPEO1_W { w: self }
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&mut self) -> WINEO0_W {
        WINEO0_W { w: self }
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&mut self) -> COMPEI0_W {
        COMPEI0_W { w: self }
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&mut self) -> COMPEI1_W {
        COMPEI1_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei0(&mut self) -> INVEI0_W {
        INVEI0_W { w: self }
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei1(&mut self) -> INVEI1_W {
        INVEI1_W { w: self }
    }
}
