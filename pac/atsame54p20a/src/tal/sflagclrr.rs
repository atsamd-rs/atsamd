#[doc = "Reader of register SFLAGCLRR%s"]
pub type R = crate::R<u8, super::SFLAGCLRR>;
#[doc = "Writer for register SFLAGCLRR%s"]
pub type W = crate::W<u8, super::SFLAGCLRR>;
#[doc = "Register SFLAGCLRR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::SFLAGCLRR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IPS`"]
pub type IPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS`"]
pub struct IPS_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS_W<'a> {
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
    #[doc = "Bit 0 - Inter-Process Signal n"]
    #[inline(always)]
    pub fn ips(&self) -> IPS_R {
        IPS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Inter-Process Signal n"]
    #[inline(always)]
    pub fn ips(&mut self) -> IPS_W {
        IPS_W { w: self }
    }
}
