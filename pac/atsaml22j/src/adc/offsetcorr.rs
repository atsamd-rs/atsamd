#[doc = "Register `OFFSETCORR` reader"]
pub struct R(crate::R<OFFSETCORR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFFSETCORR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFFSETCORR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFFSETCORR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFFSETCORR` writer"]
pub struct W(crate::W<OFFSETCORR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFFSETCORR_SPEC>;
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
impl From<crate::W<OFFSETCORR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFFSETCORR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSETCORR` reader - Offset Correction Value"]
pub type OFFSETCORR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSETCORR` writer - Offset Correction Value"]
pub type OFFSETCORR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, OFFSETCORR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Offset Correction Value"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Offset Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn offsetcorr(&mut self) -> OFFSETCORR_W<0> {
        OFFSETCORR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offsetcorr](index.html) module"]
pub struct OFFSETCORR_SPEC;
impl crate::RegisterSpec for OFFSETCORR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [offsetcorr::R](R) reader structure"]
impl crate::Readable for OFFSETCORR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [offsetcorr::W](W) writer structure"]
impl crate::Writable for OFFSETCORR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFFSETCORR to value 0"]
impl crate::Resettable for OFFSETCORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
