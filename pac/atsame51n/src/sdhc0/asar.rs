#[doc = "Register `ASAR[%s]` reader"]
pub struct R(crate::R<ASAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASAR[%s]` writer"]
pub struct W(crate::W<ASAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASAR_SPEC>;
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
impl From<crate::W<ASAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADMASA` reader - ADMA System Address"]
pub struct ADMASA_R(crate::FieldReader<u32, u32>);
impl ADMASA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADMASA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADMASA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADMASA` writer - ADMA System Address"]
pub struct ADMASA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMASA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn admasa(&self) -> ADMASA_R {
        ADMASA_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn admasa(&mut self) -> ADMASA_W {
        ADMASA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADMA System Address n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asar](index.html) module"]
pub struct ASAR_SPEC;
impl crate::RegisterSpec for ASAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asar::R](R) reader structure"]
impl crate::Readable for ASAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asar::W](W) writer structure"]
impl crate::Writable for ASAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASAR[%s]
to value 0"]
impl crate::Resettable for ASAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
