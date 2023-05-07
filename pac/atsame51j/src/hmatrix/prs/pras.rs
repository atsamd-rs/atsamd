#[doc = "Register `PRAS` reader"]
pub struct R(crate::R<PRAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRAS` writer"]
pub struct W(crate::W<PRAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRAS_SPEC>;
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
impl From<crate::W<PRAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRAS_SPEC>) -> Self {
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
#[doc = "Priority A for Slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras](index.html) module"]
pub struct PRAS_SPEC;
impl crate::RegisterSpec for PRAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pras::R](R) reader structure"]
impl crate::Readable for PRAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pras::W](W) writer structure"]
impl crate::Writable for PRAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRAS to value 0"]
impl crate::Resettable for PRAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
