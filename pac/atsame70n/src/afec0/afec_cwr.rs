#[doc = "Register `AFEC_CWR` reader"]
pub struct R(crate::R<AFEC_CWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_CWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_CWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_CWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_CWR` writer"]
pub struct W(crate::W<AFEC_CWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_CWR_SPEC>;
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
impl From<crate::W<AFEC_CWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_CWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWTHRES` reader - Low Threshold"]
pub struct LOWTHRES_R(crate::FieldReader<u16, u16>);
impl LOWTHRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LOWTHRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWTHRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWTHRES` writer - Low Threshold"]
pub struct LOWTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `HIGHTHRES` reader - High Threshold"]
pub struct HIGHTHRES_R(crate::FieldReader<u16, u16>);
impl HIGHTHRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HIGHTHRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGHTHRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGHTHRES` writer - High Threshold"]
pub struct HIGHTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&self) -> LOWTHRES_R {
        LOWTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&self) -> HIGHTHRES_R {
        HIGHTHRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&mut self) -> LOWTHRES_W {
        LOWTHRES_W { w: self }
    }
    #[doc = "Bits 16:31 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&mut self) -> HIGHTHRES_W {
        HIGHTHRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cwr](index.html) module"]
pub struct AFEC_CWR_SPEC;
impl crate::RegisterSpec for AFEC_CWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_cwr::R](R) reader structure"]
impl crate::Readable for AFEC_CWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_cwr::W](W) writer structure"]
impl crate::Writable for AFEC_CWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_CWR to value 0"]
impl crate::Resettable for AFEC_CWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
