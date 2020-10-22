#[doc = "Reader of register BASEADDR"]
pub type R = crate::R<u32, super::BASEADDR>;
#[doc = "Writer for register BASEADDR"]
pub type W = crate::W<u32, super::BASEADDR>;
#[doc = "Register BASEADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::BASEADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASEADDR`"]
pub type BASEADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASEADDR`"]
pub struct BASEADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    pub fn baseaddr(&mut self) -> BASEADDR_W {
        BASEADDR_W { w: self }
    }
}
