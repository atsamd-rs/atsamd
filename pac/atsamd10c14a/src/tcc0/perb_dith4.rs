#[doc = "Reader of register PERB_DITH4"]
pub type R = crate::R<u32, super::PERB_DITH4>;
#[doc = "Writer for register PERB_DITH4"]
pub type W = crate::W<u32, super::PERB_DITH4>;
#[doc = "Register PERB_DITH4 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PERB_DITH4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `DITHERCYB`"]
pub type DITHERCYB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DITHERCYB`"]
pub struct DITHERCYB_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHERCYB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PERB`"]
pub type PERB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERB`"]
pub struct PERB_W<'a> {
    w: &'a mut W,
}
impl<'a> PERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | (((value as u32) & 0x000f_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DITHERCYB_R {
        DITHERCYB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&mut self) -> DITHERCYB_W {
        DITHERCYB_W { w: self }
    }
    #[doc = "Bits 4:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&mut self) -> PERB_W {
        PERB_W { w: self }
    }
}
