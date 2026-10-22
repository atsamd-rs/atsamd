#[doc = "Register `HASHKEY%s` reader"]
pub struct R(crate::R<HASHKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHKEY%s` writer"]
pub struct W(crate::W<HASHKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHKEY_SPEC>;
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
impl From<crate::W<HASHKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHKEY_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash key n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashkey](index.html) module"]
pub struct HASHKEY_SPEC;
impl crate::RegisterSpec for HASHKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashkey::R](R) reader structure"]
impl crate::Readable for HASHKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashkey::W](W) writer structure"]
impl crate::Writable for HASHKEY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASHKEY%s to value 0"]
impl crate::Resettable for HASHKEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
