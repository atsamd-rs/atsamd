#[doc = "Reader of register OFFSETCORR"]
pub type R = crate::R<u16, super::OFFSETCORR>;
#[doc = "Writer for register OFFSETCORR"]
pub type W = crate::W<u16, super::OFFSETCORR>;
#[doc = "Register OFFSETCORR `reset()`'s with value 0"]
impl crate::ResetValue for super::OFFSETCORR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFSETCORR`"]
pub type OFFSETCORR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFFSETCORR`"]
pub struct OFFSETCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u16) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Offset Correction Value"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Offset Correction Value"]
    #[inline(always)]
    pub fn offsetcorr(&mut self) -> OFFSETCORR_W {
        OFFSETCORR_W { w: self }
    }
}
