#[doc = "Register `TSEVT` reader"]
pub struct R(crate::R<TSEVT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSEVT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSEVT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSEVT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSEVT` writer"]
pub struct W(crate::W<TSEVT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSEVT_SPEC>;
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
impl From<crate::W<TSEVT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSEVT_SPEC>) -> Self {
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
#[doc = "ETM TimeStamp Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsevt](index.html) module"]
pub struct TSEVT_SPEC;
impl crate::RegisterSpec for TSEVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsevt::R](R) reader structure"]
impl crate::Readable for TSEVT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsevt::W](W) writer structure"]
impl crate::Writable for TSEVT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSEVT to value 0"]
impl crate::Resettable for TSEVT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
