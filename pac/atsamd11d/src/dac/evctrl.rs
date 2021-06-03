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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `EMPTYEO`"]
pub type EMPTYEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMPTYEO`"]
pub struct EMPTYEO_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYEO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Start Conversion Event Input"]
    #[inline(always)]
    pub fn startei(&self) -> STARTEI_R {
        STARTEI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty Event Output"]
    #[inline(always)]
    pub fn emptyeo(&self) -> EMPTYEO_R {
        EMPTYEO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event Input"]
    #[inline(always)]
    pub fn startei(&mut self) -> STARTEI_W {
        STARTEI_W { w: self }
    }
    #[doc = "Bit 1 - Data Buffer Empty Event Output"]
    #[inline(always)]
    pub fn emptyeo(&mut self) -> EMPTYEO_W {
        EMPTYEO_W { w: self }
    }
}
