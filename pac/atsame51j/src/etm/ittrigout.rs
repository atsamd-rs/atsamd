#[doc = "Register `ITTRIGOUT` writer"]
pub struct W(crate::W<ITTRIGOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITTRIGOUT_SPEC>;
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
impl From<crate::W<ITTRIGOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITTRIGOUT_SPEC>) -> Self {
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
#[doc = "ETM Integration Test Trigger Out\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ittrigout](index.html) module"]
pub struct ITTRIGOUT_SPEC;
impl crate::RegisterSpec for ITTRIGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ittrigout::W](W) writer structure"]
impl crate::Writable for ITTRIGOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ITTRIGOUT to value 0"]
impl crate::Resettable for ITTRIGOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
