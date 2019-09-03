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
#[doc = "Reader of field `CKRDY`"]
pub type CKRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKRDY`"]
pub struct CKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CKRDY_W<'a> {
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
    #[doc = "Bit 0 - Clock Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ckrdy(&self) -> CKRDY_R {
        CKRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ckrdy(&mut self) -> CKRDY_W {
        CKRDY_W { w: self }
    }
}
