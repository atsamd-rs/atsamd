#[doc = "Reader of register DRVCTRL"]
pub type R = crate::R<u8, super::DRVCTRL>;
#[doc = "Writer for register DRVCTRL"]
pub type W = crate::W<u8, super::DRVCTRL>;
#[doc = "Register DRVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DRVCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INVEN0`"]
pub type INVEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEN0`"]
pub struct INVEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN0_W<'a> {
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
#[doc = "Reader of field `INVEN1`"]
pub type INVEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEN1`"]
pub struct INVEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN1_W<'a> {
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
    #[doc = "Bit 0 - Output Waveform Invert Enable 0"]
    #[inline(always)]
    pub fn inven0(&self) -> INVEN0_R {
        INVEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Waveform Invert Enable 1"]
    #[inline(always)]
    pub fn inven1(&self) -> INVEN1_R {
        INVEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Waveform Invert Enable 0"]
    #[inline(always)]
    pub fn inven0(&mut self) -> INVEN0_W {
        INVEN0_W { w: self }
    }
    #[doc = "Bit 1 - Output Waveform Invert Enable 1"]
    #[inline(always)]
    pub fn inven1(&mut self) -> INVEN1_W {
        INVEN1_W { w: self }
    }
}
