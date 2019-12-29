#[doc = "Reader of register TPQ"]
pub type R = crate::R<u32, super::TPQ>;
#[doc = "Writer for register TPQ"]
pub type W = crate::W<u32, super::TPQ>;
#[doc = "Register TPQ `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::TPQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `TPQ`"]
pub type TPQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TPQ`"]
pub struct TPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Pause Quantum"]
    #[inline(always)]
    pub fn tpq(&self) -> TPQ_R {
        TPQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Pause Quantum"]
    #[inline(always)]
    pub fn tpq(&mut self) -> TPQ_W {
        TPQ_W { w: self }
    }
}
