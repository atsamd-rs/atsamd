#[doc = "Register `DATABUFPTR` reader"]
pub struct R(crate::R<DATABUFPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATABUFPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATABUFPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATABUFPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATABUFPTR` writer"]
pub struct W(crate::W<DATABUFPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATABUFPTR_SPEC>;
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
impl From<crate::W<DATABUFPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATABUFPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDATAPTR` reader - Input Data Pointer"]
pub struct INDATAPTR_R(crate::FieldReader<u8, u8>);
impl INDATAPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INDATAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDATAPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDATAPTR` writer - Input Data Pointer"]
pub struct INDATAPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INDATAPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Data Pointer"]
    #[inline(always)]
    pub fn indataptr(&self) -> INDATAPTR_R {
        INDATAPTR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Data Pointer"]
    #[inline(always)]
    pub fn indataptr(&mut self) -> INDATAPTR_W {
        INDATAPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data buffer pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databufptr](index.html) module"]
pub struct DATABUFPTR_SPEC;
impl crate::RegisterSpec for DATABUFPTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [databufptr::R](R) reader structure"]
impl crate::Readable for DATABUFPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [databufptr::W](W) writer structure"]
impl crate::Writable for DATABUFPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATABUFPTR to value 0"]
impl crate::Resettable for DATABUFPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
