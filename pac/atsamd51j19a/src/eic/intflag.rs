#[doc = "Reader of register INTFLAG"]
pub type R = crate::R<u32, super::INTFLAG>;
#[doc = "Writer for register INTFLAG"]
pub type W = crate::W<u32, super::INTFLAG>;
#[doc = "Register INTFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTINT`"]
pub type EXTINT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EXTINT`"]
pub struct EXTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - External Interrupt"]
    #[inline(always)]
    pub fn extint(&self) -> EXTINT_R {
        EXTINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt"]
    #[inline(always)]
    pub fn extint(&mut self) -> EXTINT_W {
        EXTINT_W { w: self }
    }
}
