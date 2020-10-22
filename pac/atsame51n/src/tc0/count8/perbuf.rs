#[doc = "Reader of register PERBUF"]
pub type R = crate::R<u8, super::PERBUF>;
#[doc = "Writer for register PERBUF"]
pub type W = crate::W<u8, super::PERBUF>;
#[doc = "Register PERBUF `reset()`'s with value 0xff"]
impl crate::ResetValue for super::PERBUF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `PERBUF`"]
pub type PERBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERBUF`"]
pub struct PERBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PERBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PERBUF_R {
        PERBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&mut self) -> PERBUF_W {
        PERBUF_W { w: self }
    }
}
