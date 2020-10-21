#[doc = "Reader of register CALIB"]
pub type R = crate::R<u16, super::CALIB>;
#[doc = "Writer for register CALIB"]
pub type W = crate::W<u16, super::CALIB>;
#[doc = "Register CALIB `reset()`'s with value 0"]
impl crate::ResetValue for super::CALIB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BIASCOMP`"]
pub type BIASCOMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIASCOMP`"]
pub struct BIASCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASCOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BIASR2R`"]
pub type BIASR2R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIASR2R`"]
pub struct BIASR2R_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASR2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `BIASREFBUF`"]
pub type BIASREFBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIASREFBUF`"]
pub struct BIASREFBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASREFBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline(always)]
    pub fn biascomp(&self) -> BIASCOMP_R {
        BIASCOMP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Bias R2R Ampli scaling"]
    #[inline(always)]
    pub fn biasr2r(&self) -> BIASR2R_R {
        BIASR2R_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline(always)]
    pub fn biasrefbuf(&self) -> BIASREFBUF_R {
        BIASREFBUF_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline(always)]
    pub fn biascomp(&mut self) -> BIASCOMP_W {
        BIASCOMP_W { w: self }
    }
    #[doc = "Bits 4:6 - Bias R2R Ampli scaling"]
    #[inline(always)]
    pub fn biasr2r(&mut self) -> BIASR2R_W {
        BIASR2R_W { w: self }
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline(always)]
    pub fn biasrefbuf(&mut self) -> BIASREFBUF_W {
        BIASREFBUF_W { w: self }
    }
}
