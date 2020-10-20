#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u8, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u8, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATARDY`"]
pub type DATARDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATARDY`"]
pub struct DATARDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARDY_W<'a> {
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
    #[doc = "Bit 0 - Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn datardy(&self) -> DATARDY_R {
        DATARDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn datardy(&mut self) -> DATARDY_W {
        DATARDY_W { w: self }
    }
}
