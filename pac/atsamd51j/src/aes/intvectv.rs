#[doc = "Register `INTVECTV[%s]` writer"]
pub struct W(crate::W<INTVECTV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTVECTV_SPEC>;
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
impl From<crate::W<INTVECTV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTVECTV_SPEC>) -> Self {
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
#[doc = "Initialisation Vector n\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intvectv](index.html) module"]
pub struct INTVECTV_SPEC;
impl crate::RegisterSpec for INTVECTV_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intvectv::W](W) writer structure"]
impl crate::Writable for INTVECTV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTVECTV[%s]
to value 0"]
impl crate::Resettable for INTVECTV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
