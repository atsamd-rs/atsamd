#[doc = "Register `CRCDATAIN` reader"]
pub struct R(crate::R<CRCDATAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDATAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDATAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDATAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDATAIN` writer"]
pub struct W(crate::W<CRCDATAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDATAIN_SPEC>;
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
impl From<crate::W<CRCDATAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDATAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCDATAIN` reader - CRC Data Input"]
pub struct CRCDATAIN_R(crate::FieldReader<u32, u32>);
impl CRCDATAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CRCDATAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCDATAIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCDATAIN` writer - CRC Data Input"]
pub struct CRCDATAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCDATAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    pub fn crcdatain(&self) -> CRCDATAIN_R {
        CRCDATAIN_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    pub fn crcdatain(&mut self) -> CRCDATAIN_W {
        CRCDATAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdatain](index.html) module"]
pub struct CRCDATAIN_SPEC;
impl crate::RegisterSpec for CRCDATAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcdatain::R](R) reader structure"]
impl crate::Readable for CRCDATAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdatain::W](W) writer structure"]
impl crate::Writable for CRCDATAIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCDATAIN to value 0"]
impl crate::Resettable for CRCDATAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
