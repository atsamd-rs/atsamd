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
pub struct VALUE_R(crate::FieldReader<u8, u8>);
impl VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALUE` writer - Correction Value"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u8 & 0x7f);
        self.w
    }
}
#[doc = "Field `SIGN` reader - Correction Sign"]
pub struct SIGN_R(crate::FieldReader<bool, bool>);
impl SIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGN` writer - Correction Sign"]
pub struct SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Correction Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Correction Sign"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Correction Value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
    #[doc = "Bit 7 - Correction Sign"]
    #[inline(always)]
    pub fn sign(&mut self) -> SIGN_W {
        SIGN_W { w: self }
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
}
#[doc = "`reset()` method sets FREQCORR to value 0"]
impl crate::Resettable for FREQCORR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
