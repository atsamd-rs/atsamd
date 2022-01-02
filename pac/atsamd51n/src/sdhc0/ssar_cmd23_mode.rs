#[doc = "Register `SSAR_CMD23_MODE` reader"]
pub struct R(crate::R<SSAR_CMD23_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSAR_CMD23_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSAR_CMD23_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSAR_CMD23_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSAR_CMD23_MODE` writer"]
pub struct W(crate::W<SSAR_CMD23_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSAR_CMD23_MODE_SPEC>;
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
impl From<crate::W<SSAR_CMD23_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSAR_CMD23_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARG2` reader - Argument 2"]
pub struct ARG2_R(crate::FieldReader<u32, u32>);
impl ARG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ARG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARG2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARG2` writer - Argument 2"]
pub struct ARG2_W<'a> {
    w: &'a mut W,
}
impl<'a> ARG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Argument 2"]
    #[inline(always)]
    pub fn arg2(&self) -> ARG2_R {
        ARG2_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Argument 2"]
    #[inline(always)]
    pub fn arg2(&mut self) -> ARG2_W {
        ARG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMA System Address / Argument 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssar_cmd23_mode](index.html) module"]
pub struct SSAR_CMD23_MODE_SPEC;
impl crate::RegisterSpec for SSAR_CMD23_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssar_cmd23_mode::R](R) reader structure"]
impl crate::Readable for SSAR_CMD23_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssar_cmd23_mode::W](W) writer structure"]
impl crate::Writable for SSAR_CMD23_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSAR_CMD23_MODE to value 0"]
impl crate::Resettable for SSAR_CMD23_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
