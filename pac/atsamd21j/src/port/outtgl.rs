#[doc = "Register `OUTTGL%s` reader"]
pub struct R(crate::R<OUTTGL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTTGL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTTGL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTTGL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTTGL%s` writer"]
pub struct W(crate::W<OUTTGL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTTGL_SPEC>;
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
impl From<crate::W<OUTTGL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTTGL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTTGL` reader - Port Data Output Value Toggle"]
pub type OUTTGL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUTTGL` writer - Port Data Output Value Toggle"]
pub type OUTTGL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTTGL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Toggle"]
    #[inline(always)]
    pub fn outtgl(&self) -> OUTTGL_R {
        OUTTGL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn outtgl(&mut self) -> OUTTGL_W<0> {
        OUTTGL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Output Value Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outtgl](index.html) module"]
pub struct OUTTGL_SPEC;
impl crate::RegisterSpec for OUTTGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outtgl::R](R) reader structure"]
impl crate::Readable for OUTTGL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outtgl::W](W) writer structure"]
impl crate::Writable for OUTTGL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTTGL%s to value 0"]
impl crate::Resettable for OUTTGL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
