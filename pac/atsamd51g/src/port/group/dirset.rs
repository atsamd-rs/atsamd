#[doc = "Reader of register DIRSET"]
pub type R = crate::R<u32, super::DIRSET>;
#[doc = "Writer for register DIRSET"]
pub type W = crate::W<u32, super::DIRSET>;
#[doc = "Register DIRSET `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIRSET`"]
pub type DIRSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIRSET`"]
pub struct DIRSET_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Set"]
    #[inline(always)]
    pub fn dirset(&self) -> DIRSET_R {
        DIRSET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Set"]
    #[inline(always)]
    pub fn dirset(&mut self) -> DIRSET_W {
        DIRSET_W { w: self }
    }
}
