#[doc = "Register `TPQ` reader"]
pub struct R(crate::R<TPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPQ` writer"]
pub struct W(crate::W<TPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPQ_SPEC>;
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
impl From<crate::W<TPQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPQ` reader - Transmit Pause Quantum"]
pub struct TPQ_R(crate::FieldReader<u16, u16>);
impl TPQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPQ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPQ` writer - Transmit Pause Quantum"]
pub struct TPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Pause Quantum"]
    #[inline(always)]
    pub fn tpq(&self) -> TPQ_R {
        TPQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Pause Quantum"]
    #[inline(always)]
    pub fn tpq(&mut self) -> TPQ_W {
        TPQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpq](index.html) module"]
pub struct TPQ_SPEC;
impl crate::RegisterSpec for TPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpq::R](R) reader structure"]
impl crate::Readable for TPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpq::W](W) writer structure"]
impl crate::Writable for TPQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPQ to value 0xffff"]
impl crate::Resettable for TPQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
