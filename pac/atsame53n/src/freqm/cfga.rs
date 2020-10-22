#[doc = "Reader of register CFGA"]
pub type R = crate::R<u16, super::CFGA>;
#[doc = "Writer for register CFGA"]
pub type W = crate::W<u16, super::CFGA>;
#[doc = "Register CFGA `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REFNUM`"]
pub type REFNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFNUM`"]
pub struct REFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> REFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&self) -> REFNUM_R {
        REFNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&mut self) -> REFNUM_W {
        REFNUM_W { w: self }
    }
}
