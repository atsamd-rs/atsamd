#[doc = "Reader of register DIRCLR"]
pub type R = crate::R<u32, super::DIRCLR>;
#[doc = "Writer for register DIRCLR"]
pub type W = crate::W<u32, super::DIRCLR>;
#[doc = "Register DIRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIRCLR`"]
pub type DIRCLR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIRCLR`"]
pub struct DIRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Clear"]
    #[inline(always)]
    pub fn dirclr(&self) -> DIRCLR_R {
        DIRCLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Clear"]
    #[inline(always)]
    pub fn dirclr(&mut self) -> DIRCLR_W {
        DIRCLR_W { w: self }
    }
}
