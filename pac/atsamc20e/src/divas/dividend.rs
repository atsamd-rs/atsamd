#[doc = "Register `DIVIDEND` reader"]
pub struct R(crate::R<DIVIDEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVIDEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVIDEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVIDEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVIDEND` writer"]
pub struct W(crate::W<DIVIDEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVIDEND_SPEC>;
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
impl From<crate::W<DIVIDEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVIDEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVIDEND` reader - DIVIDEND"]
pub struct DIVIDEND_R(crate::FieldReader<u32, u32>);
impl DIVIDEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIVIDEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVIDEND_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVIDEND` writer - DIVIDEND"]
pub struct DIVIDEND_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DIVIDEND"]
    #[inline(always)]
    pub fn dividend(&self) -> DIVIDEND_R {
        DIVIDEND_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DIVIDEND"]
    #[inline(always)]
    pub fn dividend(&mut self) -> DIVIDEND_W {
        DIVIDEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dividend\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dividend](index.html) module"]
pub struct DIVIDEND_SPEC;
impl crate::RegisterSpec for DIVIDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dividend::R](R) reader structure"]
impl crate::Readable for DIVIDEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dividend::W](W) writer structure"]
impl crate::Writable for DIVIDEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIVIDEND to value 0"]
impl crate::Resettable for DIVIDEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
