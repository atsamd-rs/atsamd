#[doc = "Reader of register WINUT"]
pub type R = crate::R<u16, super::WINUT>;
#[doc = "Writer for register WINUT"]
pub type W = crate::W<u16, super::WINUT>;
#[doc = "Register WINUT `reset()`'s with value 0"]
impl crate::ResetValue for super::WINUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WINUT`"]
pub type WINUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WINUT`"]
pub struct WINUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WINUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Window Upper Threshold"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Upper Threshold"]
    #[inline(always)]
    pub fn winut(&mut self) -> WINUT_W {
        WINUT_W { w: self }
    }
}
