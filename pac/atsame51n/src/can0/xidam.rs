#[doc = "Reader of register XIDAM"]
pub type R = crate::R<u32, super::XIDAM>;
#[doc = "Writer for register XIDAM"]
pub type W = crate::W<u32, super::XIDAM>;
#[doc = "Register XIDAM `reset()`'s with value 0x1fff_ffff"]
impl crate::ResetValue for super::XIDAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1fff_ffff
    }
}
#[doc = "Reader of field `EIDM`"]
pub type EIDM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EIDM`"]
pub struct EIDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EIDM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&mut self) -> EIDM_W {
        EIDM_W { w: self }
    }
}
