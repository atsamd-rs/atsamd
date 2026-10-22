#[doc = "Register `GAINCORR` reader"]
pub struct R(crate::R<GAINCORR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAINCORR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAINCORR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAINCORR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAINCORR` writer"]
pub struct W(crate::W<GAINCORR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAINCORR_SPEC>;
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
impl From<crate::W<GAINCORR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAINCORR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAINCORR` reader - Gain Correction Value"]
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
#[doc = "Field `GAINCORR` writer - Gain Correction Value"]
pub struct GAINCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u16 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Gain Correction Value"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Gain Correction Value"]
    #[inline(always)]
    pub fn gaincorr(&mut self) -> GAINCORR_W {
        GAINCORR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gain Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gaincorr](index.html) module"]
pub struct GAINCORR_SPEC;
impl crate::RegisterSpec for GAINCORR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gaincorr::R](R) reader structure"]
impl crate::Readable for GAINCORR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gaincorr::W](W) writer structure"]
impl crate::Writable for GAINCORR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAINCORR to value 0"]
impl crate::Resettable for GAINCORR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
