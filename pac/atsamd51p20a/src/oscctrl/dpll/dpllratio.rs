#[doc = "Reader of register DPLLRATIO"]
pub type R = crate::R<u32, super::DPLLRATIO>;
#[doc = "Writer for register DPLLRATIO"]
pub type W = crate::W<u32, super::DPLLRATIO>;
#[doc = "Register DPLLRATIO `reset()`'s with value 0"]
impl crate::ResetValue for super::DPLLRATIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LDR`"]
pub type LDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LDR`"]
pub struct LDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `LDRFRAC`"]
pub type LDRFRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LDRFRAC`"]
pub struct LDRFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRFRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Loop Divider Ratio"]
    #[inline(always)]
    pub fn ldr(&self) -> LDR_R {
        LDR_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:20 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    pub fn ldrfrac(&self) -> LDRFRAC_R {
        LDRFRAC_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Loop Divider Ratio"]
    #[inline(always)]
    pub fn ldr(&mut self) -> LDR_W {
        LDR_W { w: self }
    }
    #[doc = "Bits 16:20 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    pub fn ldrfrac(&mut self) -> LDRFRAC_W {
        LDRFRAC_W { w: self }
    }
}
