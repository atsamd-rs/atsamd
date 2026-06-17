#[doc = "Register `WINLT` reader"]
pub struct R(crate::R<WINLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINLT` writer"]
pub struct W(crate::W<WINLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINLT_SPEC>;
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
impl From<crate::W<WINLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINLT` reader - Window Lower Threshold"]
pub type WINLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WINLT` writer - Window Lower Threshold"]
pub type WINLT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WINLT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn winlt(&mut self) -> WINLT_W<0> {
        WINLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Monitor Lower Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winlt](index.html) module"]
pub struct WINLT_SPEC;
impl crate::RegisterSpec for WINLT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [winlt::R](R) reader structure"]
impl crate::Readable for WINLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winlt::W](W) writer structure"]
impl crate::Writable for WINLT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WINLT to value 0"]
impl crate::Resettable for WINLT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
