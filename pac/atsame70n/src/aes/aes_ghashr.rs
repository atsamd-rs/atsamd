#[doc = "Register `AES_GHASHR[%s]` reader"]
pub struct R(crate::R<AES_GHASHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_GHASHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_GHASHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_GHASHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_GHASHR[%s]` writer"]
pub struct W(crate::W<AES_GHASHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_GHASHR_SPEC>;
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
impl From<crate::W<AES_GHASHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_GHASHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GHASH` reader - Intermediate GCM Hash Word x"]
pub struct GHASH_R(crate::FieldReader<u32, u32>);
impl GHASH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GHASH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GHASH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GHASH` writer - Intermediate GCM Hash Word x"]
pub struct GHASH_W<'a> {
    w: &'a mut W,
}
impl<'a> GHASH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    pub fn ghash(&self) -> GHASH_R {
        GHASH_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    pub fn ghash(&mut self) -> GHASH_W {
        GHASH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GCM Intermediate Hash Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ghashr](index.html) module"]
pub struct AES_GHASHR_SPEC;
impl crate::RegisterSpec for AES_GHASHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_ghashr::R](R) reader structure"]
impl crate::Readable for AES_GHASHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_ghashr::W](W) writer structure"]
impl crate::Writable for AES_GHASHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AES_GHASHR[%s]
to value 0"]
impl crate::Resettable for AES_GHASHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
