#[doc = "Reader of register WRBADDR"]
pub type R = crate::R<u32, super::WRBADDR>;
#[doc = "Writer for register WRBADDR"]
pub type W = crate::W<u32, super::WRBADDR>;
#[doc = "Register WRBADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::WRBADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRBADDR`"]
pub type WRBADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WRBADDR`"]
pub struct WRBADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRBADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
    #[inline(always)]
    pub fn wrbaddr(&self) -> WRBADDR_R {
        WRBADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
    #[inline(always)]
    pub fn wrbaddr(&mut self) -> WRBADDR_W {
        WRBADDR_W { w: self }
    }
}
