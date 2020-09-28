#[doc = "Reader of register FILTERBUF"]
pub type R = crate::R<u8, super::FILTERBUF>;
#[doc = "Writer for register FILTERBUF"]
pub type W = crate::W<u8, super::FILTERBUF>;
#[doc = "Register FILTERBUF `reset()`'s with value 0"]
impl crate::ResetValue for super::FILTERBUF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILTERBUF`"]
pub type FILTERBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTERBUF`"]
pub struct FILTERBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Filter Buffer Value"]
    #[inline(always)]
    pub fn filterbuf(&self) -> FILTERBUF_R {
        FILTERBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Buffer Value"]
    #[inline(always)]
    pub fn filterbuf(&mut self) -> FILTERBUF_W {
        FILTERBUF_W { w: self }
    }
}
