#[doc = "Register `DEBOUNCEN` reader"]
pub struct R(crate::R<DEBOUNCEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBOUNCEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBOUNCEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBOUNCEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBOUNCEN` writer"]
pub struct W(crate::W<DEBOUNCEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBOUNCEN_SPEC>;
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
impl From<crate::W<DEBOUNCEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBOUNCEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBOUNCEN` reader - Debouncer Enable"]
pub type DEBOUNCEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEBOUNCEN` writer - Debouncer Enable"]
pub type DEBOUNCEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBOUNCEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Debouncer Enable"]
    #[inline(always)]
    pub fn debouncen(&self) -> DEBOUNCEN_R {
        DEBOUNCEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Debouncer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debouncen(&mut self) -> DEBOUNCEN_W<0> {
        DEBOUNCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debouncer Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debouncen](index.html) module"]
pub struct DEBOUNCEN_SPEC;
impl crate::RegisterSpec for DEBOUNCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debouncen::R](R) reader structure"]
impl crate::Readable for DEBOUNCEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debouncen::W](W) writer structure"]
impl crate::Writable for DEBOUNCEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBOUNCEN to value 0"]
impl crate::Resettable for DEBOUNCEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
