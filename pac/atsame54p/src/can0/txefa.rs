#[doc = "Reader of register TXEFA"]
pub type R = crate::R<u32, super::TXEFA>;
#[doc = "Writer for register TXEFA"]
pub type W = crate::W<u32, super::TXEFA>;
#[doc = "Register TXEFA `reset()`'s with value 0"]
impl crate::ResetValue for super::TXEFA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFAI`"]
pub type EFAI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFAI`"]
pub struct EFAI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFAI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&mut self) -> EFAI_W {
        EFAI_W { w: self }
    }
}
