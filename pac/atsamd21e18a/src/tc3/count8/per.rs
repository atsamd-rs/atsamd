#[doc = "Reader of register PER"]
pub type R = crate::R<u8, super::PER>;
#[doc = "Writer for register PER"]
pub type W = crate::W<u8, super::PER>;
#[doc = "Register PER `reset()`'s with value 0xff"]
impl crate::ResetValue for super::PER {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `PER`"]
pub type PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER`"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Period Value"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
}
