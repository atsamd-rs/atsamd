#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIB` writer"]
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEARITY_CAL` reader - Linearity Calibration Value"]
pub struct LINEARITY_CAL_R(crate::FieldReader<u8, u8>);
impl LINEARITY_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LINEARITY_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINEARITY_CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINEARITY_CAL` writer - Linearity Calibration Value"]
pub struct LINEARITY_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LINEARITY_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `BIAS_CAL` reader - Bias Calibration Value"]
pub struct BIAS_CAL_R(crate::FieldReader<u8, u8>);
impl BIAS_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BIAS_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_CAL` writer - Bias Calibration Value"]
pub struct BIAS_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u16 & 0x07) << 8);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calib::W](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
