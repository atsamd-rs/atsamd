#[doc = "Register `WINUT` reader"]
pub struct R(crate::R<WINUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINUT` writer"]
pub struct W(crate::W<WINUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINUT_SPEC>;
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
impl From<crate::W<WINUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINUT` reader - Window Upper Threshold"]
pub struct WINUT_R(crate::FieldReader<u16, u16>);
impl WINUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WINUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINUT` writer - Window Upper Threshold"]
pub struct WINUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WINUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value as u16;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Window Upper Threshold"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(self.bits as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Upper Threshold"]
    #[inline(always)]
    pub fn winut(&mut self) -> WINUT_W {
        WINUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Monitor Upper Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winut](index.html) module"]
pub struct WINUT_SPEC;
impl crate::RegisterSpec for WINUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [winut::R](R) reader structure"]
impl crate::Readable for WINUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winut::W](W) writer structure"]
impl crate::Writable for WINUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WINUT to value 0"]
impl crate::Resettable for WINUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
