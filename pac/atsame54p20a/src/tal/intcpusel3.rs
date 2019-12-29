#[doc = "Reader of register INTCPUSEL3"]
pub type R = crate::R<u32, super::INTCPUSEL3>;
#[doc = "Writer for register INTCPUSEL3"]
pub type W = crate::W<u32, super::INTCPUSEL3>;
#[doc = "Register INTCPUSEL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCPUSEL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAMECC`"]
pub type RAMECC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMECC`"]
pub struct RAMECC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMECC_W<'a> {
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
    #[doc = "Bit 0 - RAMECC Interrupt CPU Select"]
    #[inline(always)]
    pub fn ramecc(&self) -> RAMECC_R {
        RAMECC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAMECC Interrupt CPU Select"]
    #[inline(always)]
    pub fn ramecc(&mut self) -> RAMECC_W {
        RAMECC_W { w: self }
    }
}
