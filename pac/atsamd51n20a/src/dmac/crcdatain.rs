#[doc = "Reader of register CRCDATAIN"]
pub type R = crate::R<u32, super::CRCDATAIN>;
#[doc = "Writer for register CRCDATAIN"]
pub type W = crate::W<u32, super::CRCDATAIN>;
#[doc = "Register CRCDATAIN `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCDATAIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRCDATAIN`"]
pub type CRCDATAIN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRCDATAIN`"]
pub struct CRCDATAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCDATAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    pub fn crcdatain(&self) -> CRCDATAIN_R {
        CRCDATAIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    pub fn crcdatain(&mut self) -> CRCDATAIN_W {
        CRCDATAIN_W { w: self }
    }
}
