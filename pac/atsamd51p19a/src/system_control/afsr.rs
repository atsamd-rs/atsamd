#[doc = "Reader of register AFSR"]
pub type R = crate::R<u32, super::AFSR>;
#[doc = "Writer for register AFSR"]
pub type W = crate::W<u32, super::AFSR>;
#[doc = "Register AFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMPDEF`"]
pub type IMPDEF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IMPDEF`"]
pub struct IMPDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IMPDEF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AUXFAULT input signals"]
    #[inline(always)]
    pub fn impdef(&self) -> IMPDEF_R {
        IMPDEF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AUXFAULT input signals"]
    #[inline(always)]
    pub fn impdef(&mut self) -> IMPDEF_W {
        IMPDEF_W { w: self }
    }
}
