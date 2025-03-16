#[doc = "Register `RANDSEED` reader"]
pub struct R(crate::R<RANDSEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANDSEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANDSEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANDSEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RANDSEED` writer"]
pub struct W(crate::W<RANDSEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RANDSEED_SPEC>;
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
impl From<crate::W<RANDSEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RANDSEED_SPEC>) -> Self {
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
#[doc = "Random Seed\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [randseed](index.html) module"]
pub struct RANDSEED_SPEC;
impl crate::RegisterSpec for RANDSEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [randseed::R](R) reader structure"]
impl crate::Readable for RANDSEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [randseed::W](W) writer structure"]
impl crate::Writable for RANDSEED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RANDSEED to value 0"]
impl crate::Resettable for RANDSEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
