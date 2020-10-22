#[doc = "Reader of register ITCTRL"]
pub type R = crate::R<u32, super::ITCTRL>;
#[doc = "Writer for register ITCTRL"]
pub type W = crate::W<u32, super::ITCTRL>;
#[doc = "Register ITCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ITCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTEGRATION`"]
pub type INTEGRATION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEGRATION`"]
pub struct INTEGRATION_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEGRATION_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn integration(&self) -> INTEGRATION_R {
        INTEGRATION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn integration(&mut self) -> INTEGRATION_W {
        INTEGRATION_W { w: self }
    }
}
