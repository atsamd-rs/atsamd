#[doc = "Register `SCALER%s` reader"]
pub struct R(crate::R<SCALER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCALER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCALER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCALER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCALER%s` writer"]
pub struct W(crate::W<SCALER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCALER_SPEC>;
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
impl From<crate::W<SCALER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCALER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Scaler Value"]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VALUE` writer - Scaler Value"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SCALER_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Scaler Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Scaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scaler n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scaler](index.html) module"]
pub struct SCALER_SPEC;
impl crate::RegisterSpec for SCALER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scaler::R](R) reader structure"]
impl crate::Readable for SCALER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scaler::W](W) writer structure"]
impl crate::Writable for SCALER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCALER%s to value 0"]
impl crate::Resettable for SCALER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
