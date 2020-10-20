#[doc = "Reader of register HASH"]
pub type R = crate::R<u32, super::HASH>;
#[doc = "Writer for register HASH"]
pub type W = crate::W<u32, super::HASH>;
#[doc = "Register HASH `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HASA`"]
pub type HASA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HASA`"]
pub struct HASA_W<'a> {
    w: &'a mut W,
}
impl<'a> HASA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - Hash Area Start Address"]
    #[inline(always)]
    pub fn hasa(&self) -> HASA_R {
        HASA_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31 - Hash Area Start Address"]
    #[inline(always)]
    pub fn hasa(&mut self) -> HASA_W {
        HASA_W { w: self }
    }
}
