#[doc = "Register `RJFML` reader"]
pub struct R(crate::R<RJFML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RJFML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RJFML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RJFML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RJFML` writer"]
pub struct W(crate::W<RJFML_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RJFML_SPEC>;
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
impl From<crate::W<RJFML_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RJFML_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FML` reader - Frame Max Length"]
pub struct FML_R(crate::FieldReader<u16, u16>);
impl FML_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FML_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FML` writer - Frame Max Length"]
pub struct FML_W<'a> {
    w: &'a mut W,
}
impl<'a> FML_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Frame Max Length"]
    #[inline(always)]
    pub fn fml(&self) -> FML_R {
        FML_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Max Length"]
    #[inline(always)]
    pub fn fml(&mut self) -> FML_W {
        FML_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Jumbo Frame Max Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rjfml](index.html) module"]
pub struct RJFML_SPEC;
impl crate::RegisterSpec for RJFML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rjfml::R](R) reader structure"]
impl crate::Readable for RJFML_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rjfml::W](W) writer structure"]
impl crate::Writable for RJFML_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RJFML to value 0x3fff"]
impl crate::Resettable for RJFML_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff
    }
}
