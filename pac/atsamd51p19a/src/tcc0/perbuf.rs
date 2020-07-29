#[doc = "Reader of register PERBUF"]
pub type R = crate::R<u32, super::PERBUF>;
#[doc = "Writer for register PERBUF"]
pub type W = crate::W<u32, super::PERBUF>;
#[doc = "Register PERBUF `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PERBUF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PERBUF`"]
pub type PERBUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERBUF`"]
pub struct PERBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PERBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PERBUF_R {
        PERBUF_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&mut self) -> PERBUF_W {
        PERBUF_W { w: self }
    }
}
