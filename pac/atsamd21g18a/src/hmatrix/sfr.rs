#[doc = "Reader of register SFR%s"]
pub type R = crate::R<u32, super::SFR>;
#[doc = "Writer for register SFR%s"]
pub type W = crate::W<u32, super::SFR>;
#[doc = "Register SFR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::SFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SFR`"]
pub type SFR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SFR`"]
pub struct SFR_W<'a> {
    w: &'a mut W,
}
impl<'a> SFR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Special Function Register"]
    #[inline(always)]
    pub fn sfr(&self) -> SFR_R {
        SFR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Special Function Register"]
    #[inline(always)]
    pub fn sfr(&mut self) -> SFR_W {
        SFR_W { w: self }
    }
}
