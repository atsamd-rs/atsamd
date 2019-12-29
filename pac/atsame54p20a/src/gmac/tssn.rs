#[doc = "Reader of register TSSN"]
pub type R = crate::R<u32, super::TSSN>;
#[doc = "Writer for register TSSN"]
pub type W = crate::W<u32, super::TSSN>;
#[doc = "Register TSSN `reset()`'s with value 0"]
impl crate::ResetValue for super::TSSN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VTN`"]
pub type VTN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VTN`"]
pub struct VTN_W<'a> {
    w: &'a mut W,
}
impl<'a> VTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Value Timer Nanoseconds Register Capture"]
    #[inline(always)]
    pub fn vtn(&self) -> VTN_R {
        VTN_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Value Timer Nanoseconds Register Capture"]
    #[inline(always)]
    pub fn vtn(&mut self) -> VTN_W {
        VTN_W { w: self }
    }
}
