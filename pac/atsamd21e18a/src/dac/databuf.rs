#[doc = "Reader of register DATABUF"]
pub type R = crate::R<u16, super::DATABUF>;
#[doc = "Writer for register DATABUF"]
pub type W = crate::W<u16, super::DATABUF>;
#[doc = "Register DATABUF `reset()`'s with value 0"]
impl crate::ResetValue for super::DATABUF {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATABUF`"]
pub type DATABUF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATABUF`"]
pub struct DATABUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data Buffer"]
    #[inline(always)]
    pub fn databuf(&self) -> DATABUF_R {
        DATABUF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Buffer"]
    #[inline(always)]
    pub fn databuf(&mut self) -> DATABUF_W {
        DATABUF_W { w: self }
    }
}
