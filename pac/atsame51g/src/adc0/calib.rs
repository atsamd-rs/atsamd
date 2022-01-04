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
#[doc = "Field `BIASCOMP` reader - Bias Comparator Scaling"]
pub struct BIASCOMP_R(crate::FieldReader<u8, u8>);
impl BIASCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BIASCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIASCOMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIASCOMP` writer - Bias Comparator Scaling"]
pub struct BIASCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASCOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
#[doc = "Field `BIASR2R` reader - Bias R2R Ampli scaling"]
pub struct BIASR2R_R(crate::FieldReader<u8, u8>);
impl BIASR2R_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BIASR2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIASR2R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIASR2R` writer - Bias R2R Ampli scaling"]
pub struct BIASR2R_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASR2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u16 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `BIASREFBUF` reader - Bias Reference Buffer Scaling"]
pub struct BIASREFBUF_R(crate::FieldReader<u8, u8>);
impl BIASREFBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BIASREFBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIASREFBUF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIASREFBUF` writer - Bias Reference Buffer Scaling"]
pub struct BIASREFBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASREFBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u16 & 0x07) << 8);
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
