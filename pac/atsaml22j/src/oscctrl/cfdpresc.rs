#[doc = "Register `CFDPRESC` reader"]
pub struct R(crate::R<CFDPRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDPRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDPRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDPRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDPRESC` writer"]
pub struct W(crate::W<CFDPRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDPRESC_SPEC>;
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
impl From<crate::W<CFDPRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDPRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFDPRESC` reader - Clock Failure Detector Prescaler"]
pub type CFDPRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFDPRESC` writer - Clock Failure Detector Prescaler"]
pub type CFDPRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CFDPRESC_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CFDPRESC_R {
        CFDPRESC_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn cfdpresc(&mut self) -> CFDPRESC_W<0> {
        CFDPRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cloc Failure Detector Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdpresc](index.html) module"]
pub struct CFDPRESC_SPEC;
impl crate::RegisterSpec for CFDPRESC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cfdpresc::R](R) reader structure"]
impl crate::Readable for CFDPRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdpresc::W](W) writer structure"]
impl crate::Writable for CFDPRESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDPRESC to value 0"]
impl crate::Resettable for CFDPRESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
