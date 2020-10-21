#[doc = "Reader of register ASAR[%s]"]
pub type R = crate::R<u32, super::ASAR>;
#[doc = "Writer for register ASAR[%s]"]
pub type W = crate::W<u32, super::ASAR>;
#[doc = "Register ASAR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::ASAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADMASA`"]
pub type ADMASA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADMASA`"]
pub struct ADMASA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMASA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn admasa(&self) -> ADMASA_R {
        ADMASA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn admasa(&mut self) -> ADMASA_W {
        ADMASA_W { w: self }
    }
}
