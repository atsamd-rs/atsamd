#[doc = "Reader of register DFLLVAL"]
pub type R = crate::R<u32, super::DFLLVAL>;
#[doc = "Writer for register DFLLVAL"]
pub type W = crate::W<u32, super::DFLLVAL>;
#[doc = "Register DFLLVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLLVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FINE`"]
pub type FINE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FINE`"]
pub struct FINE_W<'a> {
    w: &'a mut W,
}
impl<'a> FINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `COARSE`"]
pub type COARSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COARSE`"]
pub struct COARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> COARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `DIFF`"]
pub type DIFF_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Fine Value"]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Multiplication Ratio Difference"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Fine Value"]
    #[inline(always)]
    pub fn fine(&mut self) -> FINE_W {
        FINE_W { w: self }
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&mut self) -> COARSE_W {
        COARSE_W { w: self }
    }
}
