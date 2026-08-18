#[doc = "Register `CCBUF%s` reader"]
pub struct R(crate::R<CCBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCBUF%s` writer"]
pub struct W(crate::W<CCBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCBUF_SPEC>;
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
impl From<crate::W<CCBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCBUF` reader - Counter/Compare Buffer Value"]
pub type CCBUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCBUF` writer - Counter/Compare Buffer Value"]
pub type CCBUF_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CCBUF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Counter/Compare Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CCBUF_R {
        CCBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Compare Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccbuf(&mut self) -> CCBUF_W<0> {
        CCBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COUNT8 Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccbuf](index.html) module"]
pub struct CCBUF_SPEC;
impl crate::RegisterSpec for CCBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ccbuf::R](R) reader structure"]
impl crate::Readable for CCBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccbuf::W](W) writer structure"]
impl crate::Writable for CCBUF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCBUF%s to value 0"]
impl crate::Resettable for CCBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
