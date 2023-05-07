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
pub type FML_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FML` writer - Frame Max Length"]
pub type FML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RJFML_SPEC, u16, u16, 14, O>;
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
    #[must_use]
    pub fn fml(&mut self) -> FML_W<0> {
        FML_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RJFML to value 0x3fff"]
impl crate::Resettable for RJFML_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
