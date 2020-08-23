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
#[doc = "Reader of field `CFDEO0`"]
pub type CFDEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFDEO0`"]
pub struct CFDEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEO0_W<'a> {
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
#[doc = "Reader of field `CFDEO1`"]
pub type CFDEO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFDEO1`"]
pub struct CFDEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEO1_W<'a> {
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
    #[doc = "Bit 0 - Clock 0 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo0(&self) -> CFDEO0_R {
        CFDEO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock 1 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo1(&self) -> CFDEO1_R {
        CFDEO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock 0 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo0(&mut self) -> CFDEO0_W {
        CFDEO0_W { w: self }
    }
    #[doc = "Bit 1 - Clock 1 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo1(&mut self) -> CFDEO1_W {
        CFDEO1_W { w: self }
    }
}
