#[doc = "Reader of register RXPL"]
pub type R = crate::R<u8, super::RXPL>;
#[doc = "Writer for register RXPL"]
pub type W = crate::W<u8, super::RXPL>;
#[doc = "Register RXPL `reset()`'s with value 0"]
impl crate::ResetValue for super::RXPL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXPL`"]
pub type RXPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXPL`"]
pub struct RXPL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Pulse Length"]
    #[inline(always)]
    pub fn rxpl(&self) -> RXPL_R {
        RXPL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Pulse Length"]
    #[inline(always)]
    pub fn rxpl(&mut self) -> RXPL_W {
        RXPL_W { w: self }
    }
}
