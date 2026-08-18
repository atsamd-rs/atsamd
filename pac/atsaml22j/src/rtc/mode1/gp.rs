#[doc = "Register `GP%s` reader"]
pub struct R(crate::R<GP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP%s` writer"]
pub struct W(crate::W<GP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_SPEC>;
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
impl From<crate::W<GP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GP` reader - General Purpose"]
pub type GP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GP` writer - General Purpose"]
pub type GP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - General Purpose"]
    #[inline(always)]
    pub fn gp(&self) -> GP_R {
        GP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose"]
    #[inline(always)]
    #[must_use]
    pub fn gp(&mut self) -> GP_W<0> {
        GP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp](index.html) module"]
pub struct GP_SPEC;
impl crate::RegisterSpec for GP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp::R](R) reader structure"]
impl crate::Readable for GP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp::W](W) writer structure"]
impl crate::Writable for GP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GP%s to value 0"]
impl crate::Resettable for GP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
