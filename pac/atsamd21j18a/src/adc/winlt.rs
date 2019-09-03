#[doc = "Reader of register WINLT"]
pub type R = crate::R<u16, super::WINLT>;
#[doc = "Writer for register WINLT"]
pub type W = crate::W<u16, super::WINLT>;
#[doc = "Register WINLT `reset()`'s with value 0"]
impl crate::ResetValue for super::WINLT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WINLT`"]
pub type WINLT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WINLT`"]
pub struct WINLT_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&mut self) -> WINLT_W {
        WINLT_W { w: self }
    }
}
