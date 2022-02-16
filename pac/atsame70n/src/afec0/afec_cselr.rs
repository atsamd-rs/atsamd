#[doc = "Register `AFEC_CSELR` reader"]
pub struct R(crate::R<AFEC_CSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_CSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_CSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_CSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_CSELR` writer"]
pub struct W(crate::W<AFEC_CSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_CSELR_SPEC>;
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
impl From<crate::W<AFEC_CSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_CSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL` reader - Channel Selection"]
pub struct CSEL_R(crate::FieldReader<u8, u8>);
impl CSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL` writer - Channel Selection"]
pub struct CSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn csel(&mut self) -> CSEL_W {
        CSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cselr](index.html) module"]
pub struct AFEC_CSELR_SPEC;
impl crate::RegisterSpec for AFEC_CSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_cselr::R](R) reader structure"]
impl crate::Readable for AFEC_CSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_cselr::W](W) writer structure"]
impl crate::Writable for AFEC_CSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_CSELR to value 0"]
impl crate::Resettable for AFEC_CSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
