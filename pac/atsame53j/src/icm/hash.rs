#[doc = "Register `HASH` reader"]
pub struct R(crate::R<HASH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH` writer"]
pub struct W(crate::W<HASH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_SPEC>;
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
impl From<crate::W<HASH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASA` reader - Hash Area Start Address"]
pub struct HASA_R(crate::FieldReader<u32, u32>);
impl HASA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HASA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASA` writer - Hash Area Start Address"]
pub struct HASA_W<'a> {
    w: &'a mut W,
}
impl<'a> HASA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | ((value as u32 & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - Hash Area Start Address"]
    #[inline(always)]
    pub fn hasa(&self) -> HASA_R {
        HASA_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31 - Hash Area Start Address"]
    #[inline(always)]
    pub fn hasa(&mut self) -> HASA_W {
        HASA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region Hash Area Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash](index.html) module"]
pub struct HASH_SPEC;
impl crate::RegisterSpec for HASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash::R](R) reader structure"]
impl crate::Readable for HASH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash::W](W) writer structure"]
impl crate::Writable for HASH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH to value 0"]
impl crate::Resettable for HASH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
