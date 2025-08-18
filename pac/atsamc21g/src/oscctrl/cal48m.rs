#[doc = "Register `CAL48M` reader"]
pub struct R(crate::R<CAL48M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL48M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL48M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL48M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL48M` writer"]
pub struct W(crate::W<CAL48M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL48M_SPEC>;
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
impl From<crate::W<CAL48M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL48M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCAL` reader - Frequency Calibration (48MHz)"]
pub struct FCAL_R(crate::FieldReader<u8, u8>);
impl FCAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCAL` writer - Frequency Calibration (48MHz)"]
pub struct FCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `FRANGE` reader - Frequency Range (48MHz)"]
pub struct FRANGE_R(crate::FieldReader<u8, u8>);
impl FRANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRANGE` writer - Frequency Range (48MHz)"]
pub struct FRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `TCAL` reader - Temperature Calibration (48MHz)"]
pub struct TCAL_R(crate::FieldReader<u8, u8>);
impl TCAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCAL` writer - Temperature Calibration (48MHz)"]
pub struct TCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Frequency Calibration (48MHz)"]
    #[inline(always)]
    pub fn fcal(&self) -> FCAL_R {
        FCAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Frequency Range (48MHz)"]
    #[inline(always)]
    pub fn frange(&self) -> FRANGE_R {
        FRANGE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:21 - Temperature Calibration (48MHz)"]
    #[inline(always)]
    pub fn tcal(&self) -> TCAL_R {
        TCAL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frequency Calibration (48MHz)"]
    #[inline(always)]
    pub fn fcal(&mut self) -> FCAL_W {
        FCAL_W { w: self }
    }
    #[doc = "Bits 8:9 - Frequency Range (48MHz)"]
    #[inline(always)]
    pub fn frange(&mut self) -> FRANGE_W {
        FRANGE_W { w: self }
    }
    #[doc = "Bits 16:21 - Temperature Calibration (48MHz)"]
    #[inline(always)]
    pub fn tcal(&mut self) -> TCAL_W {
        TCAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "48MHz Oscillator Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal48m](index.html) module"]
pub struct CAL48M_SPEC;
impl crate::RegisterSpec for CAL48M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal48m::R](R) reader structure"]
impl crate::Readable for CAL48M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal48m::W](W) writer structure"]
impl crate::Writable for CAL48M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL48M to value 0"]
impl crate::Resettable for CAL48M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
