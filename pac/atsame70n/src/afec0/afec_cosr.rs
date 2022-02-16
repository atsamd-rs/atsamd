#[doc = "Register `AFEC_COSR` reader"]
pub struct R(crate::R<AFEC_COSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_COSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_COSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_COSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_COSR` writer"]
pub struct W(crate::W<AFEC_COSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_COSR_SPEC>;
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
impl From<crate::W<AFEC_COSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_COSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL` reader - Sample & Hold unit Correction Select"]
pub struct CSEL_R(crate::FieldReader<bool, bool>);
impl CSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL` writer - Sample & Hold unit Correction Select"]
pub struct CSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sample & Hold unit Correction Select"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample & Hold unit Correction Select"]
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
#[doc = "AFEC Correction Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cosr](index.html) module"]
pub struct AFEC_COSR_SPEC;
impl crate::RegisterSpec for AFEC_COSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_cosr::R](R) reader structure"]
impl crate::Readable for AFEC_COSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_cosr::W](W) writer structure"]
impl crate::Writable for AFEC_COSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_COSR to value 0"]
impl crate::Resettable for AFEC_COSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
