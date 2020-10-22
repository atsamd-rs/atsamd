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
#[doc = "Reader of field `DATARDYEO`"]
pub type DATARDYEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATARDYEO`"]
pub struct DATARDYEO_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARDYEO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Data Ready Event Output"]
    #[inline(always)]
    pub fn datardyeo(&self) -> DATARDYEO_R {
        DATARDYEO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Ready Event Output"]
    #[inline(always)]
    pub fn datardyeo(&mut self) -> DATARDYEO_W {
        DATARDYEO_W { w: self }
    }
}
