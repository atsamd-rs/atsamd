#[doc = "Register `ACPR` reader"]
pub struct R(crate::R<ACPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACPR` writer"]
pub struct W(crate::W<ACPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACPR_SPEC>;
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
impl From<crate::W<ACPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - "]
pub struct PRESCALER_R(crate::FieldReader<u16, u16>);
impl PRESCALER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PRESCALER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCALER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCALER` writer - "]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acpr](index.html) module"]
pub struct ACPR_SPEC;
impl crate::RegisterSpec for ACPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acpr::R](R) reader structure"]
impl crate::Readable for ACPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acpr::W](W) writer structure"]
impl crate::Writable for ACPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACPR to value 0"]
impl crate::Resettable for ACPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
