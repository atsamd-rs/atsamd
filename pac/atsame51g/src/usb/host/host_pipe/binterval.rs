#[doc = "Reader of register BINTERVAL"]
pub type R = crate::R<u8, super::BINTERVAL>;
#[doc = "Writer for register BINTERVAL"]
pub type W = crate::W<u8, super::BINTERVAL>;
#[doc = "Register BINTERVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::BINTERVAL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BITINTERVAL`"]
pub type BITINTERVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BITINTERVAL`"]
pub struct BITINTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BITINTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bit Interval"]
    #[inline(always)]
    pub fn bitinterval(&self) -> BITINTERVAL_R {
        BITINTERVAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit Interval"]
    #[inline(always)]
    pub fn bitinterval(&mut self) -> BITINTERVAL_W {
        BITINTERVAL_W { w: self }
    }
}
