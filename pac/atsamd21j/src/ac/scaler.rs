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
#[doc = "Field `VALUE` writer - Scaler Value"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u8 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Scaler Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Scaler Value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
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
}
#[doc = "`reset()` method sets SCALER%s to value 0"]
impl crate::Resettable for SCALER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
