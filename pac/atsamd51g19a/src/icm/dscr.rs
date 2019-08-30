#[doc = "Reader of register DSCR"]
pub type R = crate::R<u32, super::DSCR>;
#[doc = "Writer for register DSCR"]
pub type W = crate::W<u32, super::DSCR>;
#[doc = "Register DSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DASA`"]
pub type DASA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DASA`"]
pub struct DASA_W<'a> {
    w: &'a mut W,
}
impl<'a> DASA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - Descriptor Area Start Address"]
    #[inline(always)]
    pub fn dasa(&self) -> DASA_R {
        DASA_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 6:31 - Descriptor Area Start Address"]
    #[inline(always)]
    pub fn dasa(&mut self) -> DASA_W {
        DASA_W { w: self }
    }
}
