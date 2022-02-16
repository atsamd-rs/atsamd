#[doc = "Register `AFEC_COCR` reader"]
pub struct R(crate::R<AFEC_COCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_COCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_COCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_COCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_COCR` writer"]
pub struct W(crate::W<AFEC_COCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_COCR_SPEC>;
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
impl From<crate::W<AFEC_COCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_COCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AOFF` reader - Analog Offset"]
pub struct AOFF_R(crate::FieldReader<u16, u16>);
impl AOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        AOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AOFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AOFF` writer - Analog Offset"]
pub struct AOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> AOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Analog Offset"]
    #[inline(always)]
    pub fn aoff(&self) -> AOFF_R {
        AOFF_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Analog Offset"]
    #[inline(always)]
    pub fn aoff(&mut self) -> AOFF_W {
        AOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Offset Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cocr](index.html) module"]
pub struct AFEC_COCR_SPEC;
impl crate::RegisterSpec for AFEC_COCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_cocr::R](R) reader structure"]
impl crate::Readable for AFEC_COCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_cocr::W](W) writer structure"]
impl crate::Writable for AFEC_COCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_COCR to value 0"]
impl crate::Resettable for AFEC_COCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
