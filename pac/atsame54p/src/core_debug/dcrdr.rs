#[doc = "Register `DCRDR` reader"]
pub struct R(crate::R<DCRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCRDR` writer"]
pub struct W(crate::W<DCRDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRDR_SPEC>;
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
impl From<crate::W<DCRDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCRDR_SPEC>) -> Self {
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
#[doc = "Debug Core Register Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrdr](index.html) module"]
pub struct DCRDR_SPEC;
impl crate::RegisterSpec for DCRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcrdr::R](R) reader structure"]
impl crate::Readable for DCRDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcrdr::W](W) writer structure"]
impl crate::Writable for DCRDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCRDR to value 0"]
impl crate::Resettable for DCRDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
