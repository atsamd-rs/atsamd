#[doc = "Register `DIRCLR%s` reader"]
pub struct R(crate::R<DIRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRCLR%s` writer"]
pub struct W(crate::W<DIRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRCLR_SPEC>;
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
impl From<crate::W<DIRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRCLR` reader - Port Data Direction Clear"]
pub type DIRCLR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIRCLR` writer - Port Data Direction Clear"]
pub type DIRCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIRCLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Clear"]
    #[inline(always)]
    pub fn dirclr(&self) -> DIRCLR_R {
        DIRCLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dirclr(&mut self) -> DIRCLR_W<0> {
        DIRCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Direction Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirclr](index.html) module"]
pub struct DIRCLR_SPEC;
impl crate::RegisterSpec for DIRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dirclr::R](R) reader structure"]
impl crate::Readable for DIRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dirclr::W](W) writer structure"]
impl crate::Writable for DIRCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRCLR%s to value 0"]
impl crate::Resettable for DIRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
