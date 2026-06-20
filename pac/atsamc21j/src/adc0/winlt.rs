#[doc = "Register `WINLT` reader"]
pub struct R(crate::R<WINLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINLT` writer"]
pub struct W(crate::W<WINLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINLT_SPEC>;
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
impl From<crate::W<WINLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINLT` reader - Window Lower Threshold"]
pub struct WINLT_R(crate::FieldReader<u16, u16>);
impl WINLT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WINLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINLT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINLT` writer - Window Lower Threshold"]
pub struct WINLT_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value as u16;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(self.bits as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&mut self) -> WINLT_W {
        WINLT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Monitor Lower Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winlt](index.html) module"]
pub struct WINLT_SPEC;
impl crate::RegisterSpec for WINLT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [winlt::R](R) reader structure"]
impl crate::Readable for WINLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winlt::W](W) writer structure"]
impl crate::Writable for WINLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WINLT to value 0"]
impl crate::Resettable for WINLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
