#[doc = "Reader of register HSOFC"]
pub type R = crate::R<u8, super::HSOFC>;
#[doc = "Writer for register HSOFC"]
pub type W = crate::W<u8, super::HSOFC>;
#[doc = "Register HSOFC `reset()`'s with value 0"]
impl crate::ResetValue for super::HSOFC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLENC`"]
pub type FLENC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLENC`"]
pub struct FLENC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLENC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FLENCE`"]
pub type FLENCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLENCE`"]
pub struct FLENCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLENCE_W<'a> {
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
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    pub fn flenc(&self) -> FLENC_R {
        FLENC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    pub fn flence(&self) -> FLENCE_R {
        FLENCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    pub fn flenc(&mut self) -> FLENC_W {
        FLENC_W { w: self }
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    pub fn flence(&mut self) -> FLENCE_W {
        FLENCE_W { w: self }
    }
}
