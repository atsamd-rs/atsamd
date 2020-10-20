#[doc = "Reader of register PRICTRL"]
pub type R = crate::R<u8, super::PRICTRL>;
#[doc = "Writer for register PRICTRL"]
pub type W = crate::W<u8, super::PRICTRL>;
#[doc = "Register PRICTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PRICTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI`"]
pub type PRI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI`"]
pub struct PRI_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RREN`"]
pub type RREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RREN`"]
pub struct RREN_W<'a> {
    w: &'a mut W,
}
impl<'a> RREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rren(&self) -> RREN_R {
        RREN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    pub fn pri(&mut self) -> PRI_W {
        PRI_W { w: self }
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rren(&mut self) -> RREN_W {
        RREN_W { w: self }
    }
}
