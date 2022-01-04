#[doc = "Register `FILTERBUF` reader"]
pub struct R(crate::R<FILTERBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTERBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTERBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTERBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTERBUF` writer"]
pub struct W(crate::W<FILTERBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTERBUF_SPEC>;
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
impl From<crate::W<FILTERBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTERBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTERBUF` reader - Filter Buffer Value"]
pub struct FILTERBUF_R(crate::FieldReader<u8, u8>);
impl FILTERBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTERBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTERBUF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTERBUF` writer - Filter Buffer Value"]
pub struct FILTERBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Filter Buffer Value"]
    #[inline(always)]
    pub fn filterbuf(&self) -> FILTERBUF_R {
        FILTERBUF_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Buffer Value"]
    #[inline(always)]
    pub fn filterbuf(&mut self) -> FILTERBUF_W {
        FILTERBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Buffer Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filterbuf](index.html) module"]
pub struct FILTERBUF_SPEC;
impl crate::RegisterSpec for FILTERBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [filterbuf::R](R) reader structure"]
impl crate::Readable for FILTERBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filterbuf::W](W) writer structure"]
impl crate::Writable for FILTERBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTERBUF to value 0"]
impl crate::Resettable for FILTERBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
