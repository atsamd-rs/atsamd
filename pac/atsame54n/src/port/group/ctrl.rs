#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAMPLING`"]
pub type SAMPLING_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SAMPLING`"]
pub struct SAMPLING_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Input Sampling Mode"]
    #[inline(always)]
    pub fn sampling(&self) -> SAMPLING_R {
        SAMPLING_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Sampling Mode"]
    #[inline(always)]
    pub fn sampling(&mut self) -> SAMPLING_W {
        SAMPLING_W { w: self }
    }
}
