#[doc = "Register `PER` reader"]
pub struct R(crate::R<PER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PER` writer"]
pub struct W(crate::W<PER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_SPEC>;
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
impl From<crate::W<PER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER` reader - Period Value"]
pub struct PER_R(crate::FieldReader<u8, u8>);
impl PER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER` writer - Period Value"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Period Value"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COUNT8 Period Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per](index.html) module"]
pub struct PER_SPEC;
impl crate::RegisterSpec for PER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [per::R](R) reader structure"]
impl crate::Readable for PER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per::W](W) writer structure"]
impl crate::Writable for PER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PER to value 0xff"]
impl crate::Resettable for PER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
