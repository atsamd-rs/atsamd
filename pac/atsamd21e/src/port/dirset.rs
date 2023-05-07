#[doc = "Register `DIRSET%s` reader"]
pub struct R(crate::R<DIRSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRSET%s` writer"]
pub struct W(crate::W<DIRSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRSET_SPEC>;
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
impl From<crate::W<DIRSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRSET` reader - Port Data Direction Set"]
pub type DIRSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIRSET` writer - Port Data Direction Set"]
pub type DIRSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIRSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Set"]
    #[inline(always)]
    pub fn dirset(&self) -> DIRSET_R {
        DIRSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Set"]
    #[inline(always)]
    #[must_use]
    pub fn dirset(&mut self) -> DIRSET_W<0> {
        DIRSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Direction Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirset](index.html) module"]
pub struct DIRSET_SPEC;
impl crate::RegisterSpec for DIRSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dirset::R](R) reader structure"]
impl crate::Readable for DIRSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dirset::W](W) writer structure"]
impl crate::Writable for DIRSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRSET%s to value 0"]
impl crate::Resettable for DIRSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
