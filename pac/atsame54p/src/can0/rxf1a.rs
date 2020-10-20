#[doc = "Reader of register RXF1A"]
pub type R = crate::R<u32, super::RXF1A>;
#[doc = "Writer for register RXF1A"]
pub type W = crate::W<u32, super::RXF1A>;
#[doc = "Register RXF1A `reset()`'s with value 0"]
impl crate::ResetValue for super::RXF1A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F1AI`"]
pub type F1AI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F1AI`"]
pub struct F1AI_W<'a> {
    w: &'a mut W,
}
impl<'a> F1AI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index"]
    #[inline(always)]
    pub fn f1ai(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index"]
    #[inline(always)]
    pub fn f1ai(&mut self) -> F1AI_W {
        F1AI_W { w: self }
    }
}
