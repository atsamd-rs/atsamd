#[doc = "Register `KEYWORD%s` writer"]
pub struct W(crate::W<KEYWORD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYWORD_SPEC>;
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
impl From<crate::W<KEYWORD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYWORD_SPEC>) -> Self {
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
#[doc = "Keyword n\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyword](index.html) module"]
pub struct KEYWORD_SPEC;
impl crate::RegisterSpec for KEYWORD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [keyword::W](W) writer structure"]
impl crate::Writable for KEYWORD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYWORD%s to value 0"]
impl crate::Resettable for KEYWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
