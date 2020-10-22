#[doc = "Reader of register INTFLAG"]
pub type R = crate::R<u8, super::INTFLAG>;
#[doc = "Writer for register INTFLAG"]
pub type W = crate::W<u8, super::INTFLAG>;
#[doc = "Register INTFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLEEPRDY`"]
pub type SLEEPRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEPRDY`"]
pub struct SLEEPRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPRDY_W<'a> {
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
    #[doc = "Bit 0 - Sleep Mode Entry Ready"]
    #[inline(always)]
    pub fn sleeprdy(&self) -> SLEEPRDY_R {
        SLEEPRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep Mode Entry Ready"]
    #[inline(always)]
    pub fn sleeprdy(&mut self) -> SLEEPRDY_W {
        SLEEPRDY_W { w: self }
    }
}
