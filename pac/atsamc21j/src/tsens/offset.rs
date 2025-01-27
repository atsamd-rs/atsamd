#[doc = "Register `OFFSET` reader"]
pub struct R(crate::R<OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFFSET` writer"]
pub struct W(crate::W<OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFFSET_SPEC>;
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
impl From<crate::W<OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSETC` reader - Offset Correction"]
pub struct OFFSETC_R(crate::FieldReader<u32, u32>);
impl OFFSETC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OFFSETC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSETC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSETC` writer - Offset Correction"]
pub struct OFFSETC_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Offset Correction"]
    #[inline(always)]
    pub fn offsetc(&self) -> OFFSETC_R {
        OFFSETC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Offset Correction"]
    #[inline(always)]
    pub fn offsetc(&mut self) -> OFFSETC_W {
        OFFSETC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offset](index.html) module"]
pub struct OFFSET_SPEC;
impl crate::RegisterSpec for OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [offset::R](R) reader structure"]
impl crate::Readable for OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [offset::W](W) writer structure"]
impl crate::Writable for OFFSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFFSET to value 0"]
impl crate::Resettable for OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
