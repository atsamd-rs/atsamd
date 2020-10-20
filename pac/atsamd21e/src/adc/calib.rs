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
#[doc = "Reader of field `LINEARITY_CAL`"]
pub type LINEARITY_CAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINEARITY_CAL`"]
pub struct LINEARITY_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LINEARITY_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BIAS_CAL`"]
pub type BIAS_CAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIAS_CAL`"]
pub struct BIAS_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Linearity Calibration Value"]
    #[inline(always)]
    pub fn linearity_cal(&self) -> LINEARITY_CAL_R {
        LINEARITY_CAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Bias Calibration Value"]
    #[inline(always)]
    pub fn bias_cal(&self) -> BIAS_CAL_R {
        BIAS_CAL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linearity Calibration Value"]
    #[inline(always)]
    pub fn linearity_cal(&mut self) -> LINEARITY_CAL_W {
        LINEARITY_CAL_W { w: self }
    }
    #[doc = "Bits 8:10 - Bias Calibration Value"]
    #[inline(always)]
    pub fn bias_cal(&mut self) -> BIAS_CAL_W {
        BIAS_CAL_W { w: self }
    }
}
