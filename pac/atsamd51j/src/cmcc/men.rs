#[doc = "Reader of register MEN"]
pub type R = crate::R<u32, super::MEN>;
#[doc = "Writer for register MEN"]
pub type W = crate::W<u32, super::MEN>;
#[doc = "Register MEN `reset()`'s with value 0"]
impl crate::ResetValue for super::MEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MENABLE`"]
pub type MENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MENABLE`"]
pub struct MENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MENABLE_W<'a> {
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
    #[doc = "Bit 0 - Cache Controller Monitor Enable"]
    #[inline(always)]
    pub fn menable(&self) -> MENABLE_R {
        MENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Controller Monitor Enable"]
    #[inline(always)]
    pub fn menable(&mut self) -> MENABLE_W {
        MENABLE_W { w: self }
    }
}
