#[doc = "Register `PER` reader"]
pub struct R(crate::R<PER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PER` writer"]
pub struct W(crate::W<PER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_SPEC>;
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
impl From<crate::W<PER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER` reader - Counter Period"]
pub type PER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PER` writer - Counter Period"]
pub type PER_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Counter Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Period"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<0> {
        PER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE1 Counter Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per](index.html) module"]
pub struct PER_SPEC;
impl crate::RegisterSpec for PER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [per::R](R) reader structure"]
impl crate::Readable for PER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per::W](W) writer structure"]
impl crate::Writable for PER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PER to value 0"]
impl crate::Resettable for PER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
