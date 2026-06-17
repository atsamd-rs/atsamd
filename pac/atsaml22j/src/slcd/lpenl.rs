#[doc = "Register `LPENL` reader"]
pub struct R(crate::R<LPENL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPENL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPENL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPENL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPENL` writer"]
pub struct W(crate::W<LPENL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPENL_SPEC>;
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
impl From<crate::W<LPENL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPENL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPEN` reader - LCD Pin Enable"]
pub type LPEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LPEN` writer - LCD Pin Enable"]
pub type LPEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPENL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - LCD Pin Enable"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LCD Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpen(&mut self) -> LPEN_W<0> {
        LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Pin Enable Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpenl](index.html) module"]
pub struct LPENL_SPEC;
impl crate::RegisterSpec for LPENL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpenl::R](R) reader structure"]
impl crate::Readable for LPENL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpenl::W](W) writer structure"]
impl crate::Writable for LPENL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPENL to value 0"]
impl crate::Resettable for LPENL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
