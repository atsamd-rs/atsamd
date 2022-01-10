#[doc = "Register `TI` reader"]
pub struct R(crate::R<TI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TI` writer"]
pub struct W(crate::W<TI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TI_SPEC>;
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
impl From<crate::W<TI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNS` reader - Count Nanoseconds"]
pub struct CNS_R(crate::FieldReader<u8, u8>);
impl CNS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNS` writer - Count Nanoseconds"]
pub struct CNS_W<'a> {
    w: &'a mut W,
}
impl<'a> CNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ACNS` reader - Alternative Count Nanoseconds"]
pub struct ACNS_R(crate::FieldReader<u8, u8>);
impl ACNS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACNS` writer - Alternative Count Nanoseconds"]
pub struct ACNS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `NIT` reader - Number of Increments"]
pub struct NIT_R(crate::FieldReader<u8, u8>);
impl NIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIT` writer - Number of Increments"]
pub struct NIT_W<'a> {
    w: &'a mut W,
}
impl<'a> NIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    pub fn cns(&self) -> CNS_R {
        CNS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    pub fn acns(&self) -> ACNS_R {
        ACNS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    pub fn nit(&self) -> NIT_R {
        NIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    pub fn cns(&mut self) -> CNS_W {
        CNS_W { w: self }
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    pub fn acns(&mut self) -> ACNS_W {
        ACNS_W { w: self }
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    pub fn nit(&mut self) -> NIT_W {
        NIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ti](index.html) module"]
pub struct TI_SPEC;
impl crate::RegisterSpec for TI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ti::R](R) reader structure"]
impl crate::Readable for TI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ti::W](W) writer structure"]
impl crate::Writable for TI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TI to value 0"]
impl crate::Resettable for TI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
