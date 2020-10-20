#[doc = "Reader of register SPPR"]
pub type R = crate::R<u32, super::SPPR>;
#[doc = "Writer for register SPPR"]
pub type W = crate::W<u32, super::SPPR>;
#[doc = "Register SPPR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXMODE`"]
pub type TXMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXMODE`"]
pub struct TXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W {
        TXMODE_W { w: self }
    }
}
