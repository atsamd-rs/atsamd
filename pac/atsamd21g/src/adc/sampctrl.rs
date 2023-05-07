#[doc = "Register `SAMPCTRL` reader"]
pub struct R(crate::R<SAMPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPCTRL` writer"]
pub struct W(crate::W<SAMPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPCTRL_SPEC>;
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
impl From<crate::W<SAMPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLEN` reader - Sampling Time Length"]
pub type SAMPLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPLEN` writer - Sampling Time Length"]
pub type SAMPLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SAMPCTRL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Sampling Time Length"]
    #[inline(always)]
    pub fn samplen(&self) -> SAMPLEN_R {
        SAMPLEN_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Sampling Time Length"]
    #[inline(always)]
    #[must_use]
    pub fn samplen(&mut self) -> SAMPLEN_W<0> {
        SAMPLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling Time Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampctrl](index.html) module"]
pub struct SAMPCTRL_SPEC;
impl crate::RegisterSpec for SAMPCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sampctrl::R](R) reader structure"]
impl crate::Readable for SAMPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sampctrl::W](W) writer structure"]
impl crate::Writable for SAMPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPCTRL to value 0"]
impl crate::Resettable for SAMPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
