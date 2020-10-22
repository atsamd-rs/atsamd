#[doc = "Reader of register GAINCORR"]
pub type R = crate::R<u16, super::GAINCORR>;
#[doc = "Writer for register GAINCORR"]
pub type W = crate::W<u16, super::GAINCORR>;
#[doc = "Register GAINCORR `reset()`'s with value 0"]
impl crate::ResetValue for super::GAINCORR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GAINCORR`"]
pub type GAINCORR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GAINCORR`"]
pub struct GAINCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u16) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Gain Correction Value"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Gain Correction Value"]
    #[inline(always)]
    pub fn gaincorr(&mut self) -> GAINCORR_W {
        GAINCORR_W { w: self }
    }
}
