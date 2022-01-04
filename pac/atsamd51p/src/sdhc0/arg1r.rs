#[doc = "Register `ARG1R` reader"]
pub struct R(crate::R<ARG1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARG1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARG1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARG1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARG1R` writer"]
pub struct W(crate::W<ARG1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARG1R_SPEC>;
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
impl From<crate::W<ARG1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARG1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARG` reader - Argument 1"]
pub struct ARG_R(crate::FieldReader<u32, u32>);
impl ARG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ARG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARG` writer - Argument 1"]
pub struct ARG_W<'a> {
    w: &'a mut W,
}
impl<'a> ARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Argument 1"]
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Argument 1"]
    #[inline(always)]
    pub fn arg(&mut self) -> ARG_W {
        ARG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Argument 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arg1r](index.html) module"]
pub struct ARG1R_SPEC;
impl crate::RegisterSpec for ARG1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arg1r::R](R) reader structure"]
impl crate::Readable for ARG1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arg1r::W](W) writer structure"]
impl crate::Writable for ARG1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARG1R to value 0"]
impl crate::Resettable for ARG1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
