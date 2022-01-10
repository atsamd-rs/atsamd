#[doc = "Register `PFR[%s]` reader"]
pub struct R(crate::R<PFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFR[%s]` writer"]
pub struct W(crate::W<PFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFR_SPEC>;
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
impl From<crate::W<PFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFR_SPEC>) -> Self {
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
#[doc = "Processor Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfr](index.html) module"]
pub struct PFR_SPEC;
impl crate::RegisterSpec for PFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfr::R](R) reader structure"]
impl crate::Readable for PFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfr::W](W) writer structure"]
impl crate::Writable for PFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFR[%s]
to value 0"]
impl crate::Resettable for PFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
