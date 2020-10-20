#[doc = "Reader of register PER"]
pub type R = crate::R<u16, super::PER>;
#[doc = "Writer for register PER"]
pub type W = crate::W<u16, super::PER>;
#[doc = "Register PER `reset()`'s with value 0"]
impl crate::ResetValue for super::PER {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PER`"]
pub type PER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PER`"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Period"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
}
