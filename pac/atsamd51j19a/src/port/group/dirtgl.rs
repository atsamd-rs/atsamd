#[doc = "Reader of register DIRTGL"]
pub type R = crate::R<u32, super::DIRTGL>;
#[doc = "Writer for register DIRTGL"]
pub type W = crate::W<u32, super::DIRTGL>;
#[doc = "Register DIRTGL `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRTGL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIRTGL`"]
pub type DIRTGL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIRTGL`"]
pub struct DIRTGL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Toggle"]
    #[inline(always)]
    pub fn dirtgl(&self) -> DIRTGL_R {
        DIRTGL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Toggle"]
    #[inline(always)]
    pub fn dirtgl(&mut self) -> DIRTGL_W {
        DIRTGL_W { w: self }
    }
}
