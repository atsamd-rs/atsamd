#[doc = "Register `AFSR` reader"]
pub struct R(crate::R<AFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFSR` writer"]
pub struct W(crate::W<AFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFSR_SPEC>;
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
impl From<crate::W<AFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMPDEF` reader - AUXFAULT input signals"]
pub struct IMPDEF_R(crate::FieldReader<u32, u32>);
impl IMPDEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IMPDEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMPDEF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMPDEF` writer - AUXFAULT input signals"]
pub struct IMPDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IMPDEF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AUXFAULT input signals"]
    #[inline(always)]
    pub fn impdef(&self) -> IMPDEF_R {
        IMPDEF_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AUXFAULT input signals"]
    #[inline(always)]
    pub fn impdef(&mut self) -> IMPDEF_W {
        IMPDEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afsr](index.html) module"]
pub struct AFSR_SPEC;
impl crate::RegisterSpec for AFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afsr::R](R) reader structure"]
impl crate::Readable for AFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afsr::W](W) writer structure"]
impl crate::Writable for AFSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFSR to value 0"]
impl crate::Resettable for AFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
