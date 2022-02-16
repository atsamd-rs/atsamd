#[doc = "Register `AFEC_TEMPCWR` reader"]
pub struct R(crate::R<AFEC_TEMPCWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_TEMPCWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_TEMPCWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_TEMPCWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_TEMPCWR` writer"]
pub struct W(crate::W<AFEC_TEMPCWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_TEMPCWR_SPEC>;
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
impl From<crate::W<AFEC_TEMPCWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_TEMPCWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLOWTHRES` reader - Temperature Low Threshold"]
pub struct TLOWTHRES_R(crate::FieldReader<u16, u16>);
impl TLOWTHRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TLOWTHRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLOWTHRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLOWTHRES` writer - Temperature Low Threshold"]
pub struct TLOWTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `THIGHTHRES` reader - Temperature High Threshold"]
pub struct THIGHTHRES_R(crate::FieldReader<u16, u16>);
impl THIGHTHRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        THIGHTHRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THIGHTHRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THIGHTHRES` writer - Temperature High Threshold"]
pub struct THIGHTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> THIGHTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&self) -> TLOWTHRES_R {
        TLOWTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&self) -> THIGHTHRES_R {
        THIGHTHRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&mut self) -> TLOWTHRES_W {
        TLOWTHRES_W { w: self }
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&mut self) -> THIGHTHRES_W {
        THIGHTHRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Temperature Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_tempcwr](index.html) module"]
pub struct AFEC_TEMPCWR_SPEC;
impl crate::RegisterSpec for AFEC_TEMPCWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_tempcwr::R](R) reader structure"]
impl crate::Readable for AFEC_TEMPCWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_tempcwr::W](W) writer structure"]
impl crate::Writable for AFEC_TEMPCWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_TEMPCWR to value 0"]
impl crate::Resettable for AFEC_TEMPCWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
