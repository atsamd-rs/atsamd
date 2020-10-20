#[doc = "Reader of register UR"]
pub type R = crate::R<u32, super::UR>;
#[doc = "Writer for register UR"]
pub type W = crate::W<u32, super::UR>;
#[doc = "Register UR `reset()`'s with value 0"]
impl crate::ResetValue for super::UR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MII`"]
pub type MII_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MII`"]
pub struct MII_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_W<'a> {
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
    #[doc = "Bit 0 - MII Mode"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII Mode"]
    #[inline(always)]
    pub fn mii(&mut self) -> MII_W {
        MII_W { w: self }
    }
}
