#[doc = "Reader of register RJFML"]
pub type R = crate::R<u32, super::RJFML>;
#[doc = "Writer for register RJFML"]
pub type W = crate::W<u32, super::RJFML>;
#[doc = "Register RJFML `reset()`'s with value 0x3fff"]
impl crate::ResetValue for super::RJFML {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3fff
    }
}
#[doc = "Reader of field `FML`"]
pub type FML_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FML`"]
pub struct FML_W<'a> {
    w: &'a mut W,
}
impl<'a> FML_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Frame Max Length"]
    #[inline(always)]
    pub fn fml(&self) -> FML_R {
        FML_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Max Length"]
    #[inline(always)]
    pub fn fml(&mut self) -> FML_W {
        FML_W { w: self }
    }
}
