#[doc = "Reader of register TISUBN"]
pub type R = crate::R<u32, super::TISUBN>;
#[doc = "Writer for register TISUBN"]
pub type W = crate::W<u32, super::TISUBN>;
#[doc = "Register TISUBN `reset()`'s with value 0"]
impl crate::ResetValue for super::TISUBN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSBTIR`"]
pub type LSBTIR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LSBTIR`"]
pub struct LSBTIR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBTIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Lower Significant Bits of Timer Increment"]
    #[inline(always)]
    pub fn lsbtir(&self) -> LSBTIR_R {
        LSBTIR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower Significant Bits of Timer Increment"]
    #[inline(always)]
    pub fn lsbtir(&mut self) -> LSBTIR_W {
        LSBTIR_W { w: self }
    }
}
