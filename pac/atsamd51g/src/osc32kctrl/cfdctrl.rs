#[doc = "Reader of register CFDCTRL"]
pub type R = crate::R<u8, super::CFDCTRL>;
#[doc = "Writer for register CFDCTRL"]
pub type W = crate::W<u8, super::CFDCTRL>;
#[doc = "Register CFDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CFDCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFDEN`"]
pub type CFDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFDEN`"]
pub struct CFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEN_W<'a> {
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
#[doc = "Reader of field `SWBACK`"]
pub type SWBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWBACK`"]
pub struct SWBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWBACK_W<'a> {
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
#[doc = "Reader of field `CFDPRESC`"]
pub type CFDPRESC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFDPRESC`"]
pub struct CFDPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDPRESC_W<'a> {
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
    #[doc = "Bit 0 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Switch Back"]
    #[inline(always)]
    pub fn swback(&self) -> SWBACK_R {
        SWBACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CFDPRESC_R {
        CFDPRESC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W {
        CFDEN_W { w: self }
    }
    #[doc = "Bit 1 - Clock Switch Back"]
    #[inline(always)]
    pub fn swback(&mut self) -> SWBACK_W {
        SWBACK_W { w: self }
    }
    #[doc = "Bit 2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&mut self) -> CFDPRESC_W {
        CFDPRESC_W { w: self }
    }
}
