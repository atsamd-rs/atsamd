#[doc = "Register `DIVISOR` reader"]
pub struct R(crate::R<DIVISOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVISOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVISOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVISOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVISOR` writer"]
pub struct W(crate::W<DIVISOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVISOR_SPEC>;
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
impl From<crate::W<DIVISOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVISOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVISOR` reader - DIVISOR"]
pub struct DIVISOR_R(crate::FieldReader<u32, u32>);
impl DIVISOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIVISOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVISOR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVISOR` writer - DIVISOR"]
pub struct DIVISOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVISOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DIVISOR"]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DIVISOR"]
    #[inline(always)]
    pub fn divisor(&mut self) -> DIVISOR_W {
        DIVISOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divisor](index.html) module"]
pub struct DIVISOR_SPEC;
impl crate::RegisterSpec for DIVISOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divisor::R](R) reader structure"]
impl crate::Readable for DIVISOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divisor::W](W) writer structure"]
impl crate::Writable for DIVISOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIVISOR to value 0"]
impl crate::Resettable for DIVISOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
