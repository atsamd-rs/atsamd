#[doc = "Register `DEBOUNCEN` reader"]
pub struct R(crate::R<DEBOUNCEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBOUNCEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBOUNCEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBOUNCEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBOUNCEN` writer"]
pub struct W(crate::W<DEBOUNCEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBOUNCEN_SPEC>;
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
impl From<crate::W<DEBOUNCEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBOUNCEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBOUNCEN` reader - Debouncer Enable"]
pub struct DEBOUNCEN_R(crate::FieldReader<u16, u16>);
impl DEBOUNCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DEBOUNCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBOUNCEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBOUNCEN` writer - Debouncer Enable"]
pub struct DEBOUNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBOUNCEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Debouncer Enable"]
    #[inline(always)]
    pub fn debouncen(&self) -> DEBOUNCEN_R {
        DEBOUNCEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Debouncer Enable"]
    #[inline(always)]
    pub fn debouncen(&mut self) -> DEBOUNCEN_W {
        DEBOUNCEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debouncer Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debouncen](index.html) module"]
pub struct DEBOUNCEN_SPEC;
impl crate::RegisterSpec for DEBOUNCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debouncen::R](R) reader structure"]
impl crate::Readable for DEBOUNCEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debouncen::W](W) writer structure"]
impl crate::Writable for DEBOUNCEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBOUNCEN to value 0"]
impl crate::Resettable for DEBOUNCEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
