#[doc = "Register `SEQCTRL` reader"]
pub struct R(crate::R<SEQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCTRL` writer"]
pub struct W(crate::W<SEQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCTRL_SPEC>;
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
impl From<crate::W<SEQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQEN` reader - Enable Positive Input in the Sequence"]
pub type SEQEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEQEN` writer - Enable Positive Input in the Sequence"]
pub type SEQEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCTRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Enable Positive Input in the Sequence"]
    #[inline(always)]
    pub fn seqen(&self) -> SEQEN_R {
        SEQEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable Positive Input in the Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn seqen(&mut self) -> SEQEN_W<0> {
        SEQEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequence Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqctrl](index.html) module"]
pub struct SEQCTRL_SPEC;
impl crate::RegisterSpec for SEQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqctrl::R](R) reader structure"]
impl crate::Readable for SEQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqctrl::W](W) writer structure"]
impl crate::Writable for SEQCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQCTRL to value 0"]
impl crate::Resettable for SEQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
