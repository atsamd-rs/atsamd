#[doc = "Register `FREQCORR` reader"]
pub struct R(crate::R<FREQCORR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQCORR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQCORR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQCORR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQCORR` writer"]
pub struct W(crate::W<FREQCORR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQCORR_SPEC>;
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
impl From<crate::W<FREQCORR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQCORR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Correction Value"]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VALUE` writer - Correction Value"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FREQCORR_SPEC, u8, u8, 7, O>;
#[doc = "Field `SIGN` reader - Correction Sign"]
pub type SIGN_R = crate::BitReader<bool>;
#[doc = "Field `SIGN` writer - Correction Sign"]
pub type SIGN_W<'a, const O: u8> = crate::BitWriter<'a, u8, FREQCORR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Correction Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Correction Sign"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Bit 7 - Correction Sign"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<7> {
        SIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqcorr](index.html) module"]
pub struct FREQCORR_SPEC;
impl crate::RegisterSpec for FREQCORR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [freqcorr::R](R) reader structure"]
impl crate::Readable for FREQCORR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqcorr::W](W) writer structure"]
impl crate::Writable for FREQCORR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQCORR to value 0"]
impl crate::Resettable for FREQCORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
