#[doc = "Reader of register EXTCTRL"]
pub type R = crate::R<u8, super::EXTCTRL>;
#[doc = "Writer for register EXTCTRL"]
pub type W = crate::W<u8, super::EXTCTRL>;
#[doc = "Register EXTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SETDIS`"]
pub type SETDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETDIS`"]
pub struct SETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SETDIS_W<'a> {
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
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    pub fn setdis(&self) -> SETDIS_R {
        SETDIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    pub fn setdis(&mut self) -> SETDIS_W {
        SETDIS_W { w: self }
    }
}
