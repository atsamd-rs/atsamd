#[doc = "Register `OUTCLR` reader"]
pub struct R(crate::R<OUTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTCLR` writer"]
pub struct W(crate::W<OUTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTCLR_SPEC>;
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
impl From<crate::W<OUTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTCLR_SPEC>) -> Self {
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
#[doc = "Data Output Value Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outclr](index.html) module"]
pub struct OUTCLR_SPEC;
impl crate::RegisterSpec for OUTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outclr::R](R) reader structure"]
impl crate::Readable for OUTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outclr::W](W) writer structure"]
impl crate::Writable for OUTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTCLR to value 0"]
impl crate::Resettable for OUTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
