#[doc = "Register `LAR` writer"]
pub struct W(crate::W<LAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAR_SPEC>;
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
impl From<crate::W<LAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAR_SPEC>) -> Self {
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
#[doc = "ETM Lock Access Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lar](index.html) module"]
pub struct LAR_SPEC;
impl crate::RegisterSpec for LAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lar::W](W) writer structure"]
impl crate::Writable for LAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAR to value 0"]
impl crate::Resettable for LAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
