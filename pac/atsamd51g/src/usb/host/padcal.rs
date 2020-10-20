#[doc = "Reader of register PADCAL"]
pub type R = crate::R<u16, super::PADCAL>;
#[doc = "Writer for register PADCAL"]
pub type W = crate::W<u16, super::PADCAL>;
#[doc = "Register PADCAL `reset()`'s with value 0"]
impl crate::ResetValue for super::PADCAL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRANSP`"]
pub type TRANSP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRANSP`"]
pub struct TRANSP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `TRANSN`"]
pub type TRANSN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRANSN`"]
pub struct TRANSN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u16) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `TRIM`"]
pub type TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM`"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline(always)]
    pub fn transp(&self) -> TRANSP_R {
        TRANSP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline(always)]
    pub fn transn(&self) -> TRANSN_R {
        TRANSN_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline(always)]
    pub fn transp(&mut self) -> TRANSP_W {
        TRANSP_W { w: self }
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline(always)]
    pub fn transn(&mut self) -> TRANSN_W {
        TRANSN_W { w: self }
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
}
