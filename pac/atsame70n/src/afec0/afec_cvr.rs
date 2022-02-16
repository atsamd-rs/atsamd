#[doc = "Register `AFEC_CVR` reader"]
pub struct R(crate::R<AFEC_CVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_CVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_CVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_CVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_CVR` writer"]
pub struct W(crate::W<AFEC_CVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_CVR_SPEC>;
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
impl From<crate::W<AFEC_CVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_CVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub struct OFFSETCORR_R(crate::FieldReader<u16, u16>);
impl OFFSETCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OFFSETCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSETCORR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSETCORR` writer - Offset Correction"]
pub struct OFFSETCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub struct GAINCORR_R(crate::FieldReader<u16, u16>);
impl GAINCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GAINCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAINCORR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAINCORR` writer - Gain Correction"]
pub struct GAINCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&mut self) -> OFFSETCORR_W {
        OFFSETCORR_W { w: self }
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&mut self) -> GAINCORR_W {
        GAINCORR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Correction Values Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cvr](index.html) module"]
pub struct AFEC_CVR_SPEC;
impl crate::RegisterSpec for AFEC_CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_cvr::R](R) reader structure"]
impl crate::Readable for AFEC_CVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_cvr::W](W) writer structure"]
impl crate::Writable for AFEC_CVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_CVR to value 0"]
impl crate::Resettable for AFEC_CVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
