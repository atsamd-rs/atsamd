#[doc = "Reader of register IPGS"]
pub type R = crate::R<u32, super::IPGS>;
#[doc = "Writer for register IPGS"]
pub type W = crate::W<u32, super::IPGS>;
#[doc = "Register IPGS `reset()`'s with value 0"]
impl crate::ResetValue for super::IPGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FL`"]
pub type FL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FL`"]
pub struct FL_W<'a> {
    w: &'a mut W,
}
impl<'a> FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    pub fn fl(&self) -> FL_R {
        FL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    pub fn fl(&mut self) -> FL_W {
        FL_W { w: self }
    }
}
